//! Clock configuration.

use crate::hal::rcc;

pub fn init() {
    rcc::set_apb4_div(rcc::ApbDiv::Div2);
    rcc::set_apb5_div(rcc::ApbDiv::Div2);

    init_pll3();
    init_pll4();

    rcc::set_mcu_clock_source(rcc::McuSource::Pll3);
}

/// Initialize PLL3 for MCU.
fn init_pll3() {
    rcc::disable_pll3();
    rcc::set_pll3_source(rcc::Pll3Source::Hse);
    rcc::set_pll3_input_frequency_range(rcc::Pll3InputFreqRange::From8To16);
    rcc::set_pll3_prescaler(3);
    rcc::set_pll3_multiplier(52);
    rcc::set_pll3_p_divider(2);
    rcc::set_pll3_q_divider(2);
    rcc::set_pll3_r_divider(2);
    rcc::set_pll3_fractional(0);
    rcc::set_apb1_div(rcc::ApbDiv::Div2);
    rcc::set_apb2_div(rcc::ApbDiv::Div2);
    rcc::set_apb3_div(rcc::ApbDiv::Div2);
    rcc::enable_pll3();
}

/// Initialize PLL4 for SAI.
fn init_pll4() {
    rcc::set_pll4_source(rcc::Pll4Source::Hse);
    rcc::set_pll4_input_frequency_range(rcc::Pll4InputFreqRange::From8To16);

    // 98.304000 MHz for 48kHz sampling rate.
    rcc::set_pll4_prescaler(3);
    rcc::set_pll4_multiplier(61);
    rcc::set_pll4_p_divider(5);
    rcc::set_pll4_q_divider(5);
    rcc::set_pll4_r_divider(2);
    rcc::set_pll4_fractional(3604);

    rcc::enable_pll4();
}

/// Print some info.
pub fn print_info() {
    log::info!("PLL1:   {:>9} Hz", rcc::pll1_frequency());
    log::info!("PLL1 P: {:>9} Hz", rcc::pll1_p_frequency());
    log::info!("PLL1 Q: {:>9} Hz", rcc::pll1_q_frequency());
    log::info!("PLL1 R: {:>9} Hz", rcc::pll1_r_frequency());
    log::info!("PLL2:   {:>9} Hz", rcc::pll2_frequency());
    log::info!("PLL2 P: {:>9} Hz", rcc::pll2_p_frequency());
    log::info!("PLL2 Q: {:>9} Hz", rcc::pll2_q_frequency());
    log::info!("PLL2 R: {:>9} Hz", rcc::pll2_r_frequency());
    log::info!("PLL3:   {:>9} Hz", rcc::pll3_frequency());
    log::info!("PLL3 P: {:>9} Hz", rcc::pll3_p_frequency());
    log::info!("PLL3 Q: {:>9} Hz", rcc::pll3_q_frequency());
    log::info!("PLL3 R: {:>9} Hz", rcc::pll3_r_frequency());
    log::info!("PLL4:   {:>9} Hz", rcc::pll4_frequency());
    log::info!("PLL4 P: {:>9} Hz", rcc::pll4_p_frequency());
    log::info!("PLL4 Q: {:>9} Hz", rcc::pll4_q_frequency());
    log::info!("PLL4 R: {:>9} Hz", rcc::pll4_r_frequency());
    log::info!("MPU:    {:>9} Hz", rcc::mpu_frequency());
    log::info!("MCU:    {:>9} Hz", rcc::mcu_frequency());
    log::info!("ACLK:   {:>9} Hz", rcc::aclk_frequency());
    log::info!("PCLK1:  {:>9} Hz", rcc::pclk1_frequency());
    log::info!("PCLK2:  {:>9} Hz", rcc::pclk2_frequency());
    log::info!("PCLK3:  {:>9} Hz", rcc::pclk3_frequency());
    log::info!("PCLK4:  {:>9} Hz", rcc::pclk4_frequency());
    log::info!("PCLK5:  {:>9} Hz", rcc::pclk5_frequency());
    log::info!("PER_CK: {:>9} Hz", rcc::per_ck_frequency());
}
