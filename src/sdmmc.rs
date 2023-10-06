//! Secure digital input/output MultiMediaCard interface.

use core::marker::PhantomData;
use core::ops::Deref;

use cfg_if::cfg_if;

use crate::bitworker::BitWorker;
use crate::pac;
use crate::rcc;
use crate::time::Instant;
use pac::sdmmc1::RegisterBlock;
use pac::{SDMMC1, SDMMC2, SDMMC3};

/// SDMMC peripheral.
#[derive(Debug, Default)]
pub struct Sdmmc<R>
where
    R: Deref<Target = RegisterBlock>,
{
    /// Card Identification Number.
    cid: Option<[u32; 4]>,

    /// Relative Card Address
    rca: Option<u16>,

    /// Bus width.
    bus_width: BusWidth,

    /// Phantom register block.
    _regs: PhantomData<R>,
}

/// Type alias for SDMMC1.
pub type Sdmmc1 = Sdmmc<SDMMC1>;

/// Type alias for SDMMC2.
pub type Sdmmc2 = Sdmmc<SDMMC2>;

/// Type alias for SDMMC3.
pub type Sdmmc3 = Sdmmc<SDMMC3>;

/// Card initialization timeout in milliseconds.
const CARD_INIT_TIMEOUT: u64 = 1000;

/// Card clock frequency in Hz set after initialization.
const CARD_CLOCK_FREQUENCY: u32 = 25000000;

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct SdmmcConfig {
    /// Bus width.
    pub bus_width: BusWidth,
    /// Clock frequency in Hz while initializing.
    pub init_clock_frequency: u32,
    /// Clock power save (disable when bus is inactive)
    pub clock_power_save: bool,
    /// Hardware flow control.
    pub hardware_flow_control: bool,
    /// Data rate signaling.
    pub data_rate: DataRate,
    /// Enable SDR50, DDR50, SDR104, HS200 bus speed modes.
    pub high_bus_speed: bool,
    /// Data timeout in bus cycles.
    pub data_timeout: u32,
}

impl Default for SdmmcConfig {
    /// Returns the default configuration suitable for SD cards.
    /// - 400kHz init clock frequency.
    /// - 1 bit bus width.
    fn default() -> Self {
        Self {
            bus_width: BusWidth::Bits1,
            init_clock_frequency: 400000,
            clock_power_save: false,
            hardware_flow_control: true,
            data_rate: DataRate::Sdr,
            high_bus_speed: false,
            data_timeout: 5000000,
        }
    }
}

/// Bus width.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum BusWidth {
    /// 1 bit.
    #[default]
    Bits1 = 0b00,
    /// 4 bits.
    Bits4 = 0b01,
    /// 8 bits.
    Bits8 = 0b10,
}

/// Data rate signaling.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum DataRate {
    /// Single data rate.
    #[default]
    Sdr = 0b0,
    /// Double data rate.
    Ddr = 0b1,
}

/// Command response.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandResponse {
    /// No response.
    None = 0b00,
    /// Short response.
    Short = 0b01,
    /// Short response without CRC check.
    ShortNoCrc = 0b10,
    /// Long response.
    Long = 0b11,
}

/// Command configuration.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CommandConfig {
    /// Command index.
    index: u8,
    /// Argument value.
    argument: u32,
    /// Expected response.
    response: CommandResponse,
    /// Treat as data transfer command.
    data_transfer: bool,
    /// Treat as stop transmission command.
    stop_transmission: bool,
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            index: 0,
            argument: 0,
            response: CommandResponse::None,
            data_transfer: false,
            stop_transmission: false,
        }
    }
}

// ----------------------------- Errors -------------------------------

/// Errors
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    /// Initialization timeout exceeded.
    InitTimeout,
    /// Card is not supported, e.g. not V2.
    UnsupportedCard,
    /// Response timeout exceeded.
    ResponseTimeout,
    /// Response CRC failed.
    ResponseCrcFailed,
    /// Data timeout exceeded.
    DataTimeout,
    /// Data CRC failed.
    DataCrcFailed,
    /// Receive overrun.
    ReceiveOverrun,
    /// Transmit underrun.
    TransmitUnderrun,
}

// ------------------------- Implementation ---------------------------

impl<R> Sdmmc<R>
where
    R: Deref<Target = RegisterBlock> + Instance,
{
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self {
            cid: None,
            rca: None,
            bus_width: BusWidth::Bits1,
            _regs: PhantomData,
        }
    }

    /// Returns the register block.
    pub fn registers(&self) -> &'static RegisterBlock {
        R::registers()
    }

    /// Initializes the peripheral.
    pub fn init(&mut self, config: SdmmcConfig) {
        R::enable_clock();

        let regs = R::registers();

        unsafe {
            regs.sdmmc_clkcr.modify(|_, w| {
                w.pwrsav()
                    .bit(config.clock_power_save)
                    .widbus()
                    .bits(config.bus_width as u8)
                    .negedge()
                    .clear_bit()
                    .hwfc_en()
                    .bit(config.hardware_flow_control)
                    .ddr()
                    .bit(config.data_rate == DataRate::Ddr)
                    .busspeed()
                    .bit(config.high_bus_speed)
                    .selclkrx()
                    .bits(0b00)
            });
            regs.sdmmc_cmdr.write(|w| w.bits(0));
            regs.sdmmc_argr.write(|w| w.bits(0));
        }

        self.set_clock_frequency(config.init_clock_frequency);
        self.set_data_timeout(config.data_timeout);
        self.bus_width = config.bus_width;

        self.enable();
    }

    /// Initializes the card.
    pub fn init_card(&mut self) -> Result<(), Error> {
        // Reset via CMD0 - GO_IDLE_STATE
        self.send_command(CommandConfig {
            index: 0,
            ..Default::default()
        });
        while !self.is_command_sent() {}

        // Check supported version via CMD8 - SEND_IF_COND.
        // The argument specifies a check of 2.7-3.6V supply range and a pattern
        // and must be mirrored by the response.
        let argument = (0x01 << 8) | 0xAA;
        self.send_command(CommandConfig {
            index: 8,
            argument,
            response: CommandResponse::Short,
            ..Default::default()
        });
        match self.wait_for_command_response() {
            Ok(_) => {
                let response = self.short_response();
                if response != argument {
                    // Voltage/pattern check failed.
                    return Err(Error::UnsupportedCard);
                }
            }
            Err(_) => {
                // Unknown command, card is not V2.
                return Err(Error::UnsupportedCard);
            }
        }

        let init_start_time = Instant::now();

        loop {
            // Set next command as application-specific via via CMD55 - APP_CMD.
            self.send_command(CommandConfig {
                index: 55,
                response: CommandResponse::Short,
                ..Default::default()
            });
            self.wait_for_command_response()?;

            // Initialize card via ACMD41 - SD_SEND_OP_COND.
            self.send_command(CommandConfig {
                index: 41,
                argument: 0x80100000 | 0x40000000 | 0x01000000,
                response: CommandResponse::ShortNoCrc,
                ..Default::default()
            });
            self.wait_for_command_response()?;
            let ocr = self.short_response();

            if BitWorker::new(ocr).is_set(31) {
                break;
            }

            if init_start_time.is_elapsed_millis(CARD_INIT_TIMEOUT) {
                return Err(Error::InitTimeout);
            }
        }

        // Get card id data via CMD2 - ALL_SEND_CID.
        self.send_command(CommandConfig {
            index: 2,
            response: CommandResponse::Long,
            ..Default::default()
        });
        self.wait_for_command_response()?;
        self.cid = Some(self.long_response());

        // Get new relative address from card via CMD3 - SEND_RELATIVE_ADDR
        self.send_command(CommandConfig {
            index: 3,
            response: CommandResponse::Short,
            ..Default::default()
        });
        self.wait_for_command_response()?;
        self.rca = Some((self.short_response() >> 16) as u16);

        // Select the card via CMD7 - SELECT/DESELECT_CARD
        self.send_command(CommandConfig {
            index: 7,
            argument: (self.rca.unwrap() as u32) << 16,
            response: CommandResponse::Short,
            ..Default::default()
        });
        self.wait_for_command_response()?;

        let init_start_time = Instant::now();

        loop {
            // Get card status via CMD13 - SEND_STATUS
            self.send_command(CommandConfig {
                index: 13,
                argument: (self.rca.unwrap() as u32) << 16,
                response: CommandResponse::Short,
                ..Default::default()
            });
            self.wait_for_command_response()?;

            let response = self.short_response();

            if BitWorker::new(response).subvalue(9, 4) == 4 {
                // Card is now in transfer state.
                break;
            }

            if init_start_time.is_elapsed_millis(CARD_INIT_TIMEOUT) {
                return Err(Error::InitTimeout);
            }
        }

        if self.bus_width == BusWidth::Bits4 {
            // Set next command as application-specific via via CMD55 - APP_CMD.
            self.send_command(CommandConfig {
                index: 55,
                argument: (self.rca.unwrap() as u32) << 16,
                response: CommandResponse::Short,
                ..Default::default()
            });
            self.wait_for_command_response()?;

            // Set 4-bit bus width via ACMD6 - SET_BUS_WIDTH.
            self.send_command(CommandConfig {
                index: 6,
                argument: 0b10,
                response: CommandResponse::Short,
                ..Default::default()
            });
            self.wait_for_command_response()?;
        }

        self.set_clock_frequency(CARD_CLOCK_FREQUENCY);

        Ok(())
    }

    /// Reads a block of 512 bytes from the card.
    pub fn read_block(&mut self, address: u32, buffer: &mut [u8; 512]) -> Result<(), Error> {
        while self.is_busy() {}

        self.clear_all_data_flags();

        let regs = R::registers();

        unsafe {
            regs.sdmmc_dlenr.write(|w| w.datalength().bits(512));
            regs.sdmmc_dctrl
                .write(|w| w.dblocksize().bits(9).dtdir().set_bit());
        }

        self.send_command(CommandConfig {
            index: 17,
            argument: address,
            response: CommandResponse::Short,
            data_transfer: true,
            ..Default::default()
        });
        self.wait_for_command_response()?;

        let mut i = 0;

        while !self.is_data_transfer_end() {
            if self.is_data_timeout() {
                return Err(Error::DataTimeout);
            } else if self.is_data_crc_failed() {
                return Err(Error::DataCrcFailed);
            } else if self.is_receive_overrun_error() {
                return Err(Error::ReceiveOverrun);
            }

            if self.is_receiver_half_full() {
                for _ in 0..8 {
                    let bytes = regs.sdmmc_fifor0.read().bits().to_le_bytes();
                    buffer[i..i + 4].copy_from_slice(&bytes);
                    i += 4;
                }
            }
        }

        Ok(())
    }

    /// Sets the clock frequency in Hz.
    pub fn set_clock_frequency(&mut self, frequency: u32) {
        let clk_div = (R::clock_frequency() as u32 / frequency / 2) as u16;
        unsafe {
            let regs = R::registers();
            regs.sdmmc_clkcr.modify(|_, w| w.clkdiv().bits(clk_div));
        }
    }

    /// Sets the data timeout in bus clock cycles.
    pub fn set_data_timeout(&mut self, timeout: u32) {
        unsafe {
            let regs = R::registers();
            regs.sdmmc_dtimer.write(|w| w.bits(timeout));
        }
    }

    /// Sends a command.
    pub fn send_command(&mut self, config: CommandConfig) {
        while self.is_busy() {}

        self.clear_command_sent();
        self.clear_command_response_received();
        self.clear_command_response_timeout();
        self.clear_command_response_crc_failed();

        unsafe {
            let regs = R::registers();
            regs.sdmmc_argr.write(|w| w.bits(config.argument));
            regs.sdmmc_cmdr.modify(|_, w| {
                w.cmdindex()
                    .bits(config.index)
                    .cmdtrans()
                    .bit(config.data_transfer)
                    .cmdstop()
                    .bit(config.stop_transmission)
                    .waitresp()
                    .bits(config.response as u8)
                    .cpsmen()
                    .set_bit()
            });
        }
    }

    /// Waits for command response, blocking.
    pub fn wait_for_command_response(&self) -> Result<(), Error> {
        while !self.is_command_response_received() {
            if self.is_command_response_timeout() {
                return Err(Error::ResponseTimeout);
            } else if self.is_command_response_crc_failed() {
                return Err(Error::ResponseCrcFailed);
            }
        }

        Ok(())
    }

    /// Returns the short response.
    pub fn short_response(&self) -> u32 {
        let regs = R::registers();
        regs.sdmmc_resp1r.read().bits()
    }

    /// Returns the long response.
    pub fn long_response(&self) -> [u32; 4] {
        let regs = R::registers();
        [
            regs.sdmmc_resp1r.read().bits(),
            regs.sdmmc_resp2r.read().bits(),
            regs.sdmmc_resp3r.read().bits(),
            regs.sdmmc_resp4r.read().bits(),
        ]
    }

    /// Enables the peripheral.
    pub fn enable(&mut self) {
        unsafe {
            let regs = R::registers();
            regs.sdmmc_power.modify(|_, w| w.pwrctrl().bits(0b11));
        }
    }

    /// Returns if the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_power.read().pwrctrl().bits() == 0b11
    }

    /// Returns if the state machine is not idle.
    pub fn is_busy(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().cpsmact().bit_is_set()
            || regs.sdmmc_star.read().dpsmact().bit_is_set()
    }

    /// Returns if the command response CRC check has failed.
    pub fn is_command_response_crc_failed(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().ccrcfail().bit_is_set()
    }

    /// Returns if the data CRC check has failed.
    pub fn is_data_crc_failed(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dcrcfail().bit_is_set()
    }

    /// Returns if a command response timeout has occurred.
    pub fn is_command_response_timeout(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().ctimeout().bit_is_set()
    }

    /// Returns if a data timeout has occurred.
    pub fn is_data_timeout(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dtimeout().bit_is_set()
    }

    /// Returns if a transmit FIFO underrun error has occurred.
    pub fn is_transmit_underrun_error(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().txunderr().bit_is_set()
    }

    /// Returns if a receive FIFO overrun error has occurred.
    pub fn is_receive_overrun_error(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().rxoverr().bit_is_set()
    }

    /// Returns if a command response has been received.
    pub fn is_command_response_received(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().cmdrend().bit_is_set()
    }

    /// Returns if a command has been sent.
    pub fn is_command_sent(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().cmdsent().bit_is_set()
    }

    /// Returns if data transfer has been ended correctly.
    pub fn is_data_transfer_end(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dataend().bit_is_set()
    }

    /// Returns if data transfer is on hold.
    pub fn is_data_transfer_hold(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dhold().bit_is_set()
    }

    /// Returns if a data block has be sent or received.
    pub fn is_data_block_end(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dbckend().bit_is_set()
    }

    /// Returns if data transfer has been aborted.
    pub fn is_data_transfer_aborted(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().dabort().bit_is_set()
    }

    /// Returns if the transmit FIFO is empty.
    pub fn is_transmitter_empty(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().txfifoe().bit_is_set()
    }

    /// Returns if the transmit FIFO is full.
    pub fn is_transmitter_full(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().txfifof().bit_is_set()
    }

    /// Returns if the transmit FIFO is half empty.
    pub fn is_transmitter_half_empty(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().txfifohe().bit_is_set()
    }

    /// Returns if the receive FIFO is empty.
    pub fn is_receiver_empty(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().rxfifoe().bit_is_set()
    }

    /// Returns if the receive FIFO is full.
    pub fn is_receiver_full(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().rxfifof().bit_is_set()
    }

    /// Returns if the receive FIFO is half full.
    pub fn is_receiver_half_full(&self) -> bool {
        let regs = R::registers();
        regs.sdmmc_star.read().rxfifohf().bit_is_set()
    }

    /// Clears a command response CRC error.
    pub fn clear_command_response_crc_failed(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.ccrcfailc().set_bit());
    }

    /// Clears a command response timeout error.
    pub fn clear_command_response_timeout(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.ctimeoutc().set_bit());
    }

    /// Clears the command response received flag.
    pub fn clear_command_response_received(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.cmdrendc().set_bit());
    }

    /// Clears the command sent flag.
    pub fn clear_command_sent(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.cmdsentc().set_bit());
    }

    /// Clears a transmit FIFO underrun error.
    pub fn clear_transmit_underrun_error(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.txunderrc().set_bit());
    }

    /// Clears a receive FIFO overrun error.
    pub fn clear_receive_overrun_error(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.rxoverrc().set_bit());
    }

    /// Clears a data timeout error.
    pub fn clear_data_timeout(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dtimeoutc().set_bit());
    }

    /// Clears a data CRC error.
    pub fn clear_data_crc_failed(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dcrcfailc().set_bit());
    }

    /// Clears the data transfer end flag.
    pub fn clear_data_transfer_end(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dataendc().set_bit());
    }

    /// Clears the data transfer hold flag.
    pub fn clear_data_transfer_hold(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dholdc().set_bit());
    }

    /// Clears the data transfer aborted flag.
    pub fn clear_data_transfer_aborted(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dabortc().set_bit());
    }

    /// Clears the data block end flag.
    pub fn clear_data_block_end(&mut self) {
        let regs = R::registers();
        regs.sdmmc_icr.write(|w| w.dbckendc().set_bit());
    }

    /// Clears all data transfer flags.
    pub fn clear_all_data_flags(&mut self) {
        self.clear_transmit_underrun_error();
        self.clear_receive_overrun_error();
        self.clear_data_timeout();
        self.clear_data_crc_failed();
        self.clear_data_transfer_end();
        self.clear_data_transfer_hold();
        self.clear_data_transfer_aborted();
        self.clear_data_block_end();
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

// ------------------------------ SDMMC1 ------------------------------

impl Instance for SDMMC1 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SDMMC1::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb6ensetr.modify(|_, w| w.sdmmc1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb6ensetr.modify(|_, w| w.sdmmc1en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb6enclrr.modify(|_, w| w.sdmmc1en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb6enclrr.modify(|_, w| w.sdmmc1en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::hsi_frequency() as f32
    }
}

// ------------------------------ SDMMC2 ------------------------------

impl Instance for SDMMC2 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SDMMC2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb6ensetr.modify(|_, w| w.sdmmc2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb6ensetr.modify(|_, w| w.sdmmc2en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb6enclrr.modify(|_, w| w.sdmmc2en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb6enclrr.modify(|_, w| w.sdmmc2en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::hsi_frequency() as f32
    }
}

// ------------------------------ SDMMC3 ------------------------------

impl Instance for SDMMC3 {
    fn registers() -> &'static RegisterBlock {
        unsafe { &(*pac::SDMMC2::ptr()) }
    }

    fn enable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb2ensetr.modify(|_, w| w.sdmmc3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb2ensetr.modify(|_, w| w.sdmmc3en().set_bit());
            }
        }
    }

    fn disable_clock() {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_ahb2enclrr.modify(|_, w| w.sdmmc3en().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_ahb2enclrr.modify(|_, w| w.sdmmc3en().set_bit());
            }
        }
    }

    fn clock_frequency() -> f32 {
        rcc::mcu_frequency()
    }
}
