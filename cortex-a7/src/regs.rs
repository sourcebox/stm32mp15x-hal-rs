//! Register access.

#![allow(asm_sub_register)]

use core::arch::asm;

/// Return CNTPCT timer count.
pub fn cntpct() -> u64 {
    let mut lsw: u32;
    let mut msw: u32;

    unsafe {
        asm! {
            "mrrc p15, 0, {rt}, {rt2}, c14",
            rt = out(reg) lsw,
            rt2 = out(reg) msw,
        }
    }

    (msw as u64) << 32 | (lsw as u64)
}

/// Set CSSELR register value.
pub fn set_csselr(value: u32) {
    unsafe {
        asm! {
            "mcr p15, 2, {r}, c0, c0, 0",
            r = in(reg) value
        }
    }
}

/// Return CCSIDR register value.
pub fn ccsidr() -> u32 {
    let mut result: u32;
    unsafe {
        asm! {
            "mrc p15, 1, {r}, c0, c0, 0",
            r = out(reg) result
        }
    }

    result
}

/// Set DCISW register value.
pub fn set_dcisw(value: u32) {
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c7, c6, 2",
            r = in(reg) value
        }
    }
}

/// Set DCCSW register value.
pub fn set_dccsw(value: u32) {
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c7, c10, 2",
            r = in(reg) value
        }
    }
}

/// Set DCCISW register value.
pub fn set_dccisw(value: u32) {
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c7, c14, 2",
            r = in(reg) value
        }
    }
}

/// Return CBAR register value.
pub fn cbar() -> u32 {
    let mut result: u32;
    unsafe {
        asm! {
            "mrc p15, 4, {r}, c15, c0, 0",
            r = out(reg) result
        }
    }

    result
}

/// Return MPIDR register value.
pub fn mpidr() -> u32 {
    let mut result: u32;
    unsafe {
        asm! {
            "mrc p15, 0, {r}, c0, c0, 5",
            r = out(reg) result
        }
    }

    result
}
