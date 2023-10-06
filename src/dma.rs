//! Direct memory access controller.

use cfg_if::cfg_if;

use crate::pac;

pub use crate::dmamux::DmaRequestInput;

/// Initializes DMA peripherals by enabling the clocks.
pub fn init() {
    cfg_if! {
        if #[cfg(feature = "mpu-ca7")] {
            let rcc = unsafe { &(*pac::RCC::ptr()) };
            rcc.rcc_mp_ahb2ensetr.modify(|_, w|
                w.dma1en().set_bit().dma2en().set_bit().dmamuxen().set_bit());
        } else if #[cfg(feature = "mcu-cm4")] {
            let rcc = unsafe { &(*pac::RCC::ptr()) };
            rcc.rcc_mp_ahb2ensetr.modify(|_, w|
                w.dma1en().set_bit().dma2en().set_bit().dmamuxen().set_bit());
        }
    }
}

/// DMA stream configuration.
#[derive(Debug, Clone, Copy)]
pub struct DmaStreamConfig {
    /// Request input.
    pub request_input: DmaRequestInput,
    /// Transfer direction.
    pub transfer_direction: TransferDirection,
    /// Memory data size.
    pub memory_data_size: DataSize,
    /// Peripheral data size.
    pub peripheral_data_size: DataSize,
    /// Circular mode.
    pub circular: bool,
    /// Memory increment mode.
    pub memory_increment: bool,
    /// Peripheral increment mode.
    pub peripheral_increment: bool,
    /// Peripheral increment fixed to 4 (32-bit alignment).
    pub peripheral_increment_fixed: bool,
    /// Peripheral is flow controller.
    pub peripheral_flow_controller: bool,
    /// Transfer complete interrupt enable.
    pub transfer_complete_interrupt: bool,
    /// Half-transfer interrupt enable.
    pub half_transfer_interrupt: bool,
    /// Transfer_error_interrupt.
    pub transfer_error_interrupt: bool,
    /// Direct mode error interrupt enable.
    pub direct_mode_error_interrupt: bool,
    /// Double-buffer mode.
    pub double_buffer: bool,
    /// Priority level.
    pub priority_level: PriorityLevel,
    /// Memory burst transfer configuration.
    pub memory_burst_transfer: BurstTransfer,
    /// Peripheral burst transfer configuration.
    pub peripheral_burst_transfer: BurstTransfer,
    /// Bufferable transfers enable. Must be enabled for UART/USART transfers.
    pub bufferable_transfers: bool,
    /// Current target for double-buffer mode.
    pub current_target: CurrentTarget,
}

impl Default for DmaStreamConfig {
    fn default() -> Self {
        Self {
            request_input: DmaRequestInput::MemoryToMemory,
            transfer_direction: TransferDirection::PeripheralToMemory,
            memory_data_size: DataSize::Byte,
            peripheral_data_size: DataSize::Byte,
            circular: false,
            memory_increment: false,
            peripheral_increment: false,
            peripheral_increment_fixed: false,
            peripheral_flow_controller: false,
            transfer_complete_interrupt: false,
            half_transfer_interrupt: false,
            transfer_error_interrupt: false,
            direct_mode_error_interrupt: false,
            double_buffer: false,
            priority_level: PriorityLevel::Low,
            memory_burst_transfer: BurstTransfer::Single,
            peripheral_burst_transfer: BurstTransfer::Single,
            bufferable_transfers: false,
            current_target: CurrentTarget::Memory0,
        }
    }
}

/// Data transfer direction.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TransferDirection {
    /// Peripheral-to-memory.
    PeripheralToMemory = 0b00,
    /// Memory-to-peripheral.
    MemoryToPeripheral = 0b01,
    /// Memory-to-memory.
    MemoryToMemory = 0b10,
}

impl From<TransferDirection> for u8 {
    fn from(value: TransferDirection) -> Self {
        value as u8
    }
}

/// Data size.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum DataSize {
    /// Byte, 8-bit.
    Byte = 0b00,
    /// Half-word, 16-bit.
    HalfWord = 0b01,
    /// Word, 32-bit.
    Word = 0b10,
}

impl From<DataSize> for u8 {
    fn from(value: DataSize) -> Self {
        value as u8
    }
}

/// Priority level.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PriorityLevel {
    /// Low.
    Low = 0b00,
    /// Medium.
    Medium = 0b01,
    /// High.
    High = 0b10,
    /// Very High.
    VeryHigh = 0b11,
}

impl From<PriorityLevel> for u8 {
    fn from(value: PriorityLevel) -> Self {
        value as u8
    }
}

/// Burst transfer configuration
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum BurstTransfer {
    /// Single transfer.
    Single = 0b00,
    /// Incremental burst of 4 beats.
    Incremental4 = 0b01,
    /// Incremental burst of 8 beats.
    Incremental8 = 0b10,
    /// Incremental burst of 16 beats.
    Incremental16 = 0b11,
}

impl From<BurstTransfer> for u8 {
    fn from(value: BurstTransfer) -> Self {
        value as u8
    }
}

/// Current target for double-buffer mode.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum CurrentTarget {
    /// Memory 0.
    Memory0 = 0b0,
    /// Memory 1.
    Memory1 = 0b1,
}

impl From<CurrentTarget> for bool {
    fn from(value: CurrentTarget) -> Self {
        value != CurrentTarget::Memory0
    }
}

/// DMA streams.
#[derive(Debug, Clone, Copy)]
pub enum DmaStream {
    /// DMA1 stream 0.
    Dma1Stream0,
    /// DMA1 stream 1.
    Dma1Stream1,
    /// DMA1 stream 2.
    Dma1Stream2,
    /// DMA1 stream 3.
    Dma1Stream3,
    /// DMA1 stream 4.
    Dma1Stream4,
    /// DMA1 stream 5.
    Dma1Stream5,
    /// DMA1 stream 6.
    Dma1Stream6,
    /// DMA1 stream 7.
    Dma1Stream7,

    /// DMA2 stream 0.
    Dma2Stream0,
    /// DMA2 stream 1.
    Dma2Stream1,
    /// DMA2 stream 2.
    Dma2Stream2,
    /// DMA2 stream 3.
    Dma2Stream3,
    /// DMA2 stream 4.
    Dma2Stream4,
    /// DMA2 stream 5.
    Dma2Stream5,
    /// DMA2 stream 6.
    Dma2Stream6,
    /// DMA2 stream 7.
    Dma2Stream7,
}

macro_rules! dma_stream_configure {
    ($dma: ident, $dma_cr: ident, $dmamux:ident, $dmamux_cr: ident, $config: ident) => {
        unsafe {
            let regs = &(*pac::$dma::ptr());
            regs.$dma_cr.modify(|_, w| {
                w.dir()
                    .bits($config.transfer_direction.into())
                    .msize()
                    .bits($config.memory_data_size.into())
                    .psize()
                    .bits($config.peripheral_data_size.into())
                    .circ()
                    .bit($config.circular)
                    .minc()
                    .bit($config.memory_increment)
                    .pinc()
                    .bit($config.peripheral_increment)
                    .pincos()
                    .bit($config.peripheral_increment_fixed)
                    .pfctrl()
                    .bit($config.peripheral_flow_controller)
                    .tcie()
                    .bit($config.transfer_complete_interrupt)
                    .htie()
                    .bit($config.half_transfer_interrupt)
                    .teie()
                    .bit($config.transfer_error_interrupt)
                    .dmeie()
                    .bit($config.direct_mode_error_interrupt)
                    .dbm()
                    .bit($config.double_buffer)
                    .pl()
                    .bits($config.priority_level.into())
                    .mburst()
                    .bits($config.memory_burst_transfer.into())
                    .pburst()
                    .bits($config.peripheral_burst_transfer.into())
                    .ct()
                    .bit($config.current_target.into())
            });

            // TRBUFF bit is missing in PAC, so handle it manually.
            if $config.bufferable_transfers {
                regs.$dma_cr.modify(|r, w| w.bits(r.bits() | (1 << 20)));
            } else {
                regs.$dma_cr.modify(|r, w| w.bits(r.bits() & !(1 << 20)));
            }

            let regs = &(*pac::$dmamux::ptr());
            regs.$dmamux_cr
                .modify(|_, w| w.dmareq_id().bits($config.request_input.into()));
        }
    };
}

macro_rules! dma_stream_enable {
    ($dma: ident, $dma_cr: ident, $state:expr) => {
        unsafe {
            let regs = &(*pac::$dma::ptr());
            regs.$dma_cr.modify(|_, w| w.en().bit($state));
        }
    };
}

impl DmaStream {
    /// Initializes the stream with a configuration.
    pub fn init(&self, config: DmaStreamConfig) {
        match self {
            DmaStream::Dma1Stream0 => {
                dma_stream_configure!(DMA1, dma_s0cr, DMAMUX1, dmamux_c0cr, config);
            }
            DmaStream::Dma1Stream1 => {
                dma_stream_configure!(DMA1, dma_s1cr, DMAMUX1, dmamux_c1cr, config);
            }
            DmaStream::Dma1Stream2 => {
                dma_stream_configure!(DMA1, dma_s2cr, DMAMUX1, dmamux_c2cr, config);
            }
            DmaStream::Dma1Stream3 => {
                dma_stream_configure!(DMA1, dma_s3cr, DMAMUX1, dmamux_c3cr, config);
            }
            DmaStream::Dma1Stream4 => {
                dma_stream_configure!(DMA1, dma_s4cr, DMAMUX1, dmamux_c4cr, config);
            }
            DmaStream::Dma1Stream5 => {
                dma_stream_configure!(DMA1, dma_s5cr, DMAMUX1, dmamux_c5cr, config);
            }
            DmaStream::Dma1Stream6 => {
                dma_stream_configure!(DMA1, dma_s6cr, DMAMUX1, dmamux_c6cr, config);
            }
            DmaStream::Dma1Stream7 => {
                dma_stream_configure!(DMA1, dma_s7cr, DMAMUX1, dmamux_c7cr, config);
            }

            DmaStream::Dma2Stream0 => {
                dma_stream_configure!(DMA2, dma_s0cr, DMAMUX1, dmamux_c8cr, config);
            }
            DmaStream::Dma2Stream1 => {
                dma_stream_configure!(DMA2, dma_s1cr, DMAMUX1, dmamux_c9cr, config);
            }
            DmaStream::Dma2Stream2 => {
                dma_stream_configure!(DMA2, dma_s2cr, DMAMUX1, dmamux_c10cr, config);
            }
            DmaStream::Dma2Stream3 => {
                dma_stream_configure!(DMA2, dma_s3cr, DMAMUX1, dmamux_c11cr, config);
            }
            DmaStream::Dma2Stream4 => {
                dma_stream_configure!(DMA2, dma_s4cr, DMAMUX1, dmamux_c12cr, config);
            }
            DmaStream::Dma2Stream5 => {
                dma_stream_configure!(DMA2, dma_s5cr, DMAMUX1, dmamux_c13cr, config);
            }
            DmaStream::Dma2Stream6 => {
                dma_stream_configure!(DMA2, dma_s6cr, DMAMUX1, dmamux_c14cr, config);
            }
            DmaStream::Dma2Stream7 => {
                dma_stream_configure!(DMA2, dma_s7cr, DMAMUX1, dmamux_c15cr, config);
            }
        }
    }

    /// Starts the transfer.
    pub fn start_transfer(
        &self,
        memory_address: impl Into<u32>,
        peripheral_address: impl Into<u32>,
        length: usize,
    ) {
        let length = length as u32;
        unsafe {
            let dma1 = &(*pac::DMA1::ptr());
            let dma2 = &(*pac::DMA2::ptr());
            match self {
                DmaStream::Dma1Stream0 => {
                    dma1.dma_s0m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s0par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s0ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream1 => {
                    dma1.dma_s1m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s1par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s1ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream2 => {
                    dma1.dma_s2m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s2par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s2ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream3 => {
                    dma1.dma_s3m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s3par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s3ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream4 => {
                    dma1.dma_s4m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s4par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s4ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream5 => {
                    dma1.dma_s5m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s5par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s5ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream6 => {
                    dma1.dma_s6m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s6par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s6ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma1Stream7 => {
                    dma1.dma_s7m0ar.write(|w| w.bits(memory_address.into()));
                    dma1.dma_s7par.write(|w| w.bits(peripheral_address.into()));
                    dma1.dma_s7ndtr.write(|w| w.bits(length));
                }

                DmaStream::Dma2Stream0 => {
                    dma2.dma_s0m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s0par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s0ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream1 => {
                    dma2.dma_s1m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s1par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s1ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream2 => {
                    dma2.dma_s2m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s2par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s2ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream3 => {
                    dma2.dma_s3m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s3par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s3ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream4 => {
                    dma2.dma_s4m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s4par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s4ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream5 => {
                    dma2.dma_s5m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s5par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s5ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream6 => {
                    dma2.dma_s6m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s6par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s6ndtr.write(|w| w.bits(length));
                }
                DmaStream::Dma2Stream7 => {
                    dma2.dma_s7m0ar.write(|w| w.bits(memory_address.into()));
                    dma2.dma_s7par.write(|w| w.bits(peripheral_address.into()));
                    dma2.dma_s7ndtr.write(|w| w.bits(length));
                }
            }
        }

        self.enable();
    }

    /// Stops the transfer. Similar to `disable`.
    pub fn stop_transfer(&self) {
        self.disable();
    }

    /// Enables the stream.
    pub fn enable(&self) {
        self.clear_all_flags();

        match self {
            DmaStream::Dma1Stream0 => {
                dma_stream_enable!(DMA1, dma_s0cr, true);
            }
            DmaStream::Dma1Stream1 => {
                dma_stream_enable!(DMA1, dma_s1cr, true);
            }
            DmaStream::Dma1Stream2 => {
                dma_stream_enable!(DMA1, dma_s2cr, true);
            }
            DmaStream::Dma1Stream3 => {
                dma_stream_enable!(DMA1, dma_s3cr, true);
            }
            DmaStream::Dma1Stream4 => {
                dma_stream_enable!(DMA1, dma_s4cr, true);
            }
            DmaStream::Dma1Stream5 => {
                dma_stream_enable!(DMA1, dma_s5cr, true);
            }
            DmaStream::Dma1Stream6 => {
                dma_stream_enable!(DMA1, dma_s6cr, true);
            }
            DmaStream::Dma1Stream7 => {
                dma_stream_enable!(DMA1, dma_s7cr, true);
            }

            DmaStream::Dma2Stream0 => {
                dma_stream_enable!(DMA2, dma_s0cr, true);
            }
            DmaStream::Dma2Stream1 => {
                dma_stream_enable!(DMA2, dma_s1cr, true);
            }
            DmaStream::Dma2Stream2 => {
                dma_stream_enable!(DMA2, dma_s2cr, true);
            }
            DmaStream::Dma2Stream3 => {
                dma_stream_enable!(DMA2, dma_s3cr, true);
            }
            DmaStream::Dma2Stream4 => {
                dma_stream_enable!(DMA2, dma_s4cr, true);
            }
            DmaStream::Dma2Stream5 => {
                dma_stream_enable!(DMA2, dma_s5cr, true);
            }
            DmaStream::Dma2Stream6 => {
                dma_stream_enable!(DMA2, dma_s6cr, true);
            }
            DmaStream::Dma2Stream7 => {
                dma_stream_enable!(DMA2, dma_s7cr, true);
            }
        }
    }

    /// Disables the stream.
    pub fn disable(&self) {
        match self {
            DmaStream::Dma1Stream0 => {
                dma_stream_enable!(DMA1, dma_s0cr, false);
            }
            DmaStream::Dma1Stream1 => {
                dma_stream_enable!(DMA1, dma_s1cr, false);
            }
            DmaStream::Dma1Stream2 => {
                dma_stream_enable!(DMA1, dma_s2cr, false);
            }
            DmaStream::Dma1Stream3 => {
                dma_stream_enable!(DMA1, dma_s3cr, false);
            }
            DmaStream::Dma1Stream4 => {
                dma_stream_enable!(DMA1, dma_s4cr, false);
            }
            DmaStream::Dma1Stream5 => {
                dma_stream_enable!(DMA1, dma_s5cr, false);
            }
            DmaStream::Dma1Stream6 => {
                dma_stream_enable!(DMA1, dma_s6cr, false);
            }
            DmaStream::Dma1Stream7 => {
                dma_stream_enable!(DMA1, dma_s7cr, false);
            }

            DmaStream::Dma2Stream0 => {
                dma_stream_enable!(DMA2, dma_s0cr, false);
            }
            DmaStream::Dma2Stream1 => {
                dma_stream_enable!(DMA2, dma_s1cr, false);
            }
            DmaStream::Dma2Stream2 => {
                dma_stream_enable!(DMA2, dma_s2cr, false);
            }
            DmaStream::Dma2Stream3 => {
                dma_stream_enable!(DMA2, dma_s3cr, false);
            }
            DmaStream::Dma2Stream4 => {
                dma_stream_enable!(DMA2, dma_s4cr, false);
            }
            DmaStream::Dma2Stream5 => {
                dma_stream_enable!(DMA2, dma_s5cr, false);
            }
            DmaStream::Dma2Stream6 => {
                dma_stream_enable!(DMA2, dma_s6cr, false);
            }
            DmaStream::Dma2Stream7 => {
                dma_stream_enable!(DMA2, dma_s7cr, false);
            }
        }
    }

    /// Returns the transfer complete flag.
    pub fn is_transfer_complete(&self) -> bool {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lisr.read().tcif0().bit(),
            DmaStream::Dma1Stream1 => dma1.dma_lisr.read().tcif1().bit(),
            DmaStream::Dma1Stream2 => dma1.dma_lisr.read().tcif2().bit(),
            DmaStream::Dma1Stream3 => dma1.dma_lisr.read().tcif3().bit(),
            DmaStream::Dma1Stream4 => dma1.dma_hisr.read().tcif4().bit(),
            DmaStream::Dma1Stream5 => dma1.dma_hisr.read().tcif5().bit(),
            DmaStream::Dma1Stream6 => dma1.dma_hisr.read().tcif6().bit(),
            DmaStream::Dma1Stream7 => dma1.dma_hisr.read().tcif7().bit(),

            DmaStream::Dma2Stream0 => dma2.dma_lisr.read().tcif0().bit(),
            DmaStream::Dma2Stream1 => dma2.dma_lisr.read().tcif1().bit(),
            DmaStream::Dma2Stream2 => dma2.dma_lisr.read().tcif2().bit(),
            DmaStream::Dma2Stream3 => dma2.dma_lisr.read().tcif3().bit(),
            DmaStream::Dma2Stream4 => dma2.dma_hisr.read().tcif4().bit(),
            DmaStream::Dma2Stream5 => dma2.dma_hisr.read().tcif5().bit(),
            DmaStream::Dma2Stream6 => dma2.dma_hisr.read().tcif6().bit(),
            DmaStream::Dma2Stream7 => dma2.dma_hisr.read().tcif7().bit(),
        }
    }

    /// Returns the half-transfer flag.
    pub fn is_half_transfer(&self) -> bool {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lisr.read().htif0().bit(),
            DmaStream::Dma1Stream1 => dma1.dma_lisr.read().htif1().bit(),
            DmaStream::Dma1Stream2 => dma1.dma_lisr.read().htif2().bit(),
            DmaStream::Dma1Stream3 => dma1.dma_lisr.read().htif3().bit(),
            DmaStream::Dma1Stream4 => dma1.dma_hisr.read().htif4().bit(),
            DmaStream::Dma1Stream5 => dma1.dma_hisr.read().htif5().bit(),
            DmaStream::Dma1Stream6 => dma1.dma_hisr.read().htif6().bit(),
            DmaStream::Dma1Stream7 => dma1.dma_hisr.read().htif7().bit(),

            DmaStream::Dma2Stream0 => dma2.dma_lisr.read().htif0().bit(),
            DmaStream::Dma2Stream1 => dma2.dma_lisr.read().htif1().bit(),
            DmaStream::Dma2Stream2 => dma2.dma_lisr.read().htif2().bit(),
            DmaStream::Dma2Stream3 => dma2.dma_lisr.read().htif3().bit(),
            DmaStream::Dma2Stream4 => dma2.dma_hisr.read().htif4().bit(),
            DmaStream::Dma2Stream5 => dma2.dma_hisr.read().htif5().bit(),
            DmaStream::Dma2Stream6 => dma2.dma_hisr.read().htif6().bit(),
            DmaStream::Dma2Stream7 => dma2.dma_hisr.read().htif7().bit(),
        }
    }

    /// Returns the transfer error flag.
    pub fn is_transfer_error(&self) -> bool {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lisr.read().teif0().bit(),
            DmaStream::Dma1Stream1 => dma1.dma_lisr.read().teif1().bit(),
            DmaStream::Dma1Stream2 => dma1.dma_lisr.read().teif2().bit(),
            DmaStream::Dma1Stream3 => dma1.dma_lisr.read().teif3().bit(),
            DmaStream::Dma1Stream4 => dma1.dma_hisr.read().teif4().bit(),
            DmaStream::Dma1Stream5 => dma1.dma_hisr.read().teif5().bit(),
            DmaStream::Dma1Stream6 => dma1.dma_hisr.read().teif6().bit(),
            DmaStream::Dma1Stream7 => dma1.dma_hisr.read().teif7().bit(),

            DmaStream::Dma2Stream0 => dma2.dma_lisr.read().teif0().bit(),
            DmaStream::Dma2Stream1 => dma2.dma_lisr.read().teif1().bit(),
            DmaStream::Dma2Stream2 => dma2.dma_lisr.read().teif2().bit(),
            DmaStream::Dma2Stream3 => dma2.dma_lisr.read().teif3().bit(),
            DmaStream::Dma2Stream4 => dma2.dma_hisr.read().teif4().bit(),
            DmaStream::Dma2Stream5 => dma2.dma_hisr.read().teif5().bit(),
            DmaStream::Dma2Stream6 => dma2.dma_hisr.read().teif6().bit(),
            DmaStream::Dma2Stream7 => dma2.dma_hisr.read().teif7().bit(),
        }
    }

    /// Returns the FIFO error flag.
    pub fn is_fifo_error(&self) -> bool {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lisr.read().feif0().bit(),
            DmaStream::Dma1Stream1 => dma1.dma_lisr.read().feif1().bit(),
            DmaStream::Dma1Stream2 => dma1.dma_lisr.read().feif2().bit(),
            DmaStream::Dma1Stream3 => dma1.dma_lisr.read().feif3().bit(),
            DmaStream::Dma1Stream4 => dma1.dma_hisr.read().feif4().bit(),
            DmaStream::Dma1Stream5 => dma1.dma_hisr.read().feif5().bit(),
            DmaStream::Dma1Stream6 => dma1.dma_hisr.read().feif6().bit(),
            DmaStream::Dma1Stream7 => dma1.dma_hisr.read().feif7().bit(),

            DmaStream::Dma2Stream0 => dma2.dma_lisr.read().feif0().bit(),
            DmaStream::Dma2Stream1 => dma2.dma_lisr.read().feif1().bit(),
            DmaStream::Dma2Stream2 => dma2.dma_lisr.read().feif2().bit(),
            DmaStream::Dma2Stream3 => dma2.dma_lisr.read().feif3().bit(),
            DmaStream::Dma2Stream4 => dma2.dma_hisr.read().feif4().bit(),
            DmaStream::Dma2Stream5 => dma2.dma_hisr.read().feif5().bit(),
            DmaStream::Dma2Stream6 => dma2.dma_hisr.read().feif6().bit(),
            DmaStream::Dma2Stream7 => dma2.dma_hisr.read().feif7().bit(),
        }
    }

    /// Returns the direct mode error flag.
    pub fn is_direct_mode_error(&self) -> bool {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lisr.read().dmeif0().bit(),
            DmaStream::Dma1Stream1 => dma1.dma_lisr.read().dmeif1().bit(),
            DmaStream::Dma1Stream2 => dma1.dma_lisr.read().dmeif2().bit(),
            DmaStream::Dma1Stream3 => dma1.dma_lisr.read().dmeif3().bit(),
            DmaStream::Dma1Stream4 => dma1.dma_hisr.read().dmeif4().bit(),
            DmaStream::Dma1Stream5 => dma1.dma_hisr.read().dmeif5().bit(),
            DmaStream::Dma1Stream6 => dma1.dma_hisr.read().dmeif6().bit(),
            DmaStream::Dma1Stream7 => dma1.dma_hisr.read().dmeif7().bit(),

            DmaStream::Dma2Stream0 => dma2.dma_lisr.read().dmeif0().bit(),
            DmaStream::Dma2Stream1 => dma2.dma_lisr.read().dmeif1().bit(),
            DmaStream::Dma2Stream2 => dma2.dma_lisr.read().dmeif2().bit(),
            DmaStream::Dma2Stream3 => dma2.dma_lisr.read().dmeif3().bit(),
            DmaStream::Dma2Stream4 => dma2.dma_hisr.read().dmeif4().bit(),
            DmaStream::Dma2Stream5 => dma2.dma_hisr.read().dmeif5().bit(),
            DmaStream::Dma2Stream6 => dma2.dma_hisr.read().dmeif6().bit(),
            DmaStream::Dma2Stream7 => dma2.dma_hisr.read().dmeif7().bit(),
        }
    }

    /// Clears all flags.
    pub fn clear_all_flags(&self) {
        self.clear_transfer_complete();
        self.clear_half_transfer();
        self.clear_transfer_error();
        self.clear_fifo_error();
        self.clear_direct_mode_error();
    }

    /// Clears the transfer compete error flag.
    pub fn clear_transfer_complete(&self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lifcr.write(|w| w.ctcif0().set_bit()),
            DmaStream::Dma1Stream1 => dma1.dma_lifcr.write(|w| w.ctcif1().set_bit()),
            DmaStream::Dma1Stream2 => dma1.dma_lifcr.write(|w| w.ctcif2().set_bit()),
            DmaStream::Dma1Stream3 => dma1.dma_lifcr.write(|w| w.ctcif3().set_bit()),
            DmaStream::Dma1Stream4 => dma1.dma_hifcr.write(|w| w.ctcif4().set_bit()),
            DmaStream::Dma1Stream5 => dma1.dma_hifcr.write(|w| w.ctcif5().set_bit()),
            DmaStream::Dma1Stream6 => dma1.dma_hifcr.write(|w| w.ctcif6().set_bit()),
            DmaStream::Dma1Stream7 => dma1.dma_hifcr.write(|w| w.ctcif7().set_bit()),

            DmaStream::Dma2Stream1 => dma2.dma_lifcr.write(|w| w.ctcif0().set_bit()),
            DmaStream::Dma2Stream0 => dma2.dma_lifcr.write(|w| w.ctcif1().set_bit()),
            DmaStream::Dma2Stream2 => dma2.dma_lifcr.write(|w| w.ctcif2().set_bit()),
            DmaStream::Dma2Stream3 => dma2.dma_lifcr.write(|w| w.ctcif3().set_bit()),
            DmaStream::Dma2Stream4 => dma2.dma_hifcr.write(|w| w.ctcif4().set_bit()),
            DmaStream::Dma2Stream5 => dma2.dma_hifcr.write(|w| w.ctcif5().set_bit()),
            DmaStream::Dma2Stream6 => dma2.dma_hifcr.write(|w| w.ctcif6().set_bit()),
            DmaStream::Dma2Stream7 => dma2.dma_hifcr.write(|w| w.ctcif7().set_bit()),
        }
    }

    /// Clears the half transfer flag.
    pub fn clear_half_transfer(&self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lifcr.write(|w| w.chtif0().set_bit()),
            DmaStream::Dma1Stream1 => dma1.dma_lifcr.write(|w| w.chtif1().set_bit()),
            DmaStream::Dma1Stream2 => dma1.dma_lifcr.write(|w| w.chtif2().set_bit()),
            DmaStream::Dma1Stream3 => dma1.dma_lifcr.write(|w| w.chtif3().set_bit()),
            DmaStream::Dma1Stream4 => dma1.dma_hifcr.write(|w| w.chtif4().set_bit()),
            DmaStream::Dma1Stream5 => dma1.dma_hifcr.write(|w| w.chtif5().set_bit()),
            DmaStream::Dma1Stream6 => dma1.dma_hifcr.write(|w| w.chtif6().set_bit()),
            DmaStream::Dma1Stream7 => dma1.dma_hifcr.write(|w| w.chtif7().set_bit()),

            DmaStream::Dma2Stream1 => dma2.dma_lifcr.write(|w| w.chtif0().set_bit()),
            DmaStream::Dma2Stream0 => dma2.dma_lifcr.write(|w| w.chtif1().set_bit()),
            DmaStream::Dma2Stream2 => dma2.dma_lifcr.write(|w| w.chtif2().set_bit()),
            DmaStream::Dma2Stream3 => dma2.dma_lifcr.write(|w| w.chtif3().set_bit()),
            DmaStream::Dma2Stream4 => dma2.dma_hifcr.write(|w| w.chtif4().set_bit()),
            DmaStream::Dma2Stream5 => dma2.dma_hifcr.write(|w| w.chtif5().set_bit()),
            DmaStream::Dma2Stream6 => dma2.dma_hifcr.write(|w| w.chtif6().set_bit()),
            DmaStream::Dma2Stream7 => dma2.dma_hifcr.write(|w| w.chtif7().set_bit()),
        }
    }

    /// Clears the transfer error flag.
    pub fn clear_transfer_error(&self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lifcr.write(|w| w.cteif0().set_bit()),
            DmaStream::Dma1Stream1 => dma1.dma_lifcr.write(|w| w.cteif1().set_bit()),
            DmaStream::Dma1Stream2 => dma1.dma_lifcr.write(|w| w.cteif2().set_bit()),
            DmaStream::Dma1Stream3 => dma1.dma_lifcr.write(|w| w.cteif3().set_bit()),
            DmaStream::Dma1Stream4 => dma1.dma_hifcr.write(|w| w.cteif4().set_bit()),
            DmaStream::Dma1Stream5 => dma1.dma_hifcr.write(|w| w.cteif5().set_bit()),
            DmaStream::Dma1Stream6 => dma1.dma_hifcr.write(|w| w.cteif6().set_bit()),
            DmaStream::Dma1Stream7 => dma1.dma_hifcr.write(|w| w.cteif7().set_bit()),

            DmaStream::Dma2Stream1 => dma2.dma_lifcr.write(|w| w.cteif0().set_bit()),
            DmaStream::Dma2Stream0 => dma2.dma_lifcr.write(|w| w.cteif1().set_bit()),
            DmaStream::Dma2Stream2 => dma2.dma_lifcr.write(|w| w.cteif2().set_bit()),
            DmaStream::Dma2Stream3 => dma2.dma_lifcr.write(|w| w.cteif3().set_bit()),
            DmaStream::Dma2Stream4 => dma2.dma_hifcr.write(|w| w.cteif4().set_bit()),
            DmaStream::Dma2Stream5 => dma2.dma_hifcr.write(|w| w.cteif5().set_bit()),
            DmaStream::Dma2Stream6 => dma2.dma_hifcr.write(|w| w.cteif6().set_bit()),
            DmaStream::Dma2Stream7 => dma2.dma_hifcr.write(|w| w.cteif7().set_bit()),
        }
    }

    /// Clears the FIFO error flag.
    pub fn clear_fifo_error(&self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lifcr.write(|w| w.cfeif0().set_bit()),
            DmaStream::Dma1Stream1 => dma1.dma_lifcr.write(|w| w.cfeif1().set_bit()),
            DmaStream::Dma1Stream2 => dma1.dma_lifcr.write(|w| w.cfeif2().set_bit()),
            DmaStream::Dma1Stream3 => dma1.dma_lifcr.write(|w| w.cfeif3().set_bit()),
            DmaStream::Dma1Stream4 => dma1.dma_hifcr.write(|w| w.cfeif4().set_bit()),
            DmaStream::Dma1Stream5 => dma1.dma_hifcr.write(|w| w.cfeif5().set_bit()),
            DmaStream::Dma1Stream6 => dma1.dma_hifcr.write(|w| w.cfeif6().set_bit()),
            DmaStream::Dma1Stream7 => dma1.dma_hifcr.write(|w| w.cfeif7().set_bit()),

            DmaStream::Dma2Stream1 => dma2.dma_lifcr.write(|w| w.cfeif0().set_bit()),
            DmaStream::Dma2Stream0 => dma2.dma_lifcr.write(|w| w.cfeif1().set_bit()),
            DmaStream::Dma2Stream2 => dma2.dma_lifcr.write(|w| w.cfeif2().set_bit()),
            DmaStream::Dma2Stream3 => dma2.dma_lifcr.write(|w| w.cfeif3().set_bit()),
            DmaStream::Dma2Stream4 => dma2.dma_hifcr.write(|w| w.cfeif4().set_bit()),
            DmaStream::Dma2Stream5 => dma2.dma_hifcr.write(|w| w.cfeif5().set_bit()),
            DmaStream::Dma2Stream6 => dma2.dma_hifcr.write(|w| w.cfeif6().set_bit()),
            DmaStream::Dma2Stream7 => dma2.dma_hifcr.write(|w| w.cfeif7().set_bit()),
        }
    }

    /// Clears the direct_mode error flag.
    pub fn clear_direct_mode_error(&self) {
        let dma1 = unsafe { &(*pac::DMA1::ptr()) };
        let dma2 = unsafe { &(*pac::DMA2::ptr()) };
        match self {
            DmaStream::Dma1Stream0 => dma1.dma_lifcr.write(|w| w.cdmeif0().set_bit()),
            DmaStream::Dma1Stream1 => dma1.dma_lifcr.write(|w| w.cdmeif1().set_bit()),
            DmaStream::Dma1Stream2 => dma1.dma_lifcr.write(|w| w.cdmeif2().set_bit()),
            DmaStream::Dma1Stream3 => dma1.dma_lifcr.write(|w| w.cdmeif3().set_bit()),
            DmaStream::Dma1Stream4 => dma1.dma_hifcr.write(|w| w.cdmeif4().set_bit()),
            DmaStream::Dma1Stream5 => dma1.dma_hifcr.write(|w| w.cdmeif5().set_bit()),
            DmaStream::Dma1Stream6 => dma1.dma_hifcr.write(|w| w.cdmeif6().set_bit()),
            DmaStream::Dma1Stream7 => dma1.dma_hifcr.write(|w| w.cdmeif7().set_bit()),

            DmaStream::Dma2Stream1 => dma2.dma_lifcr.write(|w| w.cdmeif0().set_bit()),
            DmaStream::Dma2Stream0 => dma2.dma_lifcr.write(|w| w.cdmeif1().set_bit()),
            DmaStream::Dma2Stream2 => dma2.dma_lifcr.write(|w| w.cdmeif2().set_bit()),
            DmaStream::Dma2Stream3 => dma2.dma_lifcr.write(|w| w.cdmeif3().set_bit()),
            DmaStream::Dma2Stream4 => dma2.dma_hifcr.write(|w| w.cdmeif4().set_bit()),
            DmaStream::Dma2Stream5 => dma2.dma_hifcr.write(|w| w.cdmeif5().set_bit()),
            DmaStream::Dma2Stream6 => dma2.dma_hifcr.write(|w| w.cdmeif6().set_bit()),
            DmaStream::Dma2Stream7 => dma2.dma_hifcr.write(|w| w.cdmeif7().set_bit()),
        }
    }
}
