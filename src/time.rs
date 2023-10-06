//! Time functions and structs.

use core::future::poll_fn;
use core::task::Poll;

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

// ---------------------------- Instant ------------------------------

/// Instant type representing a moment in time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instant {
    /// Microseconds value.
    micros: u64,
}

impl Instant {
    /// Returns an instant with the current time.
    pub fn now() -> Self {
        Self { micros: micros() }
    }

    /// Creates an instant from a microseconds value.
    pub fn from_micros(micros: u64) -> Self {
        Self { micros }
    }

    /// Creates an instant from a milliseconds value.
    pub fn from_millis(millis: u64) -> Self {
        Self {
            micros: millis * 1000,
        }
    }

    /// Creates an instant from a seconds value.
    pub fn from_secs(secs: u32) -> Self {
        Self {
            micros: secs as u64 * 1000000,
        }
    }

    /// Returns the microseconds value.
    pub fn to_micros(&self) -> u64 {
        self.micros
    }

    /// Returns the milliseconds value.
    pub fn to_millis(&self) -> u64 {
        self.micros / 1000
    }

    /// Returns the seconds value.
    pub fn to_secs(&self) -> u32 {
        (self.micros / 1000000) as u32
    }

    /// Returns the elapsed microseconds since this instant.
    pub fn elapsed_micros(&self) -> u64 {
        micros() - self.micros
    }

    /// Returns the elapsed milliseconds since this instant.
    pub fn elapsed_millis(&self) -> u64 {
        (micros() - self.micros) / 1000
    }

    /// Returns the elapsed seconds since this instant.
    pub fn elapsed_secs(&self) -> u32 {
        ((micros() - self.micros) / 1000000) as u32
    }

    /// Returns if a number of microseconds have elapsed since this instant.
    pub fn is_elapsed_micros(&self, micros: u64) -> bool {
        self.elapsed_micros() >= micros
    }

    /// Returns if a number of milliseconds have elapsed since this instant.
    pub fn is_elapsed_millis(&self, millis: u64) -> bool {
        self.elapsed_millis() >= millis
    }

    /// Returns if a number of seconds have elapsed since this instant.
    pub fn is_elapsed_secs(&self, secs: u32) -> bool {
        self.elapsed_secs() >= secs
    }
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

impl embedded_hal::delay::DelayUs for Delay {
    fn delay_us(&mut self, us: u32) {
        delay_us(us);
    }
}
