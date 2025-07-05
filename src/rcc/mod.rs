//! Reset and clock control.

pub mod csi;
mod hse;
mod hsi;
mod pll;

use crate::pac;

pub use hse::*;
pub use hsi::*;
pub use pll::*;

// ------------------------------- MPU -------------------------------

/// Returns the MPU clock frequency in Hz.
pub fn mpu_frequency() -> f32 {
    match mpu_source() {
        MpuSource::Hsi => hsi_frequency() as f32,
        MpuSource::Hse => hse_frequency() as f32,
        MpuSource::Pll1 => pll1_p_frequency(),
        MpuSource::MpuDiv => {
            let mpu_div = mpu_div();
            match mpu_div {
                MpuDiv::Disabled => 0.0,
                _ => pll1_frequency() / mpu_div.value() as f32,
            }
        }
    }
}

/// Returns the MPU clock source.
pub fn mpu_source() -> MpuSource {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        MpuSource::try_from(rcc.mpckselr().read().mpusrc().bits()).unwrap()
    }
}

/// Returns the MPU clock divider.
pub fn mpu_div() -> MpuDiv {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        MpuDiv::from(rcc.mpckdivr().read().mpudiv().bits())
    }
}

/// MPU clock source.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MpuSource {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
    /// PLL1 clock.
    Pll1,
    /// PLL1 clock with divider.
    MpuDiv,
}

impl TryFrom<u8> for MpuSource {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(MpuSource::Hsi),
            0b01 => Ok(MpuSource::Hse),
            0b10 => Ok(MpuSource::Pll1),
            0b11 => Ok(MpuSource::MpuDiv),
            _ => Err("Invalid value."),
        }
    }
}

impl From<MpuSource> for u8 {
    fn from(value: MpuSource) -> Self {
        match value {
            MpuSource::Hsi => 0b00,
            MpuSource::Hse => 0b01,
            MpuSource::Pll1 => 0b10,
            MpuSource::MpuDiv => 0b11,
        }
    }
}

/// MPU core clock divider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MpuDiv {
    /// Disabled, no clock generated.
    Disabled,
    /// Division by 2.
    Div2,
    /// Division by 4.
    Div4,
    /// Division by 8.
    Div8,
    /// Division by 16.
    Div16,
}

impl From<u8> for MpuDiv {
    fn from(value: u8) -> Self {
        match value {
            0b000 => MpuDiv::Disabled,
            0b001 => MpuDiv::Div2,
            0b010 => MpuDiv::Div4,
            0b011 => MpuDiv::Div8,
            _ => MpuDiv::Div16,
        }
    }
}

impl From<MpuDiv> for u8 {
    fn from(value: MpuDiv) -> Self {
        match value {
            MpuDiv::Disabled => 0b000,
            MpuDiv::Div2 => 0b001,
            MpuDiv::Div4 => 0b010,
            MpuDiv::Div8 => 0b011,
            MpuDiv::Div16 => 0b100,
        }
    }
}

impl MpuDiv {
    /// Returns the divider value.
    pub fn value(&self) -> u32 {
        match self {
            MpuDiv::Disabled => 0,
            MpuDiv::Div2 => 2,
            MpuDiv::Div4 => 4,
            MpuDiv::Div8 => 8,
            MpuDiv::Div16 => 16,
        }
    }
}

// ------------------------------- AXI -------------------------------

/// Returns the ACLK frequency in Hz.
pub fn aclk_frequency() -> f32 {
    let f = match axi_source() {
        AxiSource::Hsi => hsi_frequency() as f32,
        AxiSource::Hse => hse_frequency() as f32,
        AxiSource::Pll2 => pll2_p_frequency(),
    };
    f / axi_div().value() as f32
}

/// Returns the AXI clock source.
pub fn axi_source() -> AxiSource {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        AxiSource::try_from(rcc.assckselr().read().axissrc().bits()).unwrap()
    }
}

/// Returns the AXI clock divider.
pub fn axi_div() -> AxiDiv {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        AxiDiv::from(rcc.axidivr().read().axidiv().bits())
    }
}

/// AXI clock source.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxiSource {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
    /// PLL2 clock.
    Pll2,
}

impl TryFrom<u8> for AxiSource {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(AxiSource::Hsi),
            0b001 => Ok(AxiSource::Hse),
            0b010 => Ok(AxiSource::Pll2),
            _ => Err("Invalid value."),
        }
    }
}

impl From<AxiSource> for u8 {
    fn from(value: AxiSource) -> Self {
        match value {
            AxiSource::Hsi => 0b000,
            AxiSource::Hse => 0b001,
            AxiSource::Pll2 => 0b010,
        }
    }
}

/// AXI clock divider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AxiDiv {
    /// Division by 1.
    Div1,
    /// Division by 2.
    Div2,
    /// Division by 3.
    Div3,
    /// Division by 4.
    Div4,
}

impl From<u8> for AxiDiv {
    fn from(value: u8) -> Self {
        match value {
            0b000 => AxiDiv::Div1,
            0b001 => AxiDiv::Div2,
            0b010 => AxiDiv::Div3,
            _ => AxiDiv::Div4,
        }
    }
}

impl From<AxiDiv> for u8 {
    fn from(value: AxiDiv) -> Self {
        match value {
            AxiDiv::Div1 => 0b000,
            AxiDiv::Div2 => 0b001,
            AxiDiv::Div3 => 0b010,
            AxiDiv::Div4 => 0b011,
        }
    }
}

impl AxiDiv {
    /// Returns the divider value.
    pub fn value(&self) -> u32 {
        match self {
            AxiDiv::Div1 => 1,
            AxiDiv::Div2 => 2,
            AxiDiv::Div3 => 3,
            AxiDiv::Div4 => 4,
        }
    }
}

// ------------------------------- MCU -------------------------------

/// Sets the MCU clock source.
pub fn set_mcu_clock_source(source: McuSource) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.mssckselr()
            .modify(|_, w| w.mcussrc().bits(source.into()));
        while rcc.mssckselr().read().mcussrcrdy().bit_is_clear() {}
    }
}

/// Returns the MCU clock frequency in Hz.
pub fn mcu_frequency() -> f32 {
    let f = match mcu_source() {
        McuSource::Hsi => hsi_frequency() as f32,
        McuSource::Hse => hse_frequency() as f32,
        McuSource::Csi => csi::Csi::new().frequency() as f32,
        McuSource::Pll3 => pll3_p_frequency(),
    };
    f / mcu_div().value() as f32
}

/// Returns the MCU clock source.
pub fn mcu_source() -> McuSource {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        McuSource::try_from(rcc.mssckselr().read().mcussrc().bits()).unwrap()
    }
}

/// Returns the MCU clock divider.
pub fn mcu_div() -> McuDiv {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        McuDiv::from(rcc.mcudivr().read().mcudiv().bits())
    }
}

/// MCU clock source.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum McuSource {
    /// HSI clock.
    Hsi,
    /// HSE clock.
    Hse,
    /// CSI clock.
    Csi,
    /// PLL3 clock.
    Pll3,
}

impl TryFrom<u8> for McuSource {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(McuSource::Hsi),
            0b01 => Ok(McuSource::Hse),
            0b10 => Ok(McuSource::Csi),
            0b11 => Ok(McuSource::Pll3),
            _ => Err("Invalid value."),
        }
    }
}

impl From<McuSource> for u8 {
    fn from(value: McuSource) -> Self {
        match value {
            McuSource::Hsi => 0b00,
            McuSource::Hse => 0b01,
            McuSource::Csi => 0b10,
            McuSource::Pll3 => 0b11,
        }
    }
}

/// MCU clock divider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum McuDiv {
    /// Division by 1.
    Div1,
    /// Division by 2.
    Div2,
    /// Division by 4.
    Div4,
    /// Division by 8.
    Div8,
    /// Division by 16.
    Div16,
    /// Division by 32.
    Div32,
    /// Division by 64.
    Div64,
    /// Division by 128.
    Div128,
    /// Division by 256.
    Div256,
    /// Division by 512.
    Div512,
}

impl From<u8> for McuDiv {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => McuDiv::Div1,
            0b0001 => McuDiv::Div2,
            0b0010 => McuDiv::Div4,
            0b0011 => McuDiv::Div8,
            0b0100 => McuDiv::Div16,
            0b0101 => McuDiv::Div32,
            0b0110 => McuDiv::Div64,
            0b0111 => McuDiv::Div128,
            0b1000 => McuDiv::Div256,
            _ => McuDiv::Div512,
        }
    }
}

impl From<McuDiv> for u8 {
    fn from(value: McuDiv) -> Self {
        match value {
            McuDiv::Div1 => 0b0000,
            McuDiv::Div2 => 0b0001,
            McuDiv::Div4 => 0b0010,
            McuDiv::Div8 => 0b0011,
            McuDiv::Div16 => 0b0100,
            McuDiv::Div32 => 0b0101,
            McuDiv::Div64 => 0b0110,
            McuDiv::Div128 => 0b0111,
            McuDiv::Div256 => 0b1000,
            McuDiv::Div512 => 0b1001,
        }
    }
}

impl McuDiv {
    /// Returns the divider value.
    pub fn value(&self) -> u32 {
        match self {
            McuDiv::Div1 => 1,
            McuDiv::Div2 => 2,
            McuDiv::Div4 => 4,
            McuDiv::Div8 => 8,
            McuDiv::Div16 => 16,
            McuDiv::Div32 => 32,
            McuDiv::Div64 => 64,
            McuDiv::Div128 => 128,
            McuDiv::Div256 => 256,
            McuDiv::Div512 => 512,
        }
    }
}

// ------------------------------- APB -------------------------------

/// Returns the PCLK1 frequency in Hz.
pub fn pclk1_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    let divider = ApbDiv::try_from(rcc.apb1divr().read().apb1div().bits())
        .unwrap()
        .value();
    mcu_frequency() / divider as f32
}

/// Returns the PCLK2 frequency in Hz.
pub fn pclk2_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    let divider = ApbDiv::try_from(rcc.apb2divr().read().apb2div().bits())
        .unwrap()
        .value();
    mcu_frequency() / divider as f32
}

/// Returns the PCLK3 frequency in Hz.
pub fn pclk3_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    let divider = ApbDiv::try_from(rcc.apb3divr().read().apb3div().bits())
        .unwrap()
        .value();
    mcu_frequency() / divider as f32
}

/// Returns the PCLK4 frequency in Hz.
pub fn pclk4_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    let divider = ApbDiv::try_from(rcc.apb4divr().read().apb4div().bits())
        .unwrap()
        .value();
    aclk_frequency() / divider as f32
}

/// Returns the PCLK5 frequency in Hz.
pub fn pclk5_frequency() -> f32 {
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    let divider = ApbDiv::try_from(rcc.apb5divr().read().apb5div().bits())
        .unwrap()
        .value();
    aclk_frequency() / divider as f32
}

/// Sets the divider for APB1.
pub fn set_apb1_div(divider: ApbDiv) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb1divr()
            .modify(|_, w| w.apb1div().bits(divider.into()));
    }
}

/// Sets the divider for APB2.
pub fn set_apb2_div(divider: ApbDiv) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb2divr()
            .modify(|_, w| w.apb2div().bits(divider.into()));
    }
}

/// Sets the divider for APB2.
pub fn set_apb3_div(divider: ApbDiv) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb3divr()
            .modify(|_, w| w.apb3div().bits(divider.into()));
    }
}

/// Sets the divider for APB4.
pub fn set_apb4_div(divider: ApbDiv) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb4divr()
            .modify(|_, w| w.apb4div().bits(divider.into()));
    }
}

/// Sets the divider for APB5.
pub fn set_apb5_div(divider: ApbDiv) {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb5divr()
            .modify(|_, w| w.apb5div().bits(divider.into()));
    }
}

/// APB clock divider.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ApbDiv {
    /// Division by 1.
    Div1,
    /// Division by 2.
    Div2,
    /// Division by 4.
    Div4,
    /// Division by 8.
    Div8,
    /// Division by 16.
    Div16,
}

impl TryFrom<u8> for ApbDiv {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(ApbDiv::Div1),
            0b0001 => Ok(ApbDiv::Div2),
            0b0010 => Ok(ApbDiv::Div4),
            0b0011 => Ok(ApbDiv::Div8),
            0b0100 => Ok(ApbDiv::Div16),
            _ => Err("Invalid value."),
        }
    }
}

impl From<ApbDiv> for u8 {
    fn from(value: ApbDiv) -> Self {
        match value {
            ApbDiv::Div1 => 0b000,
            ApbDiv::Div2 => 0b001,
            ApbDiv::Div4 => 0b010,
            ApbDiv::Div8 => 0b011,
            ApbDiv::Div16 => 0b100,
        }
    }
}

impl ApbDiv {
    /// Returns the divider value.
    pub fn value(&self) -> u32 {
        match self {
            ApbDiv::Div1 => 1,
            ApbDiv::Div2 => 2,
            ApbDiv::Div4 => 4,
            ApbDiv::Div8 => 8,
            ApbDiv::Div16 => 16,
        }
    }
}

// ------------------------------- PER -------------------------------

/// Returns the PER_CK frequency in Hz.
pub fn per_ck_frequency() -> f32 {
    match per_source() {
        PerSource::Hsi => hsi_frequency() as f32,
        PerSource::Csi => csi::Csi::new().frequency() as f32,
        PerSource::Hse => hse_frequency() as f32,
        PerSource::Disabled => 0.0,
    }
}

/// Returns the PER clock source.
pub fn per_source() -> PerSource {
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        PerSource::try_from(rcc.cperckselr().read().ckpersrc().bits()).unwrap()
    }
}

/// PER clock source.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PerSource {
    /// HSI clock.
    Hsi,
    /// CSI clock.
    Csi,
    /// HSE clock.
    Hse,
    /// Disabled.
    Disabled,
}

impl TryFrom<u8> for PerSource {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(PerSource::Hsi),
            0b01 => Ok(PerSource::Csi),
            0b10 => Ok(PerSource::Hse),
            0b11 => Ok(PerSource::Disabled),
            _ => Err("Invalid value."),
        }
    }
}

impl From<PerSource> for u8 {
    fn from(value: PerSource) -> Self {
        match value {
            PerSource::Hsi => 0b00,
            PerSource::Csi => 0b01,
            PerSource::Hse => 0b10,
            PerSource::Disabled => 0b11,
        }
    }
}
