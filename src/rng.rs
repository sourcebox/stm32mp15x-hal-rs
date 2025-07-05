//! True random number generator.

use core::marker::PhantomData;
use core::ops::Deref;

use cfg_if::cfg_if;

use crate::pac;
use crate::rcc;
use pac::rng1::RegisterBlock;
use pac::{RNG1, RNG2};

/// RNG peripheral.
#[derive(Debug, Default)]
pub struct Rng<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for RNG1.
pub type Rng1 = Rng<RNG1>;

/// Type alias for RNG2.
pub type Rng2 = Rng<RNG2>;

// ------------------------- Implementation ---------------------------

impl<R> Rng<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Initializes the peripheral.
    pub fn init(&mut self) {
        let mut csi = rcc::csi::Csi::new();
        csi.enable();
        R::enable_clock();
        let regs = R::registers();
        regs.cr().modify(|_, w| w.ced().set_bit());
        self.enable();
    }

    /// Deinitializes the peripheral.
    pub fn deinit(&mut self) {
        self.disable();
        R::disable_clock();
    }

    /// Returns a generated value.
    pub fn value(&self) -> u32 {
        while !self.is_value_ready() {}
        let regs = R::registers();
        regs.dr().read().bits()
    }

    /// Enables the peripheral.
    pub fn enable(&mut self) {
        let regs = R::registers();
        regs.cr().modify(|_, w| w.rngen().set_bit());
    }

    /// Disables the peripheral.
    pub fn disable(&mut self) {
        let regs = R::registers();
        regs.cr().modify(|_, w| w.rngen().clear_bit());
    }

    /// Returns if the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        let regs = R::registers();
        regs.cr().read().rngen().bit_is_set()
    }

    /// Returns if the value is ready.
    pub fn is_value_ready(&self) -> bool {
        let regs = R::registers();
        regs.sr().read().drdy().bit_is_set()
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

    /// Returns the clock frequency in Hz.
    fn clock_frequency() -> f32;
}

// ------------------------------- RNG1 -------------------------------

impl Instance for RNG1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::RNG1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mp_ahb5ensetr().modify(|_, w| w.rng1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mc_ahb5ensetr().modify(|_, w| w.rng1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mp_ahb5enclrr().modify(|_, w| w.rng1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mc_ahb5enclrr().modify(|_, w| w.rng1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        let csi = rcc::csi::Csi::new();
        csi.frequency() as f32
    }
}

// ------------------------------- RNG2 -------------------------------

impl Instance for RNG2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::RNG2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mp_ahb3ensetr().modify(|_, w| w.rng2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mc_ahb3ensetr().modify(|_, w| w.rng2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mp_ahb3enclrr().modify(|_, w| w.rng2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.mc_ahb3enclrr().modify(|_, w| w.rng2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        let csi = rcc::csi::Csi::new();
        csi.frequency() as f32
    }
}
