//! GPIO ports and pins.

pub use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

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
                Port::A => &(*pac::GPIOA::ptr()).bsrr().write(|w| w.bits(value)),
                Port::B => &(*pac::GPIOB::ptr()).bsrr().write(|w| w.bits(value)),
                Port::C => &(*pac::GPIOC::ptr()).bsrr().write(|w| w.bits(value)),
                Port::D => &(*pac::GPIOD::ptr()).bsrr().write(|w| w.bits(value)),
                Port::E => &(*pac::GPIOE::ptr()).bsrr().write(|w| w.bits(value)),
                Port::F => &(*pac::GPIOF::ptr()).bsrr().write(|w| w.bits(value)),
                Port::G => &(*pac::GPIOG::ptr()).bsrr().write(|w| w.bits(value)),
                Port::H => &(*pac::GPIOH::ptr()).bsrr().write(|w| w.bits(value)),
                Port::I => &(*pac::GPIOI::ptr()).bsrr().write(|w| w.bits(value)),
                Port::J => &(*pac::GPIOJ::ptr()).bsrr().write(|w| w.bits(value)),
                Port::K => &(*pac::GPIOK::ptr()).bsrr().write(|w| w.bits(value)),
                Port::Z => &(*pac::GPIOZ::ptr()).bsrr().write(|w| w.bits(value)),
            };
        }
    }

    /// Reads a range of pins on a port simultaneously.
    /// - `start_pin`: First pin in the range.
    /// - `pin_count`: Total number of pins.
    pub fn get_bus_input(&self, start_pin: u8, pin_count: u8) -> u32 {
        let value = unsafe {
            match self {
                Port::A => (*pac::GPIOA::ptr()).idr().read().bits(),
                Port::B => (*pac::GPIOB::ptr()).idr().read().bits(),
                Port::C => (*pac::GPIOC::ptr()).idr().read().bits(),
                Port::D => (*pac::GPIOD::ptr()).idr().read().bits(),
                Port::E => (*pac::GPIOE::ptr()).idr().read().bits(),
                Port::F => (*pac::GPIOF::ptr()).idr().read().bits(),
                Port::G => (*pac::GPIOG::ptr()).idr().read().bits(),
                Port::H => (*pac::GPIOH::ptr()).idr().read().bits(),
                Port::I => (*pac::GPIOI::ptr()).idr().read().bits(),
                Port::J => (*pac::GPIOJ::ptr()).idr().read().bits(),
                Port::K => (*pac::GPIOK::ptr()).idr().read().bits(),
                Port::Z => (*pac::GPIOZ::ptr()).idr().read().bits(),
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
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.moder()
                    .modify(|r, w| w.bits(modr(r.bits(), self.pin, mode)));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.moder()
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
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                idr(regs.idr().read().bits(), self.pin)
            },
        }
    }

    /// Sets the output state.
    pub fn set_output_state(&mut self, state: impl Into<PinState>) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.bsrr().write(|w| w.bits(bsrr(self.pin, state.into())));
            },
        }
    }

    /// Sets the output speed.
    pub fn set_output_speed(&mut self, output_speed: OutputSpeed) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.ospeedr()
                    .modify(|r, w| w.bits(ospeedr(r.bits(), self.pin, output_speed)));
            },
        }
    }

    /// Sets the output type.
    pub fn set_output_type(&mut self, output_type: OutputType) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.otyper()
                    .modify(|r, w| w.bits(otyper(r.bits(), self.pin, output_type)));
            },
        }
    }

    /// Sets the pull-up/pull-down mode.
    pub fn set_pull_mode(&mut self, pull_mode: PullMode) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                regs.pupdr()
                    .modify(|r, w| w.bits(pupdr(r.bits(), self.pin, pull_mode)));
            },
        }
    }

    /// Sets the alternate function.
    pub fn set_alternate_function(&mut self, af: u8) {
        match self.port {
            Port::A => unsafe {
                let regs = &(*pac::GPIOA::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::B => unsafe {
                let regs = &(*pac::GPIOB::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::C => unsafe {
                let regs = &(*pac::GPIOC::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::D => unsafe {
                let regs = &(*pac::GPIOD::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::E => unsafe {
                let regs = &(*pac::GPIOE::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::F => unsafe {
                let regs = &(*pac::GPIOF::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::G => unsafe {
                let regs = &(*pac::GPIOG::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::H => unsafe {
                let regs = &(*pac::GPIOH::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::I => unsafe {
                let regs = &(*pac::GPIOI::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::J => unsafe {
                let regs = &(*pac::GPIOJ::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::K => unsafe {
                let regs = &(*pac::GPIOK::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
            Port::Z => unsafe {
                let regs = &(*pac::GPIOZ::ptr());
                if self.pin < 8 {
                    regs.afrl()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin, af)));
                } else if self.pin < 16 {
                    regs.afrh()
                        .modify(|r, w| w.bits(afr(r.bits(), self.pin - 8, af)));
                }
            },
        };
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
        rcc.mp_ahb4ensetr().modify(|_, w| {
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
        rcc.mc_ahb4ensetr().modify(|_, w| {
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
