//! Blinky example.
//!
//! Uses the two cores to blink an LED independently.

#![no_std]
#![no_main]

use panic_halt as _;
use stm32mp15x_hal as hal;

use hal::{
    gpio::{Pin, PinMode, PinState, Port},
    time, HalConfig, MemoryRegion,
};

/// Entry point for MPU0.
#[no_mangle]
pub extern "C" fn main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    hal::start_mpu1();

    let mut led = Pin::with_mode(Port::A, 14, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        time::delay_ms(500);
        led.set_output_state(PinState::Low);
        time::delay_ms(500);
    }
}

/// Entry point for MPU1.
#[no_mangle]
pub extern "C" fn mpu1_main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    let mut led = Pin::with_mode(Port::A, 13, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        time::delay_ms(333);
        led.set_output_state(PinState::Low);
        time::delay_ms(333);
    }
}

/// Returns the memory region for an address. To be used for MMU translation table.
fn memory_region_mapper(addr: u32) -> MemoryRegion {
    match addr {
        0xC2000000..=0xCFFFFFFF => MemoryRegion::Code,
        0xC0000000..=0xC1FFFFFF => MemoryRegion::Data,
        0xD0000000..=0xDFFFFFFF => MemoryRegion::Data,
        _ => MemoryRegion::Device,
    }
}

/// Dummy function called in startup code. Required by C/C++ to work correctly.
#[no_mangle]
pub extern "C" fn __libc_init_array() {}
