//! Modules dedicated to the Cortex-M4 core.

mod critical_section_impl;

use core::arch::global_asm;

// Startup code for both Cortex-A cores.
global_asm!(include_str!("mcu_cm4/startup.s"));

/// ID for the MCU.
pub const CPU_ID: u8 = 2;

/// Initializes the HAL.
///
/// This function must be called once at the beginning of the main function.
pub fn init() {
    crate::gpio::init();
    crate::dma::init();
}
