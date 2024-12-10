//! Cache control

#![allow(asm_sub_register)]

use core::arch::asm;

use crate::regs::*;

/// Invalidate all instruction caches.
pub fn invalidate_icache_all() {
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c7, c5, 0",
            "dsb",
            "isb",
            r = in(reg) 0
        }
    }
}

/// Invalidate all data caches.
pub fn invalidate_dcache_all() {
    let mut clidr: u32;
    unsafe {
        asm! {
            "mrc p15, 1, {r}, c0, c0, 1",
            r = out(reg) clidr
        }
    }

    for i in 0..7 {
        let cache_type = (clidr >> (i * 3)) & 0x07;
        if (2..=4).contains(&cache_type) {
            l1c_maintain_dcache_set_way(i, 0);
        }
    }
}

/// Clean data cache for an address range.
pub fn clean_dcache_by_range(start_addr: u32, end_addr: u32) {
    unsafe {
        asm! {
                "stmfd sp!, {{r0-r1}}",

                "mov   r0, {r0}",
                "mov   r1, {r1}",

                "bic   r0, r0, #7",
            "2:",
                "mcr   p15, 0, r0, c7, c10, 1",
                "add   r0, r0, #8",
                "cmp   r0, r1",
                "blo   1b",
                "dsb",

                "ldmfd sp!, {{r0-r1}}",
            r0 = in(reg) start_addr,
            r1 = in(reg) end_addr,
        }
    }
}

/// Apply cache maintenance to given cache level.
/// - `level:` cache level to be maintained.
/// - `maint:` 0 - invalidate, 1 - clean, otherwise - invalidate and clean.
fn l1c_maintain_dcache_set_way(level: u32, maint: u32) {
    let mut dummy = level << 1;

    // set csselr, select ccsidr register
    set_csselr(dummy);

    // get current ccsidr register
    let ccsidr = ccsidr();

    let num_sets = ((ccsidr & 0x0FFFE000) >> 13) + 1;
    let num_ways = ((ccsidr & 0x00001FF8) >> 3) + 1;
    let log2_linesize = (ccsidr & 0x00000007) + 2 + 2;
    let log2_num_ways = log2_up(num_ways);

    if log2_num_ways > 32 {
        return; // FATAL ERROR
    }

    let shift_way = 32 - log2_num_ways as u32;

    let mut way = (num_ways - 1) as i32;

    while way >= 0 {
        let mut set = (num_sets - 1) as i32;
        while set >= 0 {
            dummy = (level << 1) | ((set as u32) << log2_linesize) | ((way as u32) << shift_way);
            match maint {
                0 => set_dcisw(dummy),
                1 => set_dccsw(dummy),
                _ => set_dccisw(dummy),
            }
            set -= 1;
        }
        way -= 1;
    }

    unsafe {
        asm! { "dmb"};
    }
}

/// Calculate log2 rounded up.
///
/// - log(0)  => 0
/// - log(1)  => 0
/// - log(2)  => 1
/// - log(3)  => 2
/// - log(4)  => 2
/// - log(5)  => 3
///       :      :
/// - log(16) => 4
/// - log(32) => 5
///       :      :
fn log2_up(n: u32) -> u8 {
    if n < 2 {
        return 0;
    }

    let mut log = 0;
    let mut t = n;

    while t > 1 {
        log += 1;
        t >>= 1;
    }

    if n & 1 != 0 {
        log += 1;
    }

    log
}
