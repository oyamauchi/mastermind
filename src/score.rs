use crate::pins::{Pins, COLORS};

/// (Black, white)
pub type Score = (u8, u8);

/// Any score whose components add up to 4 or less is possible, except for (3, 1).
pub const ALL_SCORES: [Score; 14] = [
  (0, 0),
  (0, 1),
  (0, 2),
  (0, 3),
  (0, 4),
  (1, 0),
  (1, 1),
  (1, 2),
  (1, 3),
  (2, 0),
  (2, 1),
  (2, 2),
  (3, 0),
  (4, 0),
];

pub fn compute_score(attempt: Pins, actual: Pins) -> Score {
  macro_rules! min {
    ($a:expr, $b:expr) => {
      if $a < $b {
        $a
      } else {
        $b
      }
    };
  }

  let mut att_counts = [0_u8; COLORS as usize];
  let mut actual_counts = [0_u8; COLORS as usize];
  let mut black = 0;

  for i in 0..4 {
    if attempt.get(i) == actual.get(i) {
      black += 1;
    } else {
      att_counts[attempt.get(i) as usize] += 1;
      actual_counts[actual.get(i) as usize] += 1;
    }
  }

  let mut white = 0;

  for i in 0..att_counts.len() {
    white += min!(att_counts[i], actual_counts[i]);
  }

  (black, white)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      (4, 0),
      super::compute_score(Pins::new(0, 1, 2, 3), Pins::new(0, 1, 2, 3))
    );
    assert_eq!(
      (0, 4),
      super::compute_score(Pins::new(0, 1, 2, 3), Pins::new(1, 2, 3, 0))
    );
    assert_eq!(
      (1, 0),
      super::compute_score(Pins::new(0, 0, 0, 0), Pins::new(0, 1, 2, 3))
    );
    assert_eq!(
      (1, 0),
      super::compute_score(Pins::new(0, 1, 2, 3), Pins::new(0, 0, 0, 0))
    );
    assert_eq!(
      (2, 2),
      super::compute_score(Pins::new(0, 1, 2, 3), Pins::new(0, 3, 2, 1))
    );
  }
}
