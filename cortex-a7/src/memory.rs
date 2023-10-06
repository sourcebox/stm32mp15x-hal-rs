//! Memory management.

pub mod cache;
pub mod mmu;

/// Memory regions.
#[derive(Debug)]
pub enum MemoryRegion {
    /// Executable code.
    Code,
    /// Application data, heap, stack, etc.
    Data,
    /// Device memory for peripherals.
    Device,
}
