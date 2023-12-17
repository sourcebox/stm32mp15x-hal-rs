//! GPIO ports and pins.

pub use embedded_hal::digital::{
    ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin, ToggleableOutputPin,
};

use crate::bitworker::BitWorker;
use crate::pac;

/// Pin mode.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PinMode {
    /// Input pin.
    Input,
    /// Onput pin.
    Output,
    /// Pin with alternate function.
    Alt(u8),
    /// Analog pin for use with ADC.
    Analog,
}

impl From<PinMode> for u8 {
    fn from(value: PinMode) -> Self {
        match value {
            PinMode::Input => 0b00,
            PinMode::Output => 0b01,
            PinMode::Alt(_) => 0b10,
            PinMode::Analog => 0b11,
        }
    }
}

/// Pin output type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum OutputType {
    /// Push-pull output.
    PushPull,
    /// Open drain output.
    OpenDrain,
}

impl From<OutputType> for u8 {
    fn from(value: OutputType) -> Self {
        match value {
            OutputType::PushPull => 0b0,
            OutputType::OpenDrain => 0b1,
        }
    }
}

/// Pin output speed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum OutputSpeed {
    /// Low speed.
    Low,
    /// Medium speed.
    Medium,
    /// High speed.
    High,
    /// Very high speed.  
    VeryHigh,
}

impl From<OutputSpeed> for u8 {
    fn from(value: OutputSpeed) -> Self {
        match value {
            OutputSpeed::Low => 0b00,
            OutputSpeed::Medium => 0b01,
            OutputSpeed::High => 0b10,
            OutputSpeed::VeryHigh => 0b11,
        }
    }
}

/// Pin pull-up/pull-down configuration.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PullMode {
    /// No pull-up or pull-down, floating.
    Floating,
    /// Pull-up enabled.
    PullUp,
    /// Pull-down enabled.
    PullDown,
}

impl From<PullMode> for u8 {
    fn from(value: PullMode) -> Self {
        match value {
            PullMode::Floating => 0b00,
            PullMode::PullUp => 0b01,
            PullMode::PullDown => 0b10,
        }
    }
}

/// Port letters.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Port {
    /// Port A.
    A,
    /// Port B.
    B,
    /// Port C.
    C,
    /// Port D.
    D,
    /// Port E.
    E,
    /// Port F.
    F,
    /// Port G.
    G,
    /// Port H.
    H,
    /// Port I.
    I,
    /// Port J.
    J,
    /// Port K.
    K,
    /// Port Z.
    Z,
}

impl Port {
    /// Sets a range of pins on a port simultaneously.
    /// - `start_pin`: First pin in the range.
    /// - `pin_count`: Total number of pins.
    /// - `value`: Value to write.
    pub fn set_bus_output(&mut self, start_pin: u8, pin_count: u8, value: impl Into<u32>) {
        let value = BitWorker::new(value.into());
        let value =
            value.subvalue(start_pin, pin_count) | value.subvalue(start_pin + 16, pin_count);
        unsafe {
            match self {
                Port::A => &(*pac::GPIOA::ptr()).gpioa_bsrr.write(|w| w.bits(value)),
                Port::B => &(*pac::GPIOB::ptr()).gpiob_bsrr.write(|w| w.bits(value)),
                Port::C => &(*pac::GPIOC::ptr()).gpioc_bsrr.write(|w| w.bits(value)),
                Port::D => &(*pac::GPIOD::ptr()).gpiod_bsrr.write(|w| w.bits(value)),
                Port::E => &(*pac::GPIOE::ptr()).gpioe_bsrr.write(|w| w.bits(value)),
                Port::F => &(*pac::GPIOF::ptr()).gpiof_bsrr.write(|w| w.bits(value)),
                Port::G => &(*pac::GPIOG::ptr()).gpiog_bsrr.write(|w| w.bits(value)),
                Port::H => &(*pac::GPIOH::ptr()).gpioh_bsrr.write(|w| w.bits(value)),
                Port::I => &(*pac::GPIOI::ptr()).gpioi_bsrr.write(|w| w.bits(value)),
                Port::J => &(*pac::GPIOJ::ptr()).gpioj_bsrr.write(|w| w.bits(value)),
                Port::K => &(*pac::GPIOK::ptr()).gpiok_bsrr.write(|w| w.bits(value)),
                Port::Z => &(*pac::GPIOZ::ptr()).gpioz_bsrr.write(|w| w.bits(value)),
            };
        }
    }

    /// Reads a range of pins on a port simultaneously.
    /// - `start_pin`: First pin in the range.
    /// - `pin_count`: Total number of pins.
    pub fn get_bus_input(&self, start_pin: u8, pin_count: u8) -> u32 {
        let value = unsafe {
            match self {
                Port::A => (*pac::GPIOA::ptr()).gpioa_idr.read().bits(),
                Port::B => (*pac::GPIOB::ptr()).gpiob_idr.read().bits(),
                Port::C => (*pac::GPIOC::ptr()).gpioc_idr.read().bits(),
                Port::D => (*pac::GPIOD::ptr()).gpiod_idr.read().bits(),
                Port::E => (*pac::GPIOE::ptr()).gpioe_idr.read().bits(),
                Port::F => (*pac::GPIOF::ptr()).gpiof_idr.read().bits(),
                Port::G => (*pac::GPIOG::ptr()).gpiog_idr.read().bits(),
                Port::H => (*pac::GPIOH::ptr()).gpioh_idr.read().bits(),
                Port::I => (*pac::GPIOI::ptr()).gpioi_idr.read().bits(),
                Port::J => (*pac::GPIOJ::ptr()).gpioj_idr.read().bits(),
                Port::K => (*pac::GPIOK::ptr()).gpiok_idr.read().bits(),
                Port::Z => (*pac::GPIOZ::ptr()).gpioz_idr.read().bits(),
            }
        };
        BitWorker::new(value).subvalue(start_pin, pin_count)
    }
}

/// Bus covering several pins.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bus {
    /// Port of the pin.
    pub port: Port,
    /// Start pin number 0 - 15.
    pub start_pin: u8,
    /// Number of pins.
    pub pin_count: u8,
}

impl Bus {
    /// Returns a bus.
    pub fn new(port: Port, start_pin: u8, pin_count: u8) -> Self {
        Self {
            port,
            start_pin,
            pin_count,
        }
    }

    /// Writes an output value to the pins.
    pub fn set_output(&mut self, value: impl Into<u32>) {
        self.port
            .set_bus_output(self.start_pin, self.pin_count, value)
    }

    /// Reads an input value from the pins.
    pub fn get_input(&mut self) -> u32 {
        self.port.get_bus_input(self.start_pin, self.pin_count)
    }
}

/// Pin.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pin {
    /// Port of the pin.
    pub port: Port,
    /// Pin number 0 - 15.
    pub pin: u8,
}

impl Pin {
    /// Returns a pin.
    pub fn new(port: Port, pin: u8) -> Self {
        Self { port, pin }
    }

    /// Returns a pin initialized in the desired mode.
    pub fn with_mode(port: Port, pin: u8, mode: PinMode) -> Self {
        let mut pin = Self { port, pin };
        pin.set_mode(mode);

        pin
    }

    /// Returns a pin initialized in the desired mode.
    pub fn set_mode(&mut self, mode: PinMode) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.gpioa_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.gpiob_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.gpioc_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.gpiod_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.gpioe_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.gpiof_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.gpiog_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.gpioh_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.gpioi_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.gpioj_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.gpiok_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.gpioz_moder
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
        }

        match mode {
            PinMode::Alt(af_mode) => self.set_alternate_function(af_mode),
            _ => self.set_alternate_function(0),
        }
    }

    /// Returns the input state.
    pub fn get_input_state(&self) -> PinState {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                idr(regs.gpioa_idr.read().bits(), self.pin)
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                idr(regs.gpiob_idr.read().bits(), self.pin)
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                idr(regs.gpioc_idr.read().bits(), self.pin)
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                idr(regs.gpiod_idr.read().bits(), self.pin)
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                idr(regs.gpioe_idr.read().bits(), self.pin)
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                idr(regs.gpiof_idr.read().bits(), self.pin)
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                idr(regs.gpiog_idr.read().bits(), self.pin)
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                idr(regs.gpioh_idr.read().bits(), self.pin)
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                idr(regs.gpioi_idr.read().bits(), self.pin)
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                idr(regs.gpioj_idr.read().bits(), self.pin)
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                idr(regs.gpiok_idr.read().bits(), self.pin)
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                idr(regs.gpioz_idr.read().bits(), self.pin)
            },
        }
    }

    /// Sets the output state.
    pub fn set_output_state(&mut self, state: impl Into<PinState>) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.gpioa_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.gpiob_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.gpioc_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.gpiod_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.gpioe_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.gpiof_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.gpiog_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.gpioh_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.gpioi_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.gpioj_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.gpiok_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.gpioz_bsrr
                    .write(|w| w.bits(bsrr(self.pin, state.into())))
            },
        }
    }

    /// Sets the output speed.
    pub fn set_output_speed(&mut self, output_speed: OutputSpeed) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.gpioa_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.gpiob_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.gpioc_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.gpiod_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.gpioe_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.gpiof_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.gpiog_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.gpioh_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.gpioi_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.gpioj_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.gpiok_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.gpioz_ospeedr
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)))
            },
        }
    }

    /// Sets the output type.
    pub fn set_output_type(&mut self, output_type: OutputType) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.gpioa_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.gpiob_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.gpioc_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.gpiod_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.gpioe_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.gpiof_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.gpiog_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.gpioh_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.gpioi_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.gpioj_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.gpiok_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.gpioz_otyper
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)))
            },
        }
    }

    /// Sets the pull-up/pull-down mode.
    pub fn set_pull_mode(&mut self, pull_mode: PullMode) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.gpioa_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.gpiob_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.gpioc_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.gpiod_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.gpioe_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.gpiof_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.gpiog_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.gpioh_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.gpioi_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.gpioj_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.gpiok_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.gpioz_pupdr
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)))
            },
        }
    }

    /// Sets the alternate function.
    pub fn set_alternate_function(&mut self, af: u8) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                if self.pin < 8 {
                    regs.gpioa_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioa_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                if self.pin < 8 {
                    regs.gpiob_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpiob_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                if self.pin < 8 {
                    regs.gpioc_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioc_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                if self.pin < 8 {
                    regs.gpiod_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpiod_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                if self.pin < 8 {
                    regs.gpioe_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioe_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                if self.pin < 8 {
                    regs.gpiof_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpiof_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                if self.pin < 8 {
                    regs.gpiog_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpiog_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                if self.pin < 8 {
                    regs.gpioh_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioh_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                if self.pin < 8 {
                    regs.gpioi_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioi_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                if self.pin < 8 {
                    regs.gpioj_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioj_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                if self.pin < 8 {
                    regs.gpiok_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpiok_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                if self.pin < 8 {
                    regs.gpioz_afrl
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)))
                } else if self.pin < 16 {
                    regs.gpioz_afrh
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)))
                }
            },
        }
    }
}

impl ErrorType for Pin {
    type Error = core::convert::Infallible;
}

impl InputPin for Pin {
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.get_input_state() == PinState::Low)
    }

    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.get_input_state() == PinState::High)
    }
}

impl OutputPin for Pin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_output_state(PinState::Low);

        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_output_state(PinState::High);

        Ok(())
    }
}

impl StatefulOutputPin for Pin {
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.get_input_state() == PinState::Low)
    }

    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.get_input_state() == PinState::High)
    }
}

impl ToggleableOutputPin for Pin {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        match self.get_input_state() {
            PinState::Low => self.set_high(),
            PinState::High => self.set_low(),
        }
    }
}

/// Initializes the clocks for all ports.
pub fn init() {
    #[cfg(feature = "mpu-ca7")]
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mp_ahb4ensetr.modify(|_, w| {
            w.gpioaen()
                .set_bit()
                .gpioben()
                .set_bit()
                .gpiocen()
                .set_bit()
                .gpioden()
                .set_bit()
                .gpioeen()
                .set_bit()
                .gpiofen()
                .set_bit()
                .gpiogen()
                .set_bit()
                .gpiohen()
                .set_bit()
                .gpioien()
                .set_bit()
                .gpiojen()
                .set_bit()
                .gpioken()
                .set_bit()
        });
    }

    #[cfg(feature = "mcu-cm4")]
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rcc_mc_ahb4ensetr.modify(|_, w| {
            w.gpioaen()
                .set_bit()
                .gpioben()
                .set_bit()
                .gpiocen()
                .set_bit()
                .gpioden()
                .set_bit()
                .gpioeen()
                .set_bit()
                .gpiofen()
                .set_bit()
                .gpiogen()
                .set_bit()
                .gpiohen()
                .set_bit()
                .gpioien()
                .set_bit()
                .gpiojen()
                .set_bit()
                .gpioken()
                .set_bit()
        });
    }
}

/// Returns the modified MODER register value for a specific pin and mode.
fn modr(value: u32, pin: u8, mode: PinMode) -> u32 {
    BitWorker::new(value)
        .replace(u8::from(mode) as u32, pin * 2, 2)
        .value()
}

/// Returns the BSRR register value for a specific pin and state.
fn bsrr(pin: u8, state: PinState) -> u32 {
    let position = if state == PinState::High {
        pin
    } else {
        pin + 16
    };
    BitWorker::new(0).set(position).value()
}

/// Returns the modified OSPEEDR register value for a specific pin and speed.
fn ospeedr(value: u32, pin: u8, output_speed: OutputSpeed) -> u32 {
    BitWorker::new(value)
        .replace(u8::from(output_speed) as u32, pin * 2, 2)
        .value()
}

/// Returns the modified OTYPER register value for a specific pin and type.
fn otyper(value: u32, pin: u8, output_type: OutputType) -> u32 {
    BitWorker::new(value)
        .replace(u8::from(output_type) as u32, pin, 1)
        .value()
}

/// Returns the state from the IDR register value for a specific pin.
fn idr(value: u32, pin: u8) -> PinState {
    if BitWorker::new(value).is_set(pin) {
        PinState::High
    } else {
        PinState::Low
    }
}

/// Returns the modified PUPDR register value for a specific pin and pull mode.
fn pupdr(value: u32, pin: u8, pull_mode: PullMode) -> u32 {
    BitWorker::new(value)
        .replace(u8::from(pull_mode) as u32, pin * 2, 2)
        .value()
}

/// Returns the modified AFRL/AFRH register value for a specific pin and alternate function.
fn afr(value: u32, pin: u8, af: u8) -> u32 {
    BitWorker::new(value).replace(af as u32, pin * 4, 4).value()
}
