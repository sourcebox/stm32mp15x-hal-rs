//! Time functions and structs.

use core::cell::RefCell;
use core::future::poll_fn;
use core::task::Poll;

use critical_section::{CriticalSection, Mutex};

use crate::rcc::per_ck_frequency;
use crate::stgen::Stgen;

/// Return milliseconds since start.
pub fn millis() -> u64 {
    let stgen = Stgen::new();
    stgen.value() / (per_ck_frequency() as u64 / 1000)
}

/// Return microseconds since start.
pub fn micros() -> u64 {
    let stgen = Stgen::new();
    stgen.value() / (per_ck_frequency() as u64 / 1000000)
}

// ------------------------ Blocking delay ---------------------------

/// Delays for some milliseconds.
pub fn delay_ms(ms: u32) {
    let start = millis();
    while millis() < start + ms as u64 {}
}

/// Delays for some microseconds.
pub fn delay_us(us: u32) {
    let start = micros();
    while micros() < start + us as u64 {}
}

// ------------------------- Async delay -----------------------------

/// Delays asynchronuously for some milliseconds.
pub async fn delay_ms_async(ms: u32) {
    let start = millis();
    poll_fn(|cx| {
        if millis() < start + ms as u64 {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        Poll::Ready(())
    })
    .await;
}

/// Delays asynchronuously for some microseconds.
pub async fn delay_us_async(us: u32) {
    let start = micros();
    poll_fn(|cx| {
        if micros() < start + us as u64 {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        Poll::Ready(())
    })
    .await;
}

// ------------------ embedded-hal delay provider --------------------

/// Delay provider.
#[derive(Debug, Default)]
pub struct Delay;

impl Delay {
    /// Returns a new instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl embedded_hal::delay::DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        delay_us(ns / 1000);
    }
}

// ---------------------- embassy-time-driver ------------------------

struct TimeDriver {
    queue: Mutex<RefCell<embassy_time_queue_utils::Queue>>,
}

impl TimeDriver {
    fn set_alarm(&self, cs: &CriticalSection, at: u64) -> bool {
        todo!()
    }
}

impl embassy_time_driver::Driver for TimeDriver {
    fn now(&self) -> u64 {
        micros()
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(&cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(
    static TIME_DRIVER: TimeDriver = TimeDriver {
        queue: Mutex::new(RefCell::new(embassy_time_queue_utils::Queue::new()))
    }
);
