//! Inter-integrated circuit interface.
//!
//! Timing calculation taken from <https://github.com/David-OConnor/stm32-hal/blob/main/src/i2c.rs>

// Todo: error handling, timeouts, DMA, 10-bit addresses, slave mode.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ops::Deref;
use core::task::Poll;

use cfg_if::cfg_if;
use embedded_hal as eh;

use crate::pac;
use crate::rcc;
use pac::i2c1::RegisterBlock;
use pac::{I2C1, I2C2, I2C3, I2C4, I2C5, I2C6};

/// I2C peripheral.
#[derive(Debug, Default)]
pub struct I2c<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for I2C1.
pub type I2c1 = I2c<I2C1>;

/// Type alias for I2C2.
pub type I2c2 = I2c<I2C2>;

/// Type alias for I2C3.
pub type I2c3 = I2c<I2C3>;

/// Type alias for I2C4.
pub type I2c4 = I2c<I2C4>;

/// Type alias for I2C5.
pub type I2c5 = I2c<I2C5>;

/// Type alias for I2C6.
pub type I2c6 = I2c<I2C6>;

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct I2cConfig {
    /// Clock speed.
    pub speed: I2cSpeed,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            speed: I2cSpeed::Standard,
        }
    }
}

/// Speed settings.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum I2cSpeed {
    /// Standard Mode: 100kHz.
    Standard,
    /// Fast Mode: 400kHz.
    Fast,
    /// Fast Mode Plus: 1MHz.
    FastPlus,
}

impl I2cSpeed {
    /// Returns the speed in Hz.
    pub fn hz(&self) -> u32 {
        match self {
            Self::Standard => 100_000,
            Self::Fast => 400_000,
            Self::FastPlus => 1_000_000,
        }
    }
}

// ------------------------- Implementation ---------------------------

impl<R> I2c<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Initializes the peripheral.
    pub fn init(&mut self, config: I2cConfig) {
        R::enable_clock();

        self.disable();

        let presc_const = match config.speed {
            I2cSpeed::Standard => 4_000_000,
            I2cSpeed::Fast => 8_000_000,
            I2cSpeed::FastPlus => 8_000_000,
        };

        let presc_val = (R::clock_frequency() as u32 / presc_const).min(16);

        let scll_val = if presc_val == 16 {
            (R::clock_frequency() as u32 / presc_val) / (2 * config.speed.hz())
        } else {
            presc_const / (2 * config.speed.hz())
        };

        let sclh_val = match config.speed {
            I2cSpeed::Standard => scll_val - 4,
            I2cSpeed::Fast => scll_val * 4 / 10,
            I2cSpeed::FastPlus => scll_val / 2,
        };

        let presc = presc_val - 1;
        let scll = scll_val - 1;
        let sclh = sclh_val - 1;

        let sdadel = match config.speed {
            I2cSpeed::Standard => 0x2,
            I2cSpeed::Fast => 0x3,
            I2cSpeed::FastPlus => 0x0,
        };

        let scldel = match config.speed {
            I2cSpeed::Standard => 0x4,
            I2cSpeed::Fast => 0x3,
            I2cSpeed::FastPlus => 0x1,
        };

        assert!(presc <= 15);
        assert!(scldel <= 15);
        assert!(sdadel <= 15);
        assert!(scll <= 255);
        assert!(sclh <= 255);

        let regs = R::registers();

        unsafe {
            regs.i2c_timingr.write(|w| {
                w.presc()
                    .bits(presc as u8)
                    .scldel()
                    .bits(scldel as u8)
                    .sdadel()
                    .bits(sdadel as u8)
                    .sclh()
                    .bits(sclh as u8)
                    .scll()
                    .bits(scll as u8)
            });
        }

        regs.i2c_icr.write(|w| {
            w.addrcf()
                .set_bit()
                .nackcf()
                .set_bit()
                .stopcf()
                .set_bit()
                .berrcf()
                .set_bit()
                .arlocf()
                .set_bit()
                .ovrcf()
                .set_bit()
                .peccf()
                .set_bit()
                .timoutcf()
                .set_bit()
                .alertcf()
                .set_bit()
        });

        self.enable();
    }

    /// Deinitializes the peripheral.
    pub fn deinit(&mut self) {
        self.disable();
        R::disable_clock();
    }

    /// Returns if a device responds at the specified address.
    pub fn is_device_ready(&mut self, address: u8) -> bool {
        let regs = R::registers();

        // Wait for any ongoing operation to be finished.
        while regs.i2c_isr.read().busy().bit_is_set() {}

        // Clear NACK and STOP flags.
        regs.i2c_icr
            .write(|w| w.nackcf().set_bit().stopcf().set_bit());

        unsafe {
            regs.i2c_cr2.modify(|_, w| {
                w.sadd()
                    .bits((address as u16) << 1)
                    .nbytes()
                    .bits(0)
                    .rd_wrn()
                    .clear_bit()
                    .autoend()
                    .set_bit()
                    .start()
                    .set_bit()
            });
        }

        while regs.i2c_isr.read().stopf().bit_is_clear() {}

        let nack = regs.i2c_isr.read().nackf().bit_is_set();

        if nack {
            regs.i2c_icr
                .write(|w| w.nackcf().set_bit().stopcf().set_bit());
        }

        !nack
    }

    /// Reads bytes from the slave asynchronuously.
    pub async fn read_async(
        &mut self,
        address: u8,
        read: &mut [u8],
    ) -> Result<(), eh::i2c::ErrorKind> {
        self.transaction_async(address, &mut [eh::i2c::Operation::Read(read)])
            .await
    }

    /// Writes bytes to the slave asynchronuously.
    pub async fn write_async(
        &mut self,
        address: u8,
        write: &[u8],
    ) -> Result<(), eh::i2c::ErrorKind> {
        self.transaction_async(address, &mut [eh::i2c::Operation::Write(write)])
            .await
    }

    /// Writes a number of bytes to the slave, then reads some bytes back.
    pub async fn write_read_async(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), eh::i2c::ErrorKind> {
        self.transaction_async(
            address,
            &mut [
                eh::i2c::Operation::Write(write),
                eh::i2c::Operation::Read(read),
            ],
        )
        .await
    }

    /// Execute operations on the bus asynchronuously.
    pub async fn transaction_async(
        &mut self,
        address: u8,
        operations: &mut [eh::i2c::Operation<'_>],
    ) -> Result<(), eh::i2c::ErrorKind> {
        let regs = R::registers();

        // Wait for any ongoing operation to be finished.
        self.wait_while_busy_async().await;

        let mut operations = operations.iter_mut().peekable();

        while let Some(operation) = operations.next() {
            // Auto end is only set true on the last operation so that RESTART is used otherwise.
            // This is required for combined write/read within one transaction.
            let autoend = operations.peek().is_none();

            match operation {
                eh::i2c::Operation::Read(buffer) => {
                    unsafe {
                        // Set slave address, transfer size and flags.
                        regs.i2c_cr2.modify(|_, w| {
                            w.sadd()
                                .bits((address as u16) << 1)
                                .nbytes()
                                .bits(buffer.len() as u8)
                                .rd_wrn()
                                .set_bit()
                                .autoend()
                                .bit(autoend)
                                .start()
                                .set_bit()
                        });
                        regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        for byte in buffer.iter_mut() {
                            self.wait_for_receiver_not_empty_async().await;
                            *byte = regs.i2c_rxdr.read().rxdata().bits();
                        }
                        if autoend {
                            self.wait_for_stop_async().await;
                            regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        } else {
                            self.wait_for_transfer_complete_async().await;
                        }
                    }
                }
                eh::i2c::Operation::Write(buffer) => {
                    unsafe {
                        // Set slave address and transfer size.
                        regs.i2c_cr2.modify(|_, w| {
                            w.sadd()
                                .bits((address as u16) << 1)
                                .nbytes()
                                .bits(buffer.len() as u8)
                                .rd_wrn()
                                .clear_bit()
                                .autoend()
                                .bit(autoend)
                                .start()
                                .set_bit()
                        });
                        regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        for byte in buffer.iter() {
                            self.wait_for_transmitter_empty_async().await;
                            regs.i2c_txdr.write(|w| w.txdata().bits(*byte));
                        }
                        if autoend {
                            self.wait_for_stop_async().await;
                            regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        } else {
                            self.wait_for_transfer_complete_async().await;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Enables the peripheral.
    fn enable(&mut self) {
        let regs = R::registers();
        regs.i2c_cr1.modify(|_, w| w.pe().set_bit());
    }

    /// Disables the peripheral.
    fn disable(&mut self) {
        let regs = R::registers();
        regs.i2c_cr1.modify(|_, w| w.pe().clear_bit());
    }

    /// Asynchronuously wait while peripheral is busy.
    pub async fn wait_while_busy_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.i2c_isr.read().busy().bit_is_set() {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    /// Asynchronuously wait for transmitter empty.
    pub async fn wait_for_transmitter_empty_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.i2c_isr.read().txe().bit_is_clear() {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    /// Asynchronuously wait for receiver not empty.
    pub async fn wait_for_receiver_not_empty_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.i2c_isr.read().rxne().bit_is_clear() {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    /// Asynchronuously wait for stop condition.
    pub async fn wait_for_stop_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.i2c_isr.read().stopf().bit_is_clear() {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    /// Asynchronuously wait for transfer complete.
    pub async fn wait_for_transfer_complete_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.i2c_isr.read().tc().bit_is_clear() {
                cx.waker().wake_by_ref();
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await
    }

    /// Returns the register block.
    pub fn registers(&self) -> &'static RegisterBlock {
        R::registers()
    }
}

// --------------------------- embedded-hal ---------------------------

impl<R> eh::i2c::ErrorType for I2c<R>
where
    R: Deref<Target = RegisterBlock>,
{
    type Error = eh::i2c::ErrorKind;
}

impl<R> eh::i2c::I2c for I2c<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [eh::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let regs = R::registers();

        // Wait for any ongoing operation to be finished.
        while regs.i2c_isr.read().busy().bit_is_set() {}

        let mut operations = operations.iter_mut().peekable();

        while let Some(operation) = operations.next() {
            // Auto end is only set true on the last operation so that RESTART is used otherwise.
            // This is required for combined write/read within one transaction.
            let autoend = operations.peek().is_none();

            match operation {
                eh::i2c::Operation::Read(buffer) => {
                    unsafe {
                        // Set slave address, transfer size and flags.
                        regs.i2c_cr2.modify(|_, w| {
                            w.sadd()
                                .bits((address as u16) << 1)
                                .nbytes()
                                .bits(buffer.len() as u8)
                                .rd_wrn()
                                .set_bit()
                                .autoend()
                                .bit(autoend)
                                .start()
                                .set_bit()
                        });
                        regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        for byte in buffer.iter_mut() {
                            while regs.i2c_isr.read().rxne().bit_is_clear() {}
                            *byte = regs.i2c_rxdr.read().rxdata().bits();
                        }
                        if autoend {
                            while regs.i2c_isr.read().stopf().bit_is_clear() {}
                            regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        } else {
                            while regs.i2c_isr.read().tc().bit_is_clear() {}
                        }
                    }
                }
                eh::i2c::Operation::Write(buffer) => {
                    unsafe {
                        // Set slave address and transfer size.
                        regs.i2c_cr2.modify(|_, w| {
                            w.sadd()
                                .bits((address as u16) << 1)
                                .nbytes()
                                .bits(buffer.len() as u8)
                                .rd_wrn()
                                .clear_bit()
                                .autoend()
                                .bit(autoend)
                                .start()
                                .set_bit()
                        });
                        regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        for byte in buffer.iter() {
                            while regs.i2c_isr.read().txe().bit_is_clear() {}
                            regs.i2c_txdr.write(|w| w.txdata().bits(*byte));
                        }
                        if autoend {
                            while regs.i2c_isr.read().stopf().bit_is_clear() {}
                            regs.i2c_icr.write(|w| w.stopcf().set_bit());
                        } else {
                            while regs.i2c_isr.read().tc().bit_is_clear() {}
                        }
                    }
                }
            }
        }

        Ok(())
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

// ------------------------------- I2C1 -------------------------------

impl Instance for I2C1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.i2c1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.i2c1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.i2c1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.i2c1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------- I2C2 -------------------------------

impl Instance for I2C2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.i2c2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.i2c2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.i2c2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.i2c2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------- I2C3 -------------------------------

impl Instance for I2C3 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C3::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.i2c3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.i2c3en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.i2c3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.i2c3en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------- I2C4 -------------------------------

impl Instance for I2C4 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C4::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
                if #[cfg(feature = "mpu-ca7")] {
                    let rcc = unsafe { &(*pac::RCC::ptr()) };
                    rcc.rcc_mp_apb5ensetr.modify(|_, w| w.i2c4en().set_bit());
                } else if #[cfg(feature = "mcu-cm4")] {
                    let rcc = unsafe { &(*pac::RCC::ptr()) };
                    rcc.rcc_mc_apb5ensetr.modify(|_, w| w.i2c4en().set_bit());
                }

        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5enclrr.modify(|_, w| w.i2c4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5enclrr.modify(|_, w| w.i2c4en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk5_frequency()
    }
}

// ------------------------------- I2C5 -------------------------------

impl Instance for I2C5 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C5::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.i2c5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.i2c5en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.i2c5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.i2c5en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------- I2C6 -------------------------------

impl Instance for I2C6 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::I2C6::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5ensetr.modify(|_, w| w.i2c6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5ensetr.modify(|_, w| w.i2c6en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5enclrr.modify(|_, w| w.i2c6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5enclrr.modify(|_, w| w.i2c6en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk5_frequency()
    }
}
