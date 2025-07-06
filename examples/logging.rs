//! Logging example.
//!
//! Logs via the boards USART4 peripheral connected to the ST-Link.
//! A serial monitor is required on the host to view the messages.
//! Connection settings are 115200 baud, 8 data bits, no parity.

#![no_std]
#![no_main]

use panic_halt as _;
use stm32mp15x_hal as hal;

use hal::{time, HalConfig, MemoryRegion};

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

    let mut count: u32 = 0;

    loop {
        log::debug!("Tick {count}");
        count = count.wrapping_add(1);
        time::delay_ms(1000);
    }
}

/// Entry point for MPU1.
#[no_mangle]
pub extern "C" fn mpu1_main() -> ! {
    loop {}
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

mod clocks {
    use super::hal::rcc;

    pub fn init() {
        rcc::set_apb4_div(rcc::ApbDiv::Div2);
        rcc::set_apb5_div(rcc::ApbDiv::Div2);

        init_pll3();
        init_pll4();

        rcc::set_mcu_clock_source(rcc::McuSource::Pll3);
    }

    /// Initialize PLL3 for MCU.
    fn init_pll3() {
        rcc::disable_pll3();
        rcc::set_pll3_source(rcc::Pll3Source::Hse);
        rcc::set_pll3_input_frequency_range(rcc::Pll3InputFreqRange::From8To16);
        rcc::set_pll3_prescaler(3);
        rcc::set_pll3_multiplier(52);
        rcc::set_pll3_p_divider(2);
        rcc::set_pll3_q_divider(2);
        rcc::set_pll3_r_divider(2);
        rcc::set_pll3_fractional(0);
        rcc::set_apb1_div(rcc::ApbDiv::Div2);
        rcc::set_apb2_div(rcc::ApbDiv::Div2);
        rcc::set_apb3_div(rcc::ApbDiv::Div2);
        rcc::enable_pll3();
    }

    /// Initialize PLL4 for SAI.
    fn init_pll4() {
        rcc::set_pll4_source(rcc::Pll4Source::Hse);
        rcc::set_pll4_input_frequency_range(rcc::Pll4InputFreqRange::From8To16);

        // 98.304000 MHz for 48kHz sampling rate.
        rcc::set_pll4_prescaler(3);
        rcc::set_pll4_multiplier(61);
        rcc::set_pll4_p_divider(5);
        rcc::set_pll4_q_divider(5);
        rcc::set_pll4_r_divider(2);
        rcc::set_pll4_fractional(3604);

        rcc::enable_pll4();
    }

    /// Print some info.
    pub fn print_info() {
        log::info!("PLL1:   {:>9} Hz", rcc::pll1_frequency());
        log::info!("PLL1 P: {:>9} Hz", rcc::pll1_p_frequency());
        log::info!("PLL1 Q: {:>9} Hz", rcc::pll1_q_frequency());
        log::info!("PLL1 R: {:>9} Hz", rcc::pll1_r_frequency());
        log::info!("PLL2:   {:>9} Hz", rcc::pll2_frequency());
        log::info!("PLL2 P: {:>9} Hz", rcc::pll2_p_frequency());
        log::info!("PLL2 Q: {:>9} Hz", rcc::pll2_q_frequency());
        log::info!("PLL2 R: {:>9} Hz", rcc::pll2_r_frequency());
        log::info!("PLL3:   {:>9} Hz", rcc::pll3_frequency());
        log::info!("PLL3 P: {:>9} Hz", rcc::pll3_p_frequency());
        log::info!("PLL3 Q: {:>9} Hz", rcc::pll3_q_frequency());
        log::info!("PLL3 R: {:>9} Hz", rcc::pll3_r_frequency());
        log::info!("PLL4:   {:>9} Hz", rcc::pll4_frequency());
        log::info!("PLL4 P: {:>9} Hz", rcc::pll4_p_frequency());
        log::info!("PLL4 Q: {:>9} Hz", rcc::pll4_q_frequency());
        log::info!("PLL4 R: {:>9} Hz", rcc::pll4_r_frequency());
        log::info!("MPU:    {:>9} Hz", rcc::mpu_frequency());
        log::info!("MCU:    {:>9} Hz", rcc::mcu_frequency());
        log::info!("ACLK:   {:>9} Hz", rcc::aclk_frequency());
        log::info!("PCLK1:  {:>9} Hz", rcc::pclk1_frequency());
        log::info!("PCLK2:  {:>9} Hz", rcc::pclk2_frequency());
        log::info!("PCLK3:  {:>9} Hz", rcc::pclk3_frequency());
        log::info!("PCLK4:  {:>9} Hz", rcc::pclk4_frequency());
        log::info!("PCLK5:  {:>9} Hz", rcc::pclk5_frequency());
        log::info!("PER_CK: {:>9} Hz", rcc::per_ck_frequency());
    }
}

mod logger {
    use core::fmt::Write;

    use static_cell::StaticCell;

    use super::hal::{
        self,
        usart::{Usart4, UsartConfig},
    };

    /// Logger instance.
    static LOGGER: StaticCell<Logger> = StaticCell::new();

    /// Logger with level filter.
    #[derive(Debug)]
    struct Logger {
        /// Level filter.
        level_filter: log::LevelFilter,
    }

    impl Default for Logger {
        fn default() -> Self {
            Self {
                level_filter: log::LevelFilter::Trace,
            }
        }
    }

    impl log::Log for Logger {
        /// Returns if logger is enabled.
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= self.level_filter
        }

        /// Log the record.
        fn log(&self, record: &log::Record) {
            if self.enabled(record.metadata()) {
                critical_section::with(|_| {
                    let mut writer = Console;
                    writeln!(
                        &mut writer,
                        "{:<5} [{}] {}\r",
                        record.level(),
                        record.target(),
                        record.args()
                    )
                    .ok();
                });
            }
        }

        /// Flush buffered records.
        fn flush(&self) {
            // Nothing to do here
        }
    }

    /// Initialize the logger with default level (TRACE).
    pub fn init() {
        let mut usart4 = Usart4::new();
        let usart_config = UsartConfig {
            transmitter_enable: true,
            ..Default::default()
        };
        usart4.init(usart_config);

        let logger = LOGGER.init(Logger::default());
        log::set_logger(logger).unwrap();
        log::set_max_level(logger.level_filter);
    }

    /// Console for messages using USART4.
    #[derive(Debug)]
    pub struct Console;

    impl Write for Console {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            unsafe {
                let usart4 = &(*hal::pac::USART4::ptr());
                for c in s.chars() {
                    usart4.tdr().write(|w| w.bits(c as u32));
                    while usart4.isr().read().txe().bit_is_clear() {}
                }
            }

            Ok(())
        }
    }
}
