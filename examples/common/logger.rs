//! Logger using the console USART4.

use core::fmt::Write;

use static_cell::StaticCell;

use crate::hal::usart::{Usart4, UsartConfig};

use super::console::Console;

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
