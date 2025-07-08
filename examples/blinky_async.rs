//! Async blinky example.
//!
//! Uses embassy-executor to blink LEDs in separate tasks.

#![no_std]
#![no_main]

mod common;

use embassy_executor::Executor;
use embassy_time::Timer;
use hal::{
    gpio::{Pin, PinMode, PinState, Port},
    HalConfig,
};
use static_cell::StaticCell;
use stm32mp15x_hal as hal;

use common::memory_region_mapper;

/// Entry point for MPU0.
#[no_mangle]
pub extern "C" fn main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    hal::start_mpu1();

    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());

    executor.run(|spawner| {
        spawner.must_spawn(led1_task());
        spawner.must_spawn(led2_task());
    });
}

#[embassy_executor::task]
async fn led1_task() {
    let mut led = Pin::with_mode(Port::A, 14, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        Timer::after_millis(500).await;
        led.set_output_state(PinState::Low);
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn led2_task() {
    let mut led = Pin::with_mode(Port::A, 13, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        Timer::after_millis(333).await;
        led.set_output_state(PinState::Low);
        Timer::after_millis(333).await;
    }
}

/// Entry point for MPU1.
#[no_mangle]
pub extern "C" fn mpu1_main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    let executor = EXECUTOR.init(Executor::new());

    executor.run(|spawner| {
        spawner.must_spawn(led3_task());
        spawner.must_spawn(led4_task());
    });
}

#[embassy_executor::task]
async fn led3_task() {
    let mut led = Pin::with_mode(Port::H, 7, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        Timer::after_millis(750).await;
        led.set_output_state(PinState::Low);
        Timer::after_millis(750).await;
    }
}

#[embassy_executor::task]
async fn led4_task() {
    let mut led = Pin::with_mode(Port::D, 11, PinMode::Output);

    loop {
        led.set_output_state(PinState::High);
        Timer::after_millis(200).await;
        led.set_output_state(PinState::Low);
        Timer::after_millis(200).await;
    }
}
