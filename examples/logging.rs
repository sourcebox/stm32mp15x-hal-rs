//! Logging example.
//!
//! Logs via the boards USART4 peripheral connected to the ST-Link.
//! A serial monitor is required on the host to view the messages.
//! Connection settings are 115200 baud, 8 data bits, no parity.

#![no_std]
#![no_main]

mod common;

use stm32mp15x_hal as hal;

use hal::{time, HalConfig};

use common::{clocks, logger, memory_region_mapper};

/// Entry point for MPU0.
#[no_mangle]
pub extern "C" fn main() -> ! {
    let hal_config = HalConfig {
        memory_region_mapper,
    };
    hal::init(hal_config);

    // Clocks must be setup before the USART, otherwise the baudrate will be incorrect.
    clocks::init();

    logger::init();
    clocks::print_info();

    for count in 0..5 {
        log::debug!("Tick {count}");
        time::delay_ms(1000);
    }

    panic!("Panicking on purpose.");
}

/// Entry point for MPU1.
#[no_mangle]
pub extern "C" fn mpu1_main() -> ! {
    loop {}
}
