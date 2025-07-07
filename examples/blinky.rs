//! Blinky example.
//!
//! Uses the two cores to blink an LED independently.

#![no_std]
#![no_main]

mod common;

use stm32mp15x_hal as hal;

use hal::{
    gpio::{Pin, PinMode, PinState, Port},
    time, HalConfig,
};

use common::memory_region_mapper;

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
