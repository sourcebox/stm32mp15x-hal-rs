//! Console for log messages and panics.

use core::fmt::Write;

use crate::hal;

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
