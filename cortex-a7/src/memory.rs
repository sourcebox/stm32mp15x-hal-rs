//! Memory management.

pub mod cache;
pub mod mmu;

/// Memory regions.
#[derive(Debug)]
pub enum MemoryRegion {
    /// Executable code.
    Code,
    /// Application data, heap, stack, etc. using write-back cache.
    Data,
    /// Unbuffered data using write-through cache.
    UnbufferedData,
    /// Device memory for peripherals.
    Device,
}
