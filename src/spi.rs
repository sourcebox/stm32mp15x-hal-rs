//! Serial peripheral interface.

use core::marker::PhantomData;
use core::ops::Deref;

use cfg_if::cfg_if;

use crate::pac;
use crate::rcc;
use pac::spi1::RegisterBlock;
use pac::{SPI1, SPI2, SPI3, SPI4, SPI5, SPI6};

/// SPI peripheral.
#[derive(Debug, Default)]
pub struct Spi<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for SPI1.
pub type Spi1 = Spi<SPI1>;

/// Type alias for SPI2.
pub type Spi2 = Spi<SPI2>;

/// Type alias for SPI3.
pub type Spi3 = Spi<SPI3>;

/// Type alias for SPI4.
pub type Spi4 = Spi<SPI4>;

/// Type alias for SPI5.
pub type Spi5 = Spi<SPI5>;

/// Type alias for SPI6.
pub type Spi6 = Spi<SPI6>;

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct SpiConfig {
    /// Master mode enable.
    pub master_mode: bool,
    /// Communication mode.
    pub communication_mode: CommunicationMode,
    /// Clock prescaler.
    pub clock_prescaler: ClockPrescaler,
    /// Clock idle polarity.
    pub clock_polarity: ClockPolarity,
    /// Clock capture transition phase.
    pub clock_phase: ClockPhase,
    /// Data frame size, range is 4-32 bits.
    pub data_size: u8,
    /// FIFO threshold level, range is 1-16.
    pub fifo_threshold_level: u8,
    /// Least significant bit first.
    pub lsb_first: bool,
    /// Serial protocol mode.
    pub protocol_mode: ProtocolMode,
    /// SS output enable.
    pub ss_output_enable: bool,
    /// SS input/output active polarity.
    pub ss_polarity: SsPolarity,
    /// Enable DMA transfers for transmitter.
    pub tx_dma_enable: bool,
    /// Enable DMA transfers for receiver.
    pub rx_dma_enable: bool,
    /// Swap functionality of MISO and MOSI pins.
    pub swap_miso_mosi: bool,
    /// Delay in clock cycles inserted inbetween frames in master mode, range is 0-15.
    pub master_inter_data_idleness: u8,
    /// Delay in clock cycles inserted after SS going active in master mode, range is 0-15.
    pub master_ss_idleness: u8,
}

impl Default for SpiConfig {
    /// Returns the default configuration:
    /// - Master simplex transmitter mode.
    /// - Clock prescaler 1/16.
    /// - Clock idle polarity low (CPOL=0).
    /// - Clock data capture on first transition (CPHA=0).
    /// - 8 bits data size.
    /// - MSB first.
    /// - Motorola serial protocol.
    /// - SS output enabled.
    fn default() -> Self {
        Self {
            master_mode: true,
            communication_mode: CommunicationMode::SimplexTransmitter,
            clock_prescaler: ClockPrescaler::Div16,
            clock_polarity: ClockPolarity::Low,
            clock_phase: ClockPhase::First,
            data_size: 8,
            fifo_threshold_level: 1,
            lsb_first: false,
            protocol_mode: ProtocolMode::Motorola,
            ss_output_enable: true,
            ss_polarity: SsPolarity::Low,
            tx_dma_enable: false,
            rx_dma_enable: false,
            swap_miso_mosi: false,
            master_inter_data_idleness: 0,
            master_ss_idleness: 0,
        }
    }
}

/// Communication mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum CommunicationMode {
    /// Full-duplex.
    FullDuplex = 0b00,
    /// Simplex transmitter.
    SimplexTransmitter = 0b01,
    /// Simplex receiver.
    SimplexReceiver = 0b10,
    /// Half-duplex
    HalfDuplex = 0b11,
}

/// Clock prescaler.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ClockPrescaler {
    /// Divided by 2.
    Div2 = 0b000,
    /// Divided by 4.
    Div4 = 0b001,
    /// Divided by 8.
    Div8 = 0b010,
    /// Divided by 16.
    Div16 = 0b011,
    /// Divided by 32.
    Div32 = 0b100,
    /// Divided by 64.
    Div64 = 0b101,
    /// Divided by 128.
    Div128 = 0b110,
    /// Divided by 256.
    Div256 = 0b111,
}

/// Polarity when clock is idle.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ClockPolarity {
    /// Low.
    Low = 0b0,
    /// High.
    High = 0b1,
}

/// Clock transition when data is captured.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ClockPhase {
    /// Data captured on first transition.
    First = 0b0,
    /// Data captured on second transition.
    Second = 0b1,
}

/// Serial protocol mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum ProtocolMode {
    /// Motorola serial protocol.
    Motorola = 0b000,
    /// TI serial protocol.
    Ti = 0b001,
}

/// SS input/output active polarity.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum SsPolarity {
    /// Low level active.
    Low = 0b0,
    /// High level active.
    High = 0b1,
}

// ------------------------- Implementation ---------------------------

impl<R> Spi<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Initializes the peripheral.
    pub fn init(&mut self, config: SpiConfig) {
        R::enable_clock();

        self.disable();

        let regs = R::registers();

        unsafe {
            regs.spi_cfg1.modify(|_, w| {
                w.dsize()
                    .bits(config.data_size - 1)
                    .fthlv()
                    .bits(config.fifo_threshold_level - 1)
                    .txdmaen()
                    .bit(config.tx_dma_enable)
                    .rxdmaen()
                    .bit(config.rx_dma_enable)
                    .mbr()
                    .bits(config.clock_prescaler as u8)
            });
            regs.spi_cfg2.modify(|_, w| {
                w.mssi()
                    .bits(config.master_ss_idleness)
                    .midi()
                    .bits(config.master_inter_data_idleness)
                    .ioswp()
                    .bit(config.swap_miso_mosi)
                    .comm()
                    .bits(config.communication_mode as u8)
                    .sp()
                    .bits(config.protocol_mode as u8)
                    .master()
                    .bit(config.master_mode)
                    .lsbfrst()
                    .bit(config.lsb_first)
                    .cpha()
                    .bit(config.clock_phase == ClockPhase::Second)
                    .cpol()
                    .bit(config.clock_polarity == ClockPolarity::High)
                    .ssiop()
                    .bit(config.ss_polarity == SsPolarity::High)
                    .ssoe()
                    .bit(config.ss_output_enable)
            });
        }

        self.enable();
    }

    /// Deinitializes the peripheral.
    pub fn deinit(&mut self) {
        self.disable();
        R::disable_clock();
    }

    /// Write bytes from a buffer, blocking.
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.set_transfer_size(data.len() as u16);
        self.clear_transmission_transfer_filled();
        for byte in data {
            self.write_tx_fifo_byte(*byte);
        }
        while !self.is_transmission_transfer_filled() {}
        self.start_transfer();
        while !self.is_end_of_transfer() {}
        self.clear_end_of_transfer();
    }

    /// Writes a byte to the TxFIFO.
    pub fn write_tx_fifo_byte(&mut self, byte: u8) {
        let regs = R::registers();
        unsafe {
            core::ptr::write_volatile(regs.spi2s_txdr.as_ptr() as *mut u8, byte);
        }
    }

    /// Reads a byte from the RxFIFO.
    pub fn read_rx_fifo_byte(&mut self) -> u8 {
        let regs = R::registers();
        unsafe { core::ptr::read_volatile(regs.spi2s_rxdr.as_ptr() as *mut u8) }
    }

    /// Sets the transfer size.
    pub fn set_transfer_size(&mut self, size: u16) {
        let enabled = self.is_enabled();
        self.disable();
        let regs = R::registers();
        unsafe {
            regs.spi_cr2.modify(|_, w| w.tsize().bits(size));
        }
        if enabled {
            self.enable();
        }
    }

    /// Starts the transfer.
    ///
    /// Data must be written to the TxFIFO and transfer size has to be set before.
    pub fn start_transfer(&mut self) {
        self.enable();
        let regs = R::registers();
        regs.spi2s_cr1.modify(|_, w| w.cstart().set_bit());
    }

    /// Enables the peripheral.
    pub fn enable(&mut self) {
        let regs = R::registers();
        regs.spi2s_cr1.modify(|_, w| w.spe().set_bit());
    }

    /// Disables the peripheral.
    pub fn disable(&mut self) {
        let regs = R::registers();
        regs.spi2s_cr1.modify(|_, w| w.spe().clear_bit());
    }

    /// Returns if the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_cr1.read().spe().bit_is_set()
    }

    /// Returns if the TxFIFO has at least one packet of space available.
    pub fn is_transmitter_empty(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().txp().bit_is_set()
    }

    /// Returns if the RxFIFO contains at least one packet.
    pub fn is_receiver_not_empty(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().rxp().bit_is_set()
    }

    /// Returns if transmission transfer is filled.
    pub fn is_transmission_transfer_filled(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().txtf().bit_is_set()
    }

    /// Returns if transfer is complete.
    pub fn is_end_of_transfer(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().eot().bit_is_set()
    }

    /// Returns if an overrun error has occurred.
    pub fn is_overrun_error(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().ovr().bit_is_set()
    }

    /// Returns if an underrun error has occurred.
    pub fn is_underrun_error(&self) -> bool {
        let regs = R::registers();
        regs.spi2s_sr.read().udr().bit_is_set()
    }

    /// Clears the transmission transfer filled flag.
    pub fn clear_transmission_transfer_filled(&self) {
        let regs = R::registers();
        regs.spi2s_ifcr.write(|w| w.txtfc().set_bit());
    }

    /// Clears the end of transfer flag.
    pub fn clear_end_of_transfer(&self) {
        let regs = R::registers();
        regs.spi2s_ifcr.write(|w| w.eotc().set_bit());
    }

    /// Clears an overrun error.
    pub fn clear_overrun_error(&mut self) {
        let regs = R::registers();
        regs.spi2s_ifcr.write(|w| w.ovrc().set_bit());
    }

    /// Clears an underrun error.
    pub fn clear_underrun_error(&mut self) {
        let regs = R::registers();
        regs.spi2s_ifcr.write(|w| w.udrc().set_bit());
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

// ------------------------------- SPI1 -------------------------------

impl Instance for SPI1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.spi1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.spi1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.spi1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.spi1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SPI2 -------------------------------

impl Instance for SPI2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.spi2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.spi2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.spi2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.spi2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SPI3 -------------------------------

impl Instance for SPI3 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI3::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1ensetr.modify(|_, w| w.spi3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1ensetr.modify(|_, w| w.spi3en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb1enclrr.modify(|_, w| w.spi3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb1enclrr.modify(|_, w| w.spi3en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SPI4 -------------------------------

impl Instance for SPI4 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI4::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.spi4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.spi4en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.spi4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.spi4en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk2_frequency()
    }
}

// ------------------------------- SPI5 -------------------------------

impl Instance for SPI5 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI5::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.spi5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.spi5en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.spi5en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.spi5en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk2_frequency()
    }
}

// ------------------------------- SPI6 -------------------------------

impl Instance for SPI6 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SPI6::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5ensetr.modify(|_, w| w.spi6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5ensetr.modify(|_, w| w.spi6en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb5enclrr.modify(|_, w| w.spi6en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb5enclrr.modify(|_, w| w.spi6en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pclk5_frequency()
    }
}
