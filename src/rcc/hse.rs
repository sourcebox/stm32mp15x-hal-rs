//! HSE oscillator.

/// Frequency of the HSE oscillator in Hz.
/// TODO: use actual value.
const HSE_FREQUENCY: u32 = 24000000;

/// Returns the frequency of the HSE oscillator in Hz.
pub fn hse_frequency() -> u32 {
    HSE_FREQUENCY
}
