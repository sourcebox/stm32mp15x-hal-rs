//! LCD-TFT display controller.

use cfg_if::cfg_if;

use crate::pac;
use pac::ltdc::RegisterBlock;

/// LTDC peripheral.
#[derive(Debug, Default)]
pub struct Ltdc {}

// ------------------------- Configuration ---------------------------

/// Configuration settings.
#[derive(Debug, Clone)]
pub struct LtdcConfig {
    /// Active width in pixel clocks.
    pub active_width: u32,
    /// Active height in scan lines.
    pub active_height: u32,
    /// Pixel format.
    pub pixel_format: PixelFormat,
    /// Address of frame buffer containing the pixels.
    pub frame_buffer_address: u32,
    /// Horizontal synchronization width in pixel clocks.
    pub horizontal_synchronization_width: u32,
    /// Accumulated horizontal back porch in pixel clocks.
    pub horizontal_back_porch: u32,
    /// Accumulated horizontal front porch in pixel clocks.
    pub horizontal_front_porch: u32,
    /// Vertical synchronization height in scan lines.
    pub vertical_synchronization_height: u32,
    /// Accumulated vertical back porch in scan lines.
    pub vertical_back_porch: u32,
    /// Accumulated vertical back porch in scan lines.
    pub vertical_front_porch: u32,
    /// Horizontal synchronization polarity.
    pub hsync_polarity: Polarity,
    /// Vertical synchronization polarity.
    pub vsync_polarity: Polarity,
    /// Not data enable polarity.
    pub not_data_enable_polarity: Polarity,
    /// Pixel clock polarity.
    pub pixel_clock_polarity: Polarity,
}

impl Default for LtdcConfig {
    /// Returns the default configuration, ILI9341 240x320 display with RGB565 framebuffer.
    fn default() -> Self {
        Self {
            active_width: 240,
            active_height: 320,
            pixel_format: PixelFormat::Rgb565,
            frame_buffer_address: 0,
            horizontal_synchronization_width: 10,
            horizontal_back_porch: 20,
            horizontal_front_porch: 10,
            vertical_synchronization_height: 2,
            vertical_back_porch: 2,
            vertical_front_porch: 4,
            hsync_polarity: Polarity::ActiveLow,
            vsync_polarity: Polarity::ActiveLow,
            not_data_enable_polarity: Polarity::ActiveLow,
            pixel_clock_polarity: Polarity::ActiveLow,
        }
    }
}

/// Signal polarity when active.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Polarity {
    /// Low.
    ActiveLow = 0b0,
    /// High.
    ActiveHigh = 0b1,
}

/// Pixel format for framebuffer data.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PixelFormat {
    /// ARGB8888 format.
    Argb8888 = 0b000,
    /// RGB888 format.
    Rgb888 = 0b001,
    /// RGB565 format.
    Rgb565 = 0b010,
    /// ARGB1555 format.
    Argb1555 = 0b011,
    /// ARGB4444 format.
    Argb4444 = 0b100,
    /// 8-bit luminance.
    L8 = 0b101,
    /// 4-bit alpha, 4-bit luminance.
    Al44 = 0b110,
    /// 8-bit alpha, 8-bit luminance.
    Al88 = 0b111,
}

// ------------------------- Implementation ---------------------------

impl Ltdc {
    /// Returns the peripheral instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Initializes the peripheral.
    pub fn init(&mut self, config: LtdcConfig) {
        self.enable_clock();

        self.disable();

        let accumulated_horizontal_back_porch =
            config.horizontal_synchronization_width + config.horizontal_back_porch;
        let accumulated_vertical_back_porch =
            config.vertical_synchronization_height + config.vertical_back_porch;
        let accumulated_active_width = accumulated_horizontal_back_porch + config.active_width;
        let accumulated_active_height = accumulated_vertical_back_porch + config.active_height;
        let total_width = accumulated_active_width + config.horizontal_front_porch;
        let total_height = accumulated_active_height + config.vertical_front_porch;
        let window_horizontal_start_position = accumulated_horizontal_back_porch;
        let window_horizontal_stop_position =
            window_horizontal_start_position + config.active_width;
        let window_vertical_start_position = accumulated_vertical_back_porch;
        let window_vertical_stop_position = window_vertical_start_position + config.active_height;

        let bytes_per_pixel = match config.pixel_format {
            PixelFormat::Argb8888 => 4,
            PixelFormat::Rgb888 => 3,
            PixelFormat::Rgb565
            | PixelFormat::Argb1555
            | PixelFormat::Argb4444
            | PixelFormat::Al88 => 2,
            PixelFormat::L8 | PixelFormat::Al44 => 1,
        };

        let regs = self.registers();

        unsafe {
            regs.ltdc_sscr.modify(|_, w| {
                w.hsw()
                    .bits(config.horizontal_synchronization_width as u16 - 1)
                    .vsh()
                    .bits(config.vertical_synchronization_height as u16 - 1)
            });
            regs.ltdc_bpcr.modify(|_, w| {
                w.ahbp()
                    .bits(accumulated_horizontal_back_porch as u16 - 1)
                    .avbp()
                    .bits(accumulated_vertical_back_porch as u16 - 1)
            });
            regs.ltdc_awcr.modify(|_, w| {
                w.aaw()
                    .bits(accumulated_active_width as u16 - 1)
                    .aah()
                    .bits(accumulated_active_height as u16 - 1)
            });
            regs.ltdc_twcr.modify(|_, w| {
                w.totalw()
                    .bits(total_width as u16 - 1)
                    .totalh()
                    .bits(total_height as u16 - 1)
            });
            regs.ltdc_l1whpcr.modify(|_, w| {
                w.whstpos()
                    .bits(window_horizontal_start_position as u16)
                    .whsppos()
                    .bits(window_horizontal_stop_position as u16)
            });
            regs.ltdc_l1wvpcr.modify(|_, w| {
                w.wvstpos()
                    .bits(window_vertical_start_position as u16)
                    .wvsppos()
                    .bits(window_vertical_stop_position as u16)
            });
            regs.ltdc_l1pfcr
                .modify(|_, w| w.pf().bits(config.pixel_format as u8));
            regs.ltdc_l1cfbar
                .write(|w| w.bits(config.frame_buffer_address));
            // TODO: calculate correct values
            // regs.ltdc_l1cfblr
            //     .write(|w| w.bits(config.active_width * config.active_height * bytes_per_pixel));
        }

        regs.ltdc_l1cr.modify(|_, w| w.len().set_bit());

        regs.ltdc_gcr.modify(|_, w| {
            w.vspol()
                .bit(config.vsync_polarity == Polarity::ActiveHigh)
                .hspol()
                .bit(config.hsync_polarity == Polarity::ActiveHigh)
                .depol()
                .bit(config.not_data_enable_polarity == Polarity::ActiveHigh)
                .pcpol()
                .bit(config.pixel_clock_polarity == Polarity::ActiveHigh)
        });

        self.enable();
    }

    /// Deinitializes the peripheral.
    pub fn deinit(&mut self) {
        self.disable();
        self.disable_clock();
    }

    /// Enables the peripheral.
    pub fn enable(&mut self) {
        let regs = self.registers();
        regs.ltdc_gcr.modify(|_, w| w.ltdcen().set_bit());
    }

    /// Disables the peripheral.
    pub fn disable(&mut self) {
        let regs = self.registers();
        regs.ltdc_gcr.modify(|_, w| w.ltdcen().clear_bit());
    }

    /// Returns if the peripheral is enabled.
    pub fn is_enabled(&self) -> bool {
        let regs = self.registers();
        regs.ltdc_gcr.read().ltdcen().bit_is_set()
    }

    /// Returns the register block.
    pub fn registers(&self) -> &'static RegisterBlock {
        unsafe { &(*pac::LTDC::ptr()) }
    }

    /// Enables the clock.
    pub fn enable_clock(&mut self) {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb4ensetr.modify(|_, w| w.ltdcen().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb4ensetr.modify(|_, w| w.ltdcen().set_bit());
            }
        }
    }

    /// Disables the clock.
    pub fn disable_clock(&mut self) {
        cfg_if! {
            if #[cfg(feature = "mpu-ca7")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mp_apb4enclrr.modify(|_, w| w.ltdcen().set_bit());
            } else if #[cfg(feature = "mcu-cm4")] {
                let rcc = unsafe { &(*pac::RCC::ptr()) };
                rcc.rcc_mc_apb4enclrr.modify(|_, w| w.ltdcen().set_bit());
            }
        }
    }
}
