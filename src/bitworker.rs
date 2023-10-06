//! Helper module for bit manipulation.

/// Representation of value for manipulation.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct BitWorker {
    /// Current value.
    value: u32,
}

impl BitWorker {
    /// Returns a new instance with a start value.
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Returns the current value.
    pub fn value(&self) -> u32 {
        self.value
    }
    /// Returns the value for a number of bits at a specific position.
    /// - `position`:   Bit number, starting with 0
    /// - `count`:      Number of bits
    ///
    /// # Example
    ///     subvalue(0b11010100, 3, 4) -> 0b1010
    pub fn subvalue(&self, position: u8, count: u8) -> u32 {
        (self.value >> position) & bitmask(count, 0)
    }

    /// Returns if a single bit is set at a specific position.
    /// - `position`:   Number of bit to check, starting with 0
    ///
    /// # Example
    ///     is_set(0b01100100, 5) -> true
    pub fn is_set(&self, position: u8) -> bool {
        ((self.value >> position) & 0x01) != 0
    }

    /// Sets a single bit at a specific position.
    /// - `position`:   Number of bit to set, starting with 0
    ///
    /// # Example
    ///     set(0b11000001, 3) -> 0b11001001
    pub fn set(&mut self, position: u8) -> &mut Self {
        self.value |= 1 << position;

        self
    }

    /// Clears a single bit at a specific position.
    /// - `position`:   Number of bit to clear, starting with 0
    ///
    /// # Example:
    ///     clear_at(0b11000001, 6) -> 0b10000001
    pub fn clear(&mut self, position: u32) -> &mut Self {
        self.value &= !(1 << position);

        self
    }

    /// Replaces a number of bits with a new value.
    /// - `replacement`:    Replacement value
    /// - `position`:       Bit offset for replacement, starting with 0
    /// - `count`:          Number of bits to replace
    ///
    /// # Example
    ///     replace(0b10010100, 0b1110, 3, 3) -> 0b10110100
    pub fn replace(&mut self, replacement: u32, position: u8, count: u8) -> &mut Self {
        let mask = bitmask(count, position);
        self.value = (self.value & !mask) | ((replacement << position) & mask);

        self
    }

    /// Mask the value.
    /// - `mask`:   Mask to apply.
    ///
    /// # Example
    ///     mask(0b11001011, 0b01100001) -> 0b01000001
    pub fn mask(&mut self, mask: u32) -> &mut Self {
        self.value &= mask;

        self
    }
}

/// Returns a mask for a number of bits.
/// - `count`:   Number of bits
/// - `offset`:  Bit offset, starting with 0
///  
/// # Example
///     bitmask(4, 2) -> 0b111100
pub fn bitmask(count: u8, offset: u8) -> u32 {
    ((1u32 << count) - 1) << offset
}
