/// A configuration of four pins, each of which can be one of six colors, can be represented with
/// 12 bits.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pins(u16);

pub const PINS: u8 = 4;
const BITS_PER_PIN: u8 = 3;

const MASK_ONE: u16 = (1 << BITS_PER_PIN) - 1;
const MASK_ALL: u16 = (1 << (PINS * BITS_PER_PIN)) - 1;
pub const COLORS: u8 = 1 << BITS_PER_PIN;
pub const TOTAL_CONFIGS: u16 = MASK_ALL + 1;

/// The storage format is as follows:
///
/// ```
/// |<--BITS_PER_PIN-->|<--BITS_PER_PIN-->|...|<--BITS_PER_PIN-->|
/// |     index 0      |     index 1      |...|   index PINS-1   |
/// least significant                             most significant
/// ```
impl Pins {
  pub fn new(a: u8, b: u8, c: u8, d: u8) -> Pins {
    *Pins(0).set(0, a).set(1, b).set(2, c).set(3, d)
  }

  pub fn get(&self, index: u8) -> u8 {
    debug_assert!(index < PINS);
    let shift_amount = index * BITS_PER_PIN;
    ((self.0 >> shift_amount) & MASK_ONE) as u8
  }

  pub fn set(&mut self, index: u8, new_mem: u8) -> &mut Self {
    debug_assert!(index < PINS);
    debug_assert!(new_mem <= MASK_ONE as u8);
    let shift_amount = index * BITS_PER_PIN;
    let mask = MASK_ONE << shift_amount;
    let deleted = self.0 & !mask;
    self.0 = deleted | ((new_mem as u16) << shift_amount);
    self
  }

  pub fn increment(&mut self) -> &mut Self {
    self.0 = (self.0 + 1) & MASK_ALL;
    self
  }
}

impl std::fmt::Debug for Pins {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("")
      .field(&self.get(0))
      .field(&self.get(1))
      .field(&self.get(2))
      .field(&self.get(3))
      .finish()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn increment() {
    let mut p = *Pins::new(0, 0, 0, 0).increment();
    assert_eq!(Pins::new(1, 0, 0, 0), p);
    p.set(0, 7).increment();
    assert_eq!(Pins::new(0, 1, 0, 0), p);

    p = *Pins::new(7, 7, 7, 7).increment();
    assert_eq!(Pins::new(0, 0, 0, 0), p);
  }
}
