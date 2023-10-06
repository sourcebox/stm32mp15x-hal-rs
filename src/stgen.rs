//! System timer generator.

use crate::pac;
use pac::stgenc::RegisterBlock;

/// STGEN peripheral.
#[derive(Debug, Default)]
pub struct Stgen;

impl Stgen {
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the current counter value.
    pub fn value(&self) -> u64 {
        let regs = self.registers();
        let mut upper = regs.stgenc_cntcvu.read().bits();
        let lower = regs.stgenc_cntcvl.read().bits();
        if upper < regs.stgenc_cntcvu.read().bits() {
            // An overflow occurred inbetween reads of upper and lower.
            upper += 1;
        }

        (upper as u64) << 32 | lower as u64
    }

    /// Sets the base frequency in number of ticks per second.
    ///
    /// This function can only be used when counter is stopped.
    pub fn set_base_frequency(&mut self, frequency: u32) {
        let regs = self.registers();
        unsafe {
            regs.stgenc_cntfid0.write(|w| w.bits(frequency));
        }
    }

    /// Enables incrementing the counter.
    pub fn start(&mut self) {
        let regs = self.registers();
        regs.stgenc_cntcr.modify(|_, w| w.en().set_bit());
    }

    /// Disables incrementing the counter.
    pub fn stop(&mut self) {
        let regs = self.registers();
        regs.stgenc_cntcr.modify(|_, w| w.en().clear_bit());
    }

    /// Returns the register block.
    pub fn registers(&self) -> &'static RegisterBlock {
        unsafe { &(*pac::STGENC::ptr()) }
    }
}
