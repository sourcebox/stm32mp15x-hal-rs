//! MMU configuration.
//!
//! The MMU maps the virtual addresses to physical addresses using translation tables.
//! In the setup used here, no address translations are performed.
//! The MMU is only used to set memory attributes for different regions to enable caching.
//!
//! Only first level address translation (L1) via section entries is used.
//! Each entry in the table refers to a 1MB section. Accordingly, the full 4GB address space
//! consists of 4096 sections.

#![allow(asm_sub_register)]

use core::arch::asm;

use bitflags::bitflags;

use super::cache::{invalidate_dcache_all, invalidate_icache_all};
use super::MemoryRegion;

/// Number of entries in the translation table.
pub const TRANSLATION_TABLE_LENGTH: usize = 4096;

/// Translation table type alias.
pub type TranslationTable = [u32; TRANSLATION_TABLE_LENGTH];

/// L1 table entry id for a section of 1MB.
const L1_ENTRY_SECTION: u32 = 0b10;

bitflags! {
    /// Section attributes for the translation table.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct SectionAttr: u32 {
        const Bufferable = 0b1 << 2;
        const Cacheable = 0b1 << 3;
        const ExecuteNever = 0b1 << 4;
        const ApPermFault = 0b00 << 10;
        const ApPrivOnly = 0b01 << 10;
        const ApNoUserWrite = 0b10 << 10;
        const ApFullAccess = 0b11 << 10;
        const ApxReadWrite  = 0b0 << 15;
        const ApxReadOnly = 0b1 << 15;
        const Shareable = 0b1 << 16;
    }
}

/// Initializes a translation table.
///
/// The `map_fn` takes an address and returns the appropriate `MemoryRegion` for it.
pub fn init_translation_table<F>(table: &mut TranslationTable, map_fn: F)
where
    F: Fn(u32) -> MemoryRegion,
{
    for (i, table_entry) in table.iter_mut().enumerate().take(TRANSLATION_TABLE_LENGTH) {
        let addr = (i as u32) << 20;
        let region = map_fn(addr);

        let attr = match region {
            // Code is normal memory with execute permissions.
            MemoryRegion::Code => (SectionAttr::Shareable
                | SectionAttr::Cacheable
                | SectionAttr::Bufferable
                | SectionAttr::ApFullAccess)
                .bits(),

            // Data is normal memory without execute permissions.
            MemoryRegion::Data => (SectionAttr::Shareable
                | SectionAttr::Cacheable
                | SectionAttr::Bufferable
                | SectionAttr::ApFullAccess
                | SectionAttr::ExecuteNever)
                .bits(),

            // Unbuffered data is normal memory without execute permissions.
            MemoryRegion::UnbufferedData => (SectionAttr::Shareable
                | SectionAttr::Cacheable
                | SectionAttr::ApFullAccess
                | SectionAttr::ExecuteNever)
                .bits(),

            // Default is device memory without execute permissions.
            MemoryRegion::Device => {
                (SectionAttr::Bufferable | SectionAttr::ApFullAccess | SectionAttr::ExecuteNever)
                    .bits()
            }
        };

        *table_entry = addr | attr | L1_ENTRY_SECTION;
    }
}

/// Enables the MMU with a translation table.
///
/// The translation table must be initialized before calling this function.
pub fn enable(table: &TranslationTable) {
    // Set SMP bit in ACTLR
    unsafe {
        let mut auxctrl: u32;
        asm! {
            "mrc p15, 0, {r}, c1, c0,  1",
            r = out(reg) auxctrl
        }
        auxctrl |= 1 << 6;
        asm! {
            "mcr p15, 0, {r}, c1, c0,  1",
            r = in(reg) auxctrl
        }
    }

    // Set domain 0 to client
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c3, c0, 0",
            r = in(reg) 1
        }
    }

    // Always use TTBR0 for the translation table.
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c2, c0, 2",
            r = in(reg) 0
        }
    }

    // Set TTBR0 to the translation table base address,
    // page table walk inner and outer non-cacheable, non-shareable memory.
    unsafe {
        asm! {
           "mcr p15, 0, {r}, c2, c0, 0",
           "isb",
           r = in(reg) table as *const u32 as u32
        }
    }

    // Set all caches invalid.
    invalidate_tlb();
    invalidate_dcache_all();
    invalidate_icache_all();

    // Enable MMU, caches and branch prediction in SCTLR.
    unsafe {
        let mut mode: u32;
        asm! {
           "mrc p15, 0, {r}, c1, c0, 0",
           r = out(reg) mode
        }

        mode |= 0x1805; // TODO: explain this magic number.

        asm! {
           "mcr p15, 0, {r}, c1, c0, 0",
           r = in(reg) mode
        }
    }
}

/// Invalidates TLB (translation lookaside buffer)
fn invalidate_tlb() {
    unsafe {
        asm! {
            "mcr p15, 0, {r}, c8, c3, 0",
            "dsb",
            "isb",
            r = in(reg) 0
        }
    }
}
