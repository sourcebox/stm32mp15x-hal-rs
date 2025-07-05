//! HSI oscillator.

use crate::pac;

/// Frequency of the HSI oscillator in Hz.
const HSI_FREQUENCY: u32 = 64000000;

/// Returns the HSI clock frequency in Hz.
pub fn hsi_frequency() -> u32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        if rcc.ocrdyr().read().hsidivrdy().bit_is_set() {
            HSI_FREQUENCY / hsi_div().value()
        } else {
            HSI_FREQUENCY
        }
    }
}

/// Returns the HSI divider.
pub fn hsi_div() -> HsiDiv {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        HsiDiv::try_from(rcc.hsicfgr().read().hsidiv().bits()).unwrap()
    }
}

/// HSI oscillator clock divider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HsiDiv {
    /// Division by 1 (64MHz).
    Div1,
    /// Division by 2 (32MHz).
    Div2,
    /// Division by 4 (16MHz).
    Div4,
    /// Division by 8 (8MHz).
    Div8,
}

impl TryFrom<u8> for HsiDiv {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(HsiDiv::Div1),
            0b01 => Ok(HsiDiv::Div2),
            0b10 => Ok(HsiDiv::Div4),
            0b11 => Ok(HsiDiv::Div8),
            _ => Err("Invalid value."),
        }
    }
}

impl From<HsiDiv> for u8 {
    fn from(value: HsiDiv) -> Self {
        match value {
            HsiDiv::Div1 => 0b00,
            HsiDiv::Div2 => 0b01,
            HsiDiv::Div4 => 0b10,
            HsiDiv::Div8 => 0b11,
        }
    }
}

impl HsiDiv {
    /// Returns the divider value.
    pub fn value(&self) -> u32 {
        match self {
            HsiDiv::Div1 => 1,
            HsiDiv::Div2 => 2,
            HsiDiv::Div4 => 4,
            HsiDiv::Div8 => 8,
        }
    }
}
