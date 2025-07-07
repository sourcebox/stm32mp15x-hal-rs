//! Modules shared by the examples.

#![allow(dead_code)]

pub mod clocks;
pub mod console;
pub mod logger;
pub mod panic;

use crate::hal::MemoryRegion;

/// Returns the memory region for an address. To be used for MMU translation table.
pub fn memory_region_mapper(addr: u32) -> MemoryRegion {
    match addr {
        0xC2000000..=0xCFFFFFFF => MemoryRegion::Code,
        0xC0000000..=0xC1FFFFFF => MemoryRegion::Data,
        0xD0000000..=0xDFFFFFFF => MemoryRegion::Data,
        _ => MemoryRegion::Device,
    }
}
