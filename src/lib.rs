#![doc=include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "mpu-ca7")] {
        pub mod mpu_ca7;
        pub use mpu_ca7::*;
    } else if #[cfg(feature = "mcu-cm4")] {
        pub mod mcu_cm4;
        pub use mcu_cm4::*;
    }
}

pub mod bitworker;
pub mod dma;
pub mod dmamux;
pub mod gpio;
pub mod i2c;
pub mod rcc;
pub mod rng;
pub mod sai;
pub mod sdmmc;
pub mod spi;
pub mod stgen;
pub mod time;
pub mod usart;

pub use stm32mp1::stm32mp157 as pac;
