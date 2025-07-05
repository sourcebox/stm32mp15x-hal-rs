//! PLL configuration.

use super::{hse, hsi};
use crate::pac;

// ------------------------------- PLL1 -------------------------------

/// Returns if PLL1 is enabled.
pub fn is_pll1_enabled() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll1cr().read().pllon().bit_is_set()
}

/// Returns if PLL1 is ready.
pub fn is_pll1_ready() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll1cr().read().pll1rdy().bit_is_set()
}

/// Returns the PLL1 clock frequency in Hz.
pub fn pll1_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };

    if !(is_pll1_enabled() && is_pll1_ready()) {
        return 0.0;
    }

    let frac = pll1_fractional() as f32;
    let pll1_n = (rcc.pll1cfgr1().read().divn().bits() + 1) as f32;
    let pll1_m = (rcc.pll1cfgr1().read().divm1().bits() + 1) as f32;
    let pll1_vco = pll1_n + (frac / 0x2000 as f32);

    match pll12_source() {
        Pll12Source::Hsi => pll1_vco * hsi::hsi_frequency() as f32 / pll1_m,
        Pll12Source::Hse => pll1_vco * hse::hse_frequency() as f32 / pll1_m,
    }
}

/// Returns the PLL1 P frequency in Hz.
pub fn pll1_p_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll1_frequency() / (rcc.pll1cfgr2().read().divp().bits() + 1) as f32
    }
}

/// Returns the PLL1 Q frequency in Hz.
pub fn pll1_q_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll1_frequency() / (rcc.pll1cfgr2().read().divq().bits() + 1) as f32
    }
}

/// Returns the PLL1 R frequency in Hz.
pub fn pll1_r_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll1_frequency() / (rcc.pll1cfgr2().read().divr().bits() + 1) as f32
    }
}

/// Returns the PLL1 fractional value.
pub fn pll1_fractional() -> u16 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        if rcc.pll1fracr().read().fracle().bit_is_set() {
            rcc.pll1fracr().read().fracv().bits()
        } else {
            0
        }
    }
}

// ------------------------------- PLL2 -------------------------------

/// Returns if PLL2 is enabled.
pub fn is_pll2_enabled() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll2cr().read().pllon().bit_is_set()
}

/// Returns if PLL2 is ready.
pub fn is_pll2_ready() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll2cr().read().pll2rdy().bit_is_set()
}

/// Returns the PLL2 clock frequency in Hz.
pub fn pll2_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };

    if !(is_pll2_enabled() && is_pll2_ready()) {
        return 0.0;
    }

    let frac = pll2_fractional() as f32;
    let pll2_n = (rcc.pll2cfgr1().read().divn().bits() + 1) as f32;
    let pll2_m = (rcc.pll2cfgr1().read().divm2().bits() + 1) as f32;
    let pll2_vco = pll2_n + (frac / 0x2000 as f32);

    match pll12_source() {
        Pll12Source::Hsi => pll2_vco * hsi::hsi_frequency() as f32 / pll2_m,
        Pll12Source::Hse => pll2_vco * hse::hse_frequency() as f32 / pll2_m,
    }
}

/// Returns the PLL2 P frequency in Hz.
pub fn pll2_p_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll2_frequency() / (rcc.pll2cfgr2().read().divp().bits() + 1) as f32
    }
}

/// Returns the PLL2 Q frequency in Hz.
pub fn pll2_q_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll2_frequency() / (rcc.pll2cfgr2().read().divq().bits() + 1) as f32
    }
}

/// Returns the PLL2 R frequency in Hz.
pub fn pll2_r_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll2_frequency() / (rcc.pll2cfgr2().read().divr().bits() + 1) as f32
    }
}

/// Returns the PLL2 fractional value.
pub fn pll2_fractional() -> u16 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        if rcc.pll2fracr().read().fracle().bit_is_set() {
            rcc.pll2fracr().read().fracv().bits()
        } else {
            0
        }
    }
}

// ------------------------------ PLL1/2 ------------------------------

/// Returns the PLL1/2 clock source.
pub fn pll12_source() -> Pll12Source {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        Pll12Source::try_from(rcc.rck12selr().read().pll12src().bits()).unwrap()
    }
}

/// Clock sources for PLL1/2.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pll12Source {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
}

impl TryFrom<u8> for Pll12Source {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0 => Ok(Pll12Source::Hsi),
            0b1 => Ok(Pll12Source::Hse),
            _ => Err("Invalid value."),
        }
    }
}

impl From<Pll12Source> for u8 {
    fn from(value: Pll12Source) -> Self {
        match value {
            Pll12Source::Hsi => 0b0,
            Pll12Source::Hse => 0b1,
        }
    }
}

// ------------------------------- PLL3 -------------------------------

/// Enables PLL3.
pub fn enable_pll3() {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll3cr().modify(|_, w| w.pllon().set_bit());
    while rcc.pll3cr().read().pll3rdy().bit_is_clear() {}
    rcc.pll3cr()
        .modify(|_, w| w.divren().set_bit().divqen().set_bit().divpen().set_bit());
}

/// Disables PLL3.
pub fn disable_pll3() {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll3cr().modify(|_, w| {
        w.divren()
            .clear_bit()
            .divqen()
            .clear_bit()
            .divpen()
            .clear_bit()
    });
    rcc.pll3cr().modify(|_, w| w.pllon().clear_bit());
    while rcc.pll3cr().read().pll3rdy().bit_is_set() {}
}

/// Sets the PLL3 source.
pub fn set_pll3_source(source: Pll3Source) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rck3selr()
            .modify(|_, w| w.pll3src().bits(source.into()));
    }
}

/// Sets the PLL3 input frequency range.
pub fn set_pll3_input_frequency_range(freq_range: Pll3InputFreqRange) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr1()
            .modify(|_, w| w.ifrge().bits(freq_range.into()));
    }
}

/// Sets the PLL3 prescaler.
pub fn set_pll3_prescaler(prescaler: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr1()
            .modify(|_, w| w.divm3().bits((prescaler - 1).clamp(0x00, 0x3F)));
    }
}

/// Sets the PLL3 multiplier.
pub fn set_pll3_multiplier(multiplier: u16) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr1()
            .modify(|_, w| w.divn().bits((multiplier - 1).clamp(0x18, 0xC7)));
    }
}

/// Sets the PLL4 R divider.
pub fn set_pll3_r_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr2()
            .modify(|_, w| w.divr().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL3 Q divider.
pub fn set_pll3_q_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr2()
            .modify(|_, w| w.divq().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL3 P divider.
pub fn set_pll3_p_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3cfgr2()
            .modify(|_, w| w.divp().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL3 fractional value.
pub fn set_pll3_fractional(fractional: u16) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll3fracr()
            .modify(|_, w| w.fracv().bits(fractional).fracle().bit(fractional != 0));
    }
}

/// Returns if PLL3 is enabled.
pub fn is_pll3_enabled() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll3cr().read().pllon().bit_is_set()
}

/// Returns if PLL3 is ready.
pub fn is_pll3_ready() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll3cr().read().pll3rdy().bit_is_set()
}

/// Returns the PLL3 clock frequency in Hz.
pub fn pll3_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };

    if !(is_pll3_enabled() && is_pll3_ready()) {
        return 0.0;
    }

    let frac = pll3_fractional() as f32;
    let pll3_n = (rcc.pll3cfgr1().read().divn().bits() + 1) as f32;
    let pll3_m = (rcc.pll3cfgr1().read().divm3().bits() + 1) as f32;
    let pll3_vco = pll3_n + (frac / 0x2000 as f32);

    match pll3_source() {
        Pll3Source::Hsi => pll3_vco * hsi::hsi_frequency() as f32 / pll3_m,
        Pll3Source::Hse => pll3_vco * hse::hse_frequency() as f32 / pll3_m,
        Pll3Source::Csi => todo!(),
    }
}

/// Returns the PLL3 P frequency in Hz.
pub fn pll3_p_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll3_frequency() / (rcc.pll3cfgr2().read().divp().bits() + 1) as f32
    }
}

/// Returns the PLL3 Q frequency in Hz.
pub fn pll3_q_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll3_frequency() / (rcc.pll3cfgr2().read().divq().bits() + 1) as f32
    }
}

/// Returns the PLL3 R frequency in Hz.
pub fn pll3_r_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll3_frequency() / (rcc.pll3cfgr2().read().divr().bits() + 1) as f32
    }
}

/// Returns the PLL3 fractional value.
pub fn pll3_fractional() -> u16 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        if rcc.pll3fracr().read().fracle().bit_is_set() {
            rcc.pll3fracr().read().fracv().bits()
        } else {
            0
        }
    }
}

/// Returns the PLL3 clock source.
pub fn pll3_source() -> Pll3Source {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        Pll3Source::try_from(rcc.rck3selr().read().pll3src().bits()).unwrap()
    }
}

/// Clock sources for PLL3.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pll3Source {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
    /// CSI clock.
    Csi,
}

impl TryFrom<u8> for Pll3Source {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Pll3Source::Hsi),
            0b01 => Ok(Pll3Source::Hse),
            0b10 => Ok(Pll3Source::Csi),
            _ => Err("Invalid value."),
        }
    }
}

impl From<Pll3Source> for u8 {
    fn from(value: Pll3Source) -> Self {
        match value {
            Pll3Source::Hsi => 0b00,
            Pll3Source::Hse => 0b01,
            Pll3Source::Csi => 0b10,
        }
    }
}

/// Input frequency range for PLL3.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pll3InputFreqRange {
    /// 4MHz to 8MHz.
    From4To8,
    /// 8MHz to 16 MHz.
    From8To16,
}

impl TryFrom<u8> for Pll3InputFreqRange {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0 => Ok(Pll3InputFreqRange::From4To8),
            0b1 => Ok(Pll3InputFreqRange::From8To16),
            _ => Err("Invalid value."),
        }
    }
}

impl From<Pll3InputFreqRange> for u8 {
    fn from(value: Pll3InputFreqRange) -> Self {
        match value {
            Pll3InputFreqRange::From4To8 => 0b0,
            Pll3InputFreqRange::From8To16 => 0b1,
        }
    }
}

// ------------------------------- PLL4 -------------------------------

/// Enables PLL4.
pub fn enable_pll4() {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll4cr().modify(|_, w| w.pllon().set_bit());
    while rcc.pll4cr().read().pll4rdy().bit_is_clear() {}
    rcc.pll4cr()
        .modify(|_, w| w.divren().set_bit().divqen().set_bit().divpen().set_bit());
}

/// Disables PLL4.
pub fn disable_pll4() {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll4cr().modify(|_, w| {
        w.divren()
            .clear_bit()
            .divqen()
            .clear_bit()
            .divpen()
            .clear_bit()
    });
    rcc.pll4cr().modify(|_, w| w.pllon().clear_bit());
    while rcc.pll4cr().read().pll4rdy().bit_is_set() {}
}

/// Sets the PLL4 source.
pub fn set_pll4_source(source: Pll4Source) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.rck4selr()
            .modify(|_, w| w.pll4src().bits(source.into()));
    }
}

/// Sets the PLL4 input frequency range.
pub fn set_pll4_input_frequency_range(freq_range: Pll4InputFreqRange) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr1()
            .modify(|_, w| w.ifrge().bits(freq_range.into()));
    }
}

/// Sets the PLL4 prescaler.
pub fn set_pll4_prescaler(prescaler: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr1()
            .modify(|_, w| w.divm4().bits((prescaler - 1).clamp(0x00, 0x3F)));
    }
}

/// Sets the PLL4 multiplier.
pub fn set_pll4_multiplier(multiplier: u16) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr1()
            .modify(|_, w| w.divn().bits((multiplier - 1).clamp(0x18, 0xC7)));
    }
}

/// Sets the PLL4 R divider.
pub fn set_pll4_r_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr2()
            .modify(|_, w| w.divr().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL4 Q divider.
pub fn set_pll4_q_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr2()
            .modify(|_, w| w.divq().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL4 P divider.
pub fn set_pll4_p_divider(divider: u8) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4cfgr2()
            .modify(|_, w| w.divp().bits((divider - 1).clamp(0x00, 0x7F)));
    }
}

/// Sets the PLL4 fractional value.
pub fn set_pll4_fractional(fractional: u16) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.pll4fracr()
            .modify(|_, w| w.fracv().bits(fractional).fracle().bit(fractional != 0));
    }
}

/// Returns if PLL4 is enabled.
pub fn is_pll4_enabled() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll4cr().read().pllon().bit_is_set()
}

/// Returns if PLL4 is ready.
pub fn is_pll4_ready() -> bool {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    rcc.pll4cr().read().pll4rdy().bit_is_set()
}

/// Returns the PLL4 clock frequency in Hz.
pub fn pll4_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };

    if !(is_pll4_enabled() && is_pll4_ready()) {
        return 0.0;
    }

    let frac = pll4_fractional() as f32;
    let pll4_n = (rcc.pll4cfgr1().read().divn().bits() + 1) as f32;
    let pll4_m = (rcc.pll4cfgr1().read().divm4().bits() + 1) as f32;
    let pll4_vco = pll4_n + (frac / 0x2000 as f32);

    match pll4_source() {
        Pll4Source::Hsi => pll4_vco * hsi::hsi_frequency() as f32 / pll4_m,
        Pll4Source::Hse => pll4_vco * hse::hse_frequency() as f32 / pll4_m,
        Pll4Source::Csi => todo!(),
        Pll4Source::I2sClockIn => todo!(),
    }
}

/// Returns the PLL4 P frequency in Hz.
pub fn pll4_p_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll4_frequency() / (rcc.pll4cfgr2().read().divp().bits() + 1) as f32
    }
}

/// Returns the PLL4 Q frequency in Hz.
pub fn pll4_q_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll4_frequency() / (rcc.pll4cfgr2().read().divq().bits() + 1) as f32
    }
}

/// Returns the PLL4 R frequency in Hz.
pub fn pll4_r_frequency() -> f32 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        pll4_frequency() / (rcc.pll4cfgr2().read().divr().bits() + 1) as f32
    }
}

/// Returns the PLL4 fractional value.
pub fn pll4_fractional() -> u16 {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        if rcc.pll4fracr().read().fracle().bit_is_set() {
            rcc.pll4fracr().read().fracv().bits()
        } else {
            0
        }
    }
}

/// Returns the PLL4 clock source.
pub fn pll4_source() -> Pll4Source {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        Pll4Source::try_from(rcc.rck4selr().read().pll4src().bits()).unwrap()
    }
}

/// Clock sources for PLL4.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pll4Source {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
    /// CSI clock.
    Csi,
    /// I2S clock input.
    I2sClockIn,
}

impl TryFrom<u8> for Pll4Source {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Pll4Source::Hsi),
            0b01 => Ok(Pll4Source::Hse),
            0b10 => Ok(Pll4Source::Csi),
            0b11 => Ok(Pll4Source::I2sClockIn),
            _ => Err("Invalid value."),
        }
    }
}

impl From<Pll4Source> for u8 {
    fn from(value: Pll4Source) -> Self {
        match value {
            Pll4Source::Hsi => 0b00,
            Pll4Source::Hse => 0b01,
            Pll4Source::Csi => 0b10,
            Pll4Source::I2sClockIn => 0b11,
        }
    }
}

/// Input frequency range for PLL4.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Pll4InputFreqRange {
    /// 4MHz to 8MHz.
    From4To8,
    /// 8MHz to 16 MHz.
    From8To16,
}

impl TryFrom<u8> for Pll4InputFreqRange {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0 => Ok(Pll4InputFreqRange::From4To8),
            0b1 => Ok(Pll4InputFreqRange::From8To16),
            _ => Err("Invalid value."),
        }
    }
}

impl From<Pll4InputFreqRange> for u8 {
    fn from(value: Pll4InputFreqRange) -> Self {
        match value {
            Pll4InputFreqRange::From4To8 => 0b0,
            Pll4InputFreqRange::From8To16 => 0b1,
        }
    }
}
