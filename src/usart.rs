//! Universal synchronous/asynchronous receiver transmitter.

use core::future::poll_fn;
use core::marker::PhantomData;
use core::ops::Deref;
use core::task::Poll;

use cfg_if::cfg_if;

use crate::bitworker::bitmask;
use crate::pac;
use crate::rcc;
use pac::usart1::RegisterBlock;
use pac::{USART1, USART2, USART3, USART4, USART5, USART6, USART7, USART8};

/// USART peripheral.
#[derive(Debug, Default)]
pub struct Usart<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for USART1.
pub type Usart1 = Usart<USART1>;

/// Type alias for USART2.
pub type Usart2 = Usart<USART2>;

/// Type alias for USART3.
pub type Usart3 = Usart<USART3>;

/// Type alias for USART4.
pub type Usart4 = Usart<USART4>;

/// Type alias for USART5.
pub type Usart5 = Usart<USART5>;

/// Type alias for USART6.
pub type Usart6 = Usart<USART6>;

/// Type alias for USART7.
pub type Usart7 = Usart<USART7>;

/// Type alias for USART1.
pub type Usart8 = Usart<USART8>;

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct UsartConfig {
    /// Baudrate
    pub baudrate: u32,
    /// Parity control.
    pub parity: Parity,
    /// Stop bits.
    pub stop_bits: StopBits,
    /// Word length.
    pub word_length: WordLength,
    /// Oversampling mode.
    pub oversampling: OverSampling,
    /// Transmitter enable.
    pub transmitter_enable: bool,
    /// Receiver enable.
    pub receiver_enable: bool,
    /// FIFO mode enable.
    pub fifo_mode: bool,
}

impl Default for UsartConfig {
    /// Returns the default configuration:
    /// - 115200 baud.
    /// - No parity.
    /// - 8 data bits.
    fn default() -> Self {
        Self {
            baudrate: 115200,
            parity: Parity::None,
            stop_bits: StopBits::Bits1,
            word_length: WordLength::Bits8,
            oversampling: OverSampling::Times16,
            transmitter_enable: false,
            receiver_enable: false,
            fifo_mode: true,
        }
    }
}

/// Parity.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Parity {
    /// No parity.
    None,
    /// Even parity.
    Even,
    /// Odd parity.
    Odd,
}

/// Stop bits.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum StopBits {
    /// 1 stop bit.
    Bits1 = 0b00,
    /// 0.5 stop bit.
    Bits0_5 = 0b01,
    /// 2 stop bits.
    Bits2 = 0b10,
    /// 1.5 stop bits.
    Bits1_5 = 0b11,
}

impl From<StopBits> for u8 {
    fn from(value: StopBits) -> Self {
        value as u8
    }
}

/// Word length including the parity bit.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WordLength {
    /// 8 bits.
    Bits8,
    /// 9 bits.
    Bits9,
    /// 7 bits.
    Bits7,
}

impl WordLength {
    /// Returns (M1, M0) bits tuple.
    pub fn bits(&self) -> (bool, bool) {
        match self {
            Self::Bits8 => (false, false),
            Self::Bits9 => (false, true),
            Self::Bits7 => (true, false),
        }
    }
}

/// Oversampling mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum OverSampling {
    /// Oversampling by 16.
    Times16 = 0b0,
    /// Oversampling by 8.
    Times8 = 0b1,
}

impl From<OverSampling> for bool {
    fn from(value: OverSampling) -> Self {
        value == OverSampling::Times8
    }
}

// ----------------------------- Errors -------------------------------

/// Errors
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// Parity check error.
    Parity,
    /// Framing error.
    Framing,
    /// Receive buffer overrun.
    Overrun,
    /// Noise error.
    Noise,
}

// ------------------------- Implementation ---------------------------

impl<R> Usart<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Initializes the peripheral.
    pub fn init(&mut self, config: UsartConfig) {
        R::enable_clock();

        self.disable();

        let divider = (R::clock_frequency() / config.baudrate as f32) as u32;

        let brr = match config.oversampling {
            OverSampling::Times16 => divider,
            OverSampling::Times8 => {
                let upper_mask = bitmask(12, 4);
                let lower_mask = bitmask(4, 0);
                (divider & upper_mask) | ((divider & lower_mask) >> 1)
            }
        };

        let regs = R::registers();

        regs.cr1.modify(|_, w| {
            w.te()
                .bit(config.transmitter_enable)
                .re()
                .bit(config.receiver_enable)
                .m0()
                .bit(config.word_length.bits().0)
                .m1()
                .bit(config.word_length.bits().1)
                .pce()
                .bit(config.parity != Parity::None)
                .ps()
                .bit(config.parity == Parity::Odd)
                .over8()
                .bit(config.oversampling.into())
                .fifoen()
                .bit(config.fifo_mode)
        });

        unsafe {
            regs.cr2
                .modify(|_, w| w.stop().bits(config.stop_bits.into()));
            regs.brr.write(|w| w.bits(brr));
        }

        self.enable();

        // Discard any received data.
        while self.read_ready().unwrap_or(false) {
            self.read_one().ok();
        }
    }

    /// Deinitializes the peripheral.
    pub fn deinit(&mut self) {
        self.disable();
        R::disable_clock();
    }

    /// Returns if bytes have been received and can be read.
    pub fn read_ready(&self) -> Result<bool, Error> {
        Ok(self.is_receiver_not_empty())
    }

    /// Returns one byte from the receiver, blocks if none available.
    pub fn read_one(&mut self) -> Result<u8, Error> {
        while !self.read_ready()? {}

        if self.is_parity_error() {
            self.clear_parity_error();
            return Err(Error::Parity);
        } else if self.is_framing_error() {
            self.clear_framing_error();
            return Err(Error::Framing);
        } else if self.is_overrun_error() {
            self.clear_overrun_error();
            return Err(Error::Overrun);
        } else if self.is_noise_detected() {
            self.clear_noise_detected();
            return Err(Error::Noise);
        }

        let regs = R::registers();
        Ok((regs.rdr.read().bits() & 0xFF) as u8)
    }

    /// Writes received bytes into a buffer, blocks if none available.
    /// Returns the total number of read bytes.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        while !self.read_ready()? {}

        let mut count = 0;

        for byte in buffer.iter_mut() {
            *byte = self.read_one()?;
            count += 1;
            if !self.read_ready()? {
                break;
            }
        }

        Ok(count)
    }

    /// Returns one byte from the receiver asynchronuously.
    pub async fn read_one_async(&mut self) -> Result<u8, Error> {
        self.wait_for_receiver_not_empty_async().await;

        if self.is_parity_error() {
            self.clear_parity_error();
            return Err(Error::Parity);
        } else if self.is_framing_error() {
            self.clear_framing_error();
            return Err(Error::Framing);
        } else if self.is_overrun_error() {
            self.clear_overrun_error();
            return Err(Error::Overrun);
        } else if self.is_noise_detected() {
            self.clear_noise_detected();
            return Err(Error::Noise);
        }

        let regs = R::registers();
        Ok((regs.rdr.read().bits() & 0xFF) as u8)
    }

    /// Writes bytes from a buffer, blocking.
    pub fn write(&mut self, buffer: &[u8]) {
        unsafe {
            for byte in buffer {
                let regs = R::registers();
                regs.tdr.write(|w| w.bits(*byte as u32));
                while !self.is_transmitter_empty() {}
            }
        }
        while !self.is_transfer_complete() {}
    }

    /// Writes bytes from a buffer asynchronuously.
    pub async fn write_async(&mut self, buffer: &[u8]) {
        unsafe {
            for byte in buffer {
                let regs = R::registers();
                regs.tdr.write(|w| w.bits(*byte as u32));
                self.wait_for_transmitter_empty_async().await;
            }
        }
        self.wait_for_transfer_complete_async().await;
    }

    /// Enables the peripheral.
    pub fn enable(&mut self) {
        let regs = R::registers();
        regs.cr1.modify(|_, w| w.ue().set_bit());
    }

    /// Disables the peripheral.
    pub fn disable(&mut self) {
        let regs = R::registers();
        regs.cr1.modify(|_, w| w.ue().clear_bit());
    }

    /// Returns if the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        let regs = R::registers();
        regs.cr1.read().ue().bit_is_set()
    }

    /// Returns if the transmitter is enabled.
    pub fn is_transmitter_enabled(&self) -> bool {
        let regs = R::registers();
        regs.cr1.read().te().bit_is_set()
    }

    /// Returns if the receiver is enabled.
    pub fn is_receiver_enabled(&self) -> bool {
        let regs = R::registers();
        regs.cr1.read().re().bit_is_set()
    }

    /// Returns if the transmitter is empty.
    pub fn is_transmitter_empty(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().txe().bit_is_set()
    }

    /// Returns if the receiver is not empty.
    pub fn is_receiver_not_empty(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().rxne().bit_is_set()
    }

    /// Returns if the transfer is complete.
    pub fn is_transfer_complete(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().tc().bit_is_set()
    }

    /// Returns if the line is idle.
    pub fn is_idle(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().idle().bit_is_set()
    }

    /// Returns if a parity error has occurred.
    pub fn is_parity_error(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().pe().bit_is_set()
    }

    /// Returns if a framing error has occurred.
    pub fn is_framing_error(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().fe().bit_is_set()
    }

    /// Returns if an overrun error has occurred.
    pub fn is_overrun_error(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().ore().bit_is_set()
    }

    /// Returns if noise was detected on a received frame.
    pub fn is_noise_detected(&self) -> bool {
        let regs = R::registers();
        regs.isr.read().nf().bit_is_set()
    }

    /// Clears a parity error.
    pub fn clear_parity_error(&mut self) {
        let regs = R::registers();
        regs.icr.write(|w| w.pecf().set_bit());
    }

    /// Clears a framing error.
    pub fn clear_framing_error(&mut self) {
        let regs = R::registers();
        regs.icr.write(|w| w.fecf().set_bit());
    }

    /// Clears an overrun error.
    pub fn clear_overrun_error(&mut self) {
        let regs = R::registers();
        regs.icr.write(|w| w.orecf().set_bit());
    }

    /// Clears a detected noise condition.
    pub fn clear_noise_detected(&mut self) {
        let regs = R::registers();
        regs.icr.write(|w| w.ncf().set_bit());
    }

    /// Asynchronuously wait for transmitter empty.
    pub async fn wait_for_transmitter_empty_async(&self) {
        poll_fn(|cx| {
            let regs = R::registers();
            if regs.isr.read().txe().bit_is_clear() {
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
            if regs.isr.read().rxne().bit_is_clear() {
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
            if regs.isr.read().tc().bit_is_clear() {
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

// ------------------------------ USART1 ------------------------------

impl Instance for USART1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5ensetr.modify(|_, w| w.usart1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5ensetr.modify(|_, w| w.usart1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5enclrr.modify(|_, w| w.usart1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5enclrr.modify(|_, w| w.usart1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk5_frequency()
    }
}

// ------------------------------ USART2 ------------------------------

impl Instance for USART2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.usart2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.usart2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.usart2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.usart2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------ USART3 ------------------------------

impl Instance for USART3 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART3::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.usart3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.usart3en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.usart3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.usart3en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------ USART4 ------------------------------

impl Instance for USART4 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART4::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.uart4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.uart4en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.uart4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.uart4en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------ USART5 ------------------------------

impl Instance for USART5 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART5::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.uart5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.uart5en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.uart5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.uart5en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------ USART6 ------------------------------

impl Instance for USART6 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART6::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.usart6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.usart6en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.usart6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.usart6en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk2_frequency()
    }
}

// ------------------------------ USART7 ------------------------------

impl Instance for USART7 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART7::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.uart7en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.uart7en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.uart7en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.uart7en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}

// ------------------------------ USART8 ------------------------------

impl Instance for USART8 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::USART8::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.uart8en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.uart8en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.uart8en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.uart8en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk1_frequency()
    }
}
