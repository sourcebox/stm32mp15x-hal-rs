//! Panic handler using the console USART4.

use core::fmt::Write;
use core::panic::PanicInfo;
use core::sync::atomic::{compiler_fence, Ordering};

use super::console::Console;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Console;
    writeln!(&mut writer, "{}\r", info).ok();

    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
