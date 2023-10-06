//! Independent watchdogs.
//!
//! The STM32MP15x has 2 independent watchdogs dedicated to the MPUs.
//! Once enabled, a watchdog can't be disabled anymore until reset.

use core::marker::PhantomData;
use core::ops::Deref;

use crate::pac;
use crate::pac::{IWDG1, IWDG2};
use pac::iwdg1::RegisterBlock;

/// IWDG peripheral.
#[derive(Debug, Default)]
pub struct Iwdg<R> {
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for IWDG1.
pub type Iwdg1 = Iwdg<IWDG1>;

/// Type alias for IWDG2.
pub type Iwdg2 = Iwdg<IWDG2>;

// ------------------------- Configuration ---------------------------

/// Prescaler divider.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Prescaler {
    /// Division by 4.
    Div4 = 0b0000,
    /// Division by 8.
    Div8 = 0b0001,
    /// Division by 16.
    Div16 = 0b0010,
    /// Division by 32.
    Div32 = 0b0011,
    /// Division by 64.
    Div64 = 0b0100,
    /// Division by 128.
    Div128 = 0b0101,
    /// Division by 256.
    Div256 = 0b0110,
    /// Division by 512.
    Div512 = 0b0111,
    /// Division by 1024.
    Div1024,
}

impl From<Prescaler> for u8 {
    fn from(value: Prescaler) -> Self {
        value as u8
    }
}

impl From<u32> for Prescaler {
    fn from(value: u32) -> Self {
        match value {
            0b0000 => Self::Div4,
            0b0001 => Self::Div8,
            0b0010 => Self::Div16,
            0b0011 => Self::Div32,
            0b0100 => Self::Div64,
            0b0101 => Self::Div128,
            0b0110 => Self::Div256,
            0b0111 => Self::Div512,
            _ => Self::Div1024,
        }
    }
}

// ------------------------- Implementation ---------------------------

impl<R> Iwdg<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Starts the watchdog.
    pub fn start(&mut self) {
        R::enable_clock();

        unsafe {
            let regs = R::registers();
            regs.iwdg_kr.write(|w| w.bits(0x0000CCCC));
        }
    }

    /// Triggers the watchdog. Must be called periodically when it is enabled.
    pub fn trigger(&mut self) {
        unsafe {
            let regs = R::registers();
            regs.iwdg_kr.write(|w| w.bits(0x0000AAAA));
        }
    }

    /// Returns the clock prescaler.
    pub fn prescaler(&self) -> Prescaler {
        R::enable_clock();

        let regs = R::registers();
        while regs.iwdg_sr.read().pvu().bit_is_set() {}
        regs.iwdg_pr.read().bits().into()
    }

    /// Sets the clock prescaler. Must be called after the watchdog is started.
    pub fn set_prescaler(&mut self, prescaler: Prescaler) {
        R::enable_clock();

        unsafe {
            let regs = R::registers();
            regs.iwdg_kr.write(|w| w.bits(0x00005555));
            while regs.iwdg_sr.read().pvu().bit_is_set() {}
            regs.iwdg_pr.write(|w| w.pr().bits(prescaler.into()));
            while regs.iwdg_sr.read().pvu().bit_is_set() {}
        }
    }

    /// Returns the reload value.
    pub fn reload_value(&self) -> u16 {
        R::enable_clock();

        let regs = R::registers();
        while regs.iwdg_sr.read().rvu().bit_is_set() {}
        regs.iwdg_rlr.read().bits() as u16
    }

    /// Sets the reload value. Must be called after the watchdog is started.
    pub fn set_reload_value(&mut self, reload: u16) {
        R::enable_clock();

        unsafe {
            let regs = R::registers();
            regs.iwdg_kr.write(|w| w.bits(0x00005555));
            while regs.iwdg_sr.read().rvu().bit_is_set() {}
            regs.iwdg_rlr.write(|w| w.rl().bits(reload));
            while regs.iwdg_sr.read().rvu().bit_is_set() {}
        }
    }

    /// Returns the register block.
    pub fn registers(&self) -> &'static RegisterBlock {
        R::registers()
    }
}

// ---------------------------- Instance ------------------------------

/// Trait for instance specific functions.
pub trait Instance {
    /// Returns the register block.
    fn registers() -> &'static RegisterBlock;

    /// Enables the clock.
    fn enable_clock();

    /// Disables the clock.
    fn disable_clock();
}

// ------------------------------ IWDG1 -------------------------------

impl Instance for IWDG1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::IWDG1::ptr()) }
    }

    fn enable_clock() {
        let rcc = unsafe { &(*pac::RCC::ptr()) };
        rcc.rcc_mp_apb5ensetr.write(|w| w.iwdg1apben().set_bit());
    }

    fn disable_clock() {
        let rcc = unsafe { &(*pac::RCC::ptr()) };
        rcc.rcc_mp_apb5enclrr.write(|w| w.iwdg1apben().set_bit());
    }
}

// ------------------------------ IWDG2 -------------------------------

impl Instance for IWDG2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::IWDG2::ptr()) }
    }

    fn enable_clock() {
        let rcc = unsafe { &(*pac::RCC::ptr()) };
        rcc.rcc_mp_apb4ensetr.write(|w| w.iwdg2apben().set_bit());
    }

    fn disable_clock() {
        let rcc = unsafe { &(*pac::RCC::ptr()) };
        rcc.rcc_mp_apb4enclrr.write(|w| w.iwdg2apben().set_bit());
    }
}
