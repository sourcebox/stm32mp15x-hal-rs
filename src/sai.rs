//! Serial audio interface.

use core::marker::PhantomData;
use core::ops::Deref;

use cfg_if::cfg_if;

use crate::pac;
use crate::rcc;
use pac::sai1::RegisterBlock;
use pac::{SAI1, SAI2, SAI3, SAI4};

/// SAI peripheral.
#[derive(Debug, Default)]
pub struct Sai<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for SAI1.
pub type Sai1 = Sai<SAI1>;

/// Type alias for SAI2.
pub type Sai2 = Sai<SAI2>;

/// Type alias for SAI3.
pub type Sai3 = Sai<SAI3>;

/// Type alias for SAI4.
pub type Sai4 = Sai<SAI4>;

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct SaiConfig {
    /// SAI mode.
    pub mode: SaiMode,
    /// Mono mode.
    pub mono: bool,
    /// Clock strobing.
    pub clock_strobing: ClockStrobing,
    /// Least significant bit first.
    pub lsb_first: bool,
    /// Master clock generation enable.
    pub mclk_enable: bool,
    /// Master clock divider.
    pub mclk_divider: u8,
    /// Disable clock divider.
    pub no_divider: bool,
    /// Oversampling ratio.
    pub oversampling_ratio: OversamplingRatio,
    /// Protocol.
    pub protocol: Protocol,
    /// Data size.
    pub data_size: DataSize,
    /// Enable DMA transfers.
    pub dma_enable: bool,
    /// Frame length.
    pub frame_length: u8,
    /// Frame sync length.
    pub frame_sync_length: u8,
    /// Frame sync offset.
    pub frame_sync_offset: FrameSyncOffset,
    /// Frame sync polarity.
    pub frame_sync_polarity: FrameSyncPolarity,
    /// Frame sync definition.
    pub frame_sync_definition: FrameSyncDefinition,
    /// Slot size.
    pub slot_size: SlotSize,
    /// Slot enable bits.
    pub slot_enable: u16,
    /// Number of slots.
    pub slot_num: u8,
    /// First bit offset.
    pub first_bit_offset: u8,
}

impl Default for SaiConfig {
    /// Returns the default configuration which is:
    /// - Transmitter
    /// - Left aligned, 2 channels with 16-bit data size.
    /// - Master clock enabled.
    fn default() -> Self {
        Self {
            mode: SaiMode::MasterTransmitter,
            mono: false,
            clock_strobing: ClockStrobing::FallingEdge,
            lsb_first: false,
            mclk_enable: true,
            mclk_divider: 8,
            no_divider: false,
            oversampling_ratio: OversamplingRatio::Times256,
            protocol: Protocol::Free,
            data_size: DataSize::Bits16,
            dma_enable: false,
            frame_length: 32,
            frame_sync_length: 16,
            frame_sync_offset: FrameSyncOffset::FirstBit,
            frame_sync_polarity: FrameSyncPolarity::ActiveLow,
            frame_sync_definition: FrameSyncDefinition::ChannelIdent,
            slot_size: SlotSize::DataSize,
            slot_enable: 0xFFFF,
            slot_num: 2,
            first_bit_offset: 0,
        }
    }
}

/// SAI mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SaiMode {
    /// Master transmitter.
    MasterTransmitter,
    /// Master receiver.
    MasterReceiver,
    /// Slave transmitter.
    SlaveTransmitter,
    /// Slave receiver.
    SlaveReceiver,
}

impl From<SaiMode> for u8 {
    fn from(value: SaiMode) -> Self {
        match value {
            SaiMode::MasterTransmitter => 0b00,
            SaiMode::MasterReceiver => 0b01,
            SaiMode::SlaveTransmitter => 0b10,
            SaiMode::SlaveReceiver => 0b11,
        }
    }
}

/// Clock edge strobing for generated and received SCK signals.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClockStrobing {
    /// Signals generated change on SCK rising edge, signals received are sampled on the falling edge.
    RisingEdge,
    /// Signals generated change on SCK falling edge, signals received are sampled on the rising edge.
    FallingEdge,
}

impl From<ClockStrobing> for bool {
    fn from(value: ClockStrobing) -> Self {
        match value {
            ClockStrobing::RisingEdge => false,
            ClockStrobing::FallingEdge => true,
        }
    }
}

/// Oversampling ratio for master clock.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OversamplingRatio {
    /// FS * 256
    Times256 = 0b0,
    /// FS * 512
    Times512 = 0b1,
}

impl From<OversamplingRatio> for bool {
    fn from(value: OversamplingRatio) -> Self {
        match value {
            OversamplingRatio::Times256 => false,
            OversamplingRatio::Times512 => true,
        }
    }
}

/// Audio protocol to use.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Protocol {
    /// Free protocol.
    Free = 0b00,
    /// SPDIF protocol.
    Spdif = 0b01,
    /// AC'97 protocol
    Ac97 = 0b10,
}

impl From<Protocol> for u8 {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::Free => 0b00,
            Protocol::Spdif => 0b01,
            Protocol::Ac97 => 0b10,
        }
    }
}

/// Data size.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DataSize {
    /// 8 bits.
    Bits8 = 0b010,
    /// 10 bits.
    Bits10 = 0b011,
    /// 16 bits.
    Bits16 = 0b100,
    /// 20 bits.
    Bits20 = 0b101,
    /// 24 bits.
    Bits24 = 0b110,
    /// 32 bits.
    Bits32 = 0b111,
}

impl From<DataSize> for u8 {
    fn from(value: DataSize) -> Self {
        match value {
            DataSize::Bits8 => 0b010,
            DataSize::Bits10 => 0b011,
            DataSize::Bits16 => 0b100,
            DataSize::Bits20 => 0b101,
            DataSize::Bits24 => 0b110,
            DataSize::Bits32 => 0b111,
        }
    }
}

/// Frame synchonization offset.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameSyncOffset {
    /// First bit of the slot 0.
    FirstBit = 0b0,
    /// Before the first bit of the slot 0.
    BeforeFirstBit = 0b1,
}

impl From<FrameSyncOffset> for bool {
    fn from(value: FrameSyncOffset) -> Self {
        match value {
            FrameSyncOffset::FirstBit => false,
            FrameSyncOffset::BeforeFirstBit => true,
        }
    }
}

/// Frame synchonization polarity.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameSyncPolarity {
    /// Active low (falling edge).
    ActiveLow = 0b0,
    /// Active high (rising edge).
    ActiveHigh = 0b1,
}

impl From<FrameSyncPolarity> for bool {
    fn from(value: FrameSyncPolarity) -> Self {
        match value {
            FrameSyncPolarity::ActiveLow => false,
            FrameSyncPolarity::ActiveHigh => true,
        }
    }
}

/// Frame synchonization definition.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FrameSyncDefinition {
    /// Start frame signal.
    StartFrame = 0b0,
    /// Start of frame signal & channel side identification.
    ChannelIdent = 0b1,
}

impl From<FrameSyncDefinition> for bool {
    fn from(value: FrameSyncDefinition) -> Self {
        match value {
            FrameSyncDefinition::StartFrame => false,
            FrameSyncDefinition::ChannelIdent => true,
        }
    }
}

/// Slot size.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SlotSize {
    /// Equal to data size.
    DataSize = 0b00,
    /// 16 bits
    Bits16 = 0b01,
    /// 32 bits
    Bits32 = 0b10,
}

impl From<SlotSize> for u8 {
    fn from(value: SlotSize) -> Self {
        match value {
            SlotSize::DataSize => 0b00,
            SlotSize::Bits16 => 0b01,
            SlotSize::Bits32 => 0b10,
        }
    }
}

// ------------------------- Implementation ---------------------------

impl<R> Sai<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self { _regs: PhantomData }
    }

    /// Initializes block A.
    pub fn init_block_a(&mut self, config: SaiConfig) {
        R::enable_clock();

        self.disable_block_a();

        unsafe {
            let regs = R::registers();
            regs.sai_acr1.modify(|_, w| {
                w.mode()
                    .bits(config.mode.into())
                    .mono()
                    .bit(config.mono)
                    .ckstr()
                    .bit(config.clock_strobing.into())
                    .lsbfirst()
                    .bit(config.lsb_first)
                    .mcken()
                    .bit(config.mclk_enable)
                    .mckdiv()
                    .bits(config.mclk_divider)
                    .nodiv()
                    .bit(config.no_divider)
                    .osr()
                    .bit(config.oversampling_ratio.into())
                    .prtcfg()
                    .bits(config.protocol.into())
                    .ds()
                    .bits(config.data_size.into())
            });

            // DMA bit should be set after mode.
            regs.sai_acr1
                .modify(|_, w| w.dmaen().bit(config.dma_enable));

            regs.sai_afrcr.modify(|_, w| {
                w.fsoff()
                    .bit(config.frame_sync_offset.into())
                    .fspol()
                    .bit(config.frame_sync_polarity.into())
                    .frl()
                    .bits(config.frame_length - 1)
                    .fsall()
                    .bits(config.frame_sync_length - 1)
            });

            // FSDEF bit is missing in PAC, so handle it manually.
            match config.frame_sync_definition {
                FrameSyncDefinition::StartFrame => {
                    regs.sai_afrcr.modify(|r, w| w.bits(r.bits() & !(1 << 16)));
                }
                FrameSyncDefinition::ChannelIdent => {
                    regs.sai_afrcr.modify(|r, w| w.bits(r.bits() | (1 << 16)));
                }
            }

            regs.sai_aslotr.modify(|_, w| {
                w.slotsz()
                    .bits(config.slot_size.into())
                    .sloten()
                    .bits(config.slot_enable)
                    .nbslot()
                    .bits(config.slot_num - 1)
                    .fboff()
                    .bits(config.first_bit_offset)
            });
        }

        self.enable_block_a();
    }

    /// Initializes block B.
    pub fn init_block_b(&mut self, config: SaiConfig) {
        R::enable_clock();

        self.disable_block_b();

        unsafe {
            let regs = R::registers();
            regs.sai_bcr1.modify(|_, w| {
                w.mode()
                    .bits(config.mode.into())
                    .mono()
                    .bit(config.mono)
                    .ckstr()
                    .bit(config.clock_strobing.into())
                    .lsbfirst()
                    .bit(config.lsb_first)
                    .mcken()
                    .bit(config.mclk_enable)
                    .mckdiv()
                    .bits(config.mclk_divider)
                    .nodiv()
                    .bit(config.no_divider)
                    .osr()
                    .bit(config.oversampling_ratio.into())
                    .prtcfg()
                    .bits(config.protocol.into())
                    .ds()
                    .bits(config.data_size.into())
            });

            // DMA bit should be set after mode.
            regs.sai_bcr1
                .modify(|_, w| w.dmaen().bit(config.dma_enable));

            regs.sai_bfrcr.modify(|_, w| {
                w.fsoff()
                    .bit(config.frame_sync_offset.into())
                    .fspol()
                    .bit(config.frame_sync_polarity.into())
                    .frl()
                    .bits(config.frame_length - 1)
                    .fsall()
                    .bits(config.frame_sync_length - 1)
            });

            // FSDEF bit is missing in PAC, so handle it manually.
            match config.frame_sync_definition {
                FrameSyncDefinition::StartFrame => {
                    regs.sai_bfrcr.modify(|r, w| w.bits(r.bits() & !(1 << 16)));
                }
                FrameSyncDefinition::ChannelIdent => {
                    regs.sai_bfrcr.modify(|r, w| w.bits(r.bits() | (1 << 16)));
                }
            }

            regs.sai_bslotr.modify(|_, w| {
                w.slotsz()
                    .bits(config.slot_size.into())
                    .sloten()
                    .bits(config.slot_enable)
                    .nbslot()
                    .bits(config.slot_num - 1)
                    .fboff()
                    .bits(config.first_bit_offset)
            });
        }

        self.enable_block_b();
    }

    /// Deinitializes the peripheral completely (block A & B).
    pub fn deinit(&mut self) {
        self.disable_block_a();
        self.disable_block_b();
        R::disable_clock();
    }

    /// Enables the block A.
    fn enable_block_a(&mut self) {
        let regs = R::registers();
        regs.sai_acr1.modify(|_, w| w.saien().set_bit());
    }

    /// Enables the block B.
    fn enable_block_b(&mut self) {
        let regs = R::registers();
        regs.sai_bcr1.modify(|_, w| w.saien().set_bit());
    }

    /// Disables the block A.
    fn disable_block_a(&mut self) {
        let regs = R::registers();
        regs.sai_acr1.modify(|_, w| w.saien().clear_bit());
        while regs.sai_acr1.read().saien().bit_is_set() {}
    }

    /// Disables the block B.
    fn disable_block_b(&mut self) {
        let regs = R::registers();
        regs.sai_bcr1.modify(|_, w| w.saien().clear_bit());
        while regs.sai_bcr1.read().saien().bit_is_set() {}
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

// ------------------------------- SAI1 -------------------------------

impl Instance for SAI1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SAI1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.sai1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.sai1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.sai1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.sai1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SAI2 -------------------------------

impl Instance for SAI2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SAI2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.sai2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.sai2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.sai2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.sai2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SAI3 -------------------------------

impl Instance for SAI3 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SAI3::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2ensetr.modify(|_, w| w.sai3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2ensetr.modify(|_, w| w.sai3en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb2enclrr.modify(|_, w| w.sai3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb2enclrr.modify(|_, w| w.sai3en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}

// ------------------------------- SAI4 -------------------------------

impl Instance for SAI4 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SAI4::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb3ensetr.modify(|_, w| w.sai4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb3ensetr.modify(|_, w| w.sai4en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb3enclrr.modify(|_, w| w.sai4en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb3enclrr.modify(|_, w| w.sai4en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::pll4_p_frequency()
    }
}
