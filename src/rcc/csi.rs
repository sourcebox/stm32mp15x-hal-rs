//! CSI oscillator.

use crate::pac;

/// Frequency of the CSI oscillator in Hz.
const CSI_FREQUENCY: u32 = 4000000;

/// CSI peripheral.
#[derive(Debug, Default)]
pub struct Csi;

impl Csi {
    /// Returns a new instance of the peripheral.
    pub fn new() -> Self {
        Self {}
    }

    /// Enables the CSI oscillator.
    pub fn enable(&mut self) {
        unsafe {
            let rcc = &(*pac::RCC::ptr());
            rcc.ocensetr().modify(|_, w| w.csion().set_bit());
        }
    }

    /// Disables the CSI oscillator.
    pub fn disable(&mut self) {
        unsafe {
            let rcc = &(*pac::RCC::ptr());
            rcc.ocenclrr().modify(|_, w| w.csion().set_bit());
        }
    }

    /// Returns if the CSI oscillator is enabled.
    pub fn is_enabled(&self) -> bool {
        unsafe {
            let rcc = &(*pac::RCC::ptr());
            rcc.ocensetr().read().csion().bit_is_set()
        }
    }

    /// Returns if the CSI oscillator is ready.
    pub fn is_ready(&self) -> bool {
        unsafe {
            let rcc = &(*pac::RCC::ptr());
            rcc.ocrdyr().read().csirdy().bit_is_set()
        }
    }

    /// Returns the frequency of the CSI oscillator in Hz.
    pub fn frequency(&self) -> u32 {
        CSI_FREQUENCY
    }
}
