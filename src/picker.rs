use std::collections::HashSet;

use crate::pins::{Pins, TOTAL_CONFIGS};
use crate::score::{compute_score, Score, ALL_SCORES};

pub trait Picker {
  fn next_guess(&self) -> Pins;
  fn score_for_guess(&mut self, _: Pins, _: Score);
}

pub struct MinMaxPicker {
  possibilities: HashSet<Pins>,
  initialized: bool,
}

impl MinMaxPicker {
  pub fn new() -> Self {
    // Don't bother initializing the set of possible answers until after we get the score for the
    // first guess, since we'll always make the same first guess.
    Self {
      possibilities: HashSet::new(),
      initialized: false,
    }
  }
}

impl Picker for MinMaxPicker {
  fn next_guess(&self) -> Pins {
    if !self.initialized {
      // Always the same first guess.
      return Pins::new(0, 0, 1, 1);
    }

    if self.possibilities.len() == 1 {
      return *self.possibilities.iter().next().unwrap();
    }

    let mut guess = Pins::new(0, 0, 0, 0);

    let mut max_min_eliminated = 0;
    let mut max_min_guesses: Vec<Pins> = Vec::new();

    // Out of all possible guesses, pick the one that will eliminate the most possibilities from
    // the current set.
    for _ in 0..TOTAL_CONFIGS {
      guess.increment();

      // This guess will eliminate at least this many items from the possibilities set.
      let mut min_possibilities_eliminated = TOTAL_CONFIGS;

      for possible_score in ALL_SCORES {
        let mut eliminated_by_this_score = 0;

        for possible_answer in self.possibilities.iter() {
          if compute_score(guess, *possible_answer) != possible_score {
            eliminated_by_this_score += 1;
          }
        }

        if eliminated_by_this_score < min_possibilities_eliminated {
          min_possibilities_eliminated = eliminated_by_this_score;
        }
      }

      if min_possibilities_eliminated > max_min_eliminated {
        max_min_guesses.clear();
        max_min_guesses.push(guess);
        max_min_eliminated = min_possibilities_eliminated;
      } else if min_possibilities_eliminated == max_min_eliminated {
        max_min_guesses.push(guess);
      }
    }

    // If any of the min-maxed guesses is a possible answer, use that.
    for possible_answer in max_min_guesses.iter() {
      if self.possibilities.contains(possible_answer) {
        return *possible_answer;
      }
    }

    // This is OK -- this guess won't win the game, but it will still give us the maximum amount
    // of new information.
    println!("Guessing a non-possible answer");
    max_min_guesses[0]
  }

  fn score_for_guess(&mut self, guess: Pins, score: Score) {
    debug_assert!(score != (4, 0));

    if !self.initialized {
      self.initialized = true;

      // Populate possibilities with every possible solution that would result in the given score
      // for the given guess.
      let mut pins = Pins::new(0, 0, 0, 0);
      for _ in 0..TOTAL_CONFIGS {
        pins.increment();

        // We know the given guess isn't a possibility; don't bother computing its score.
        if pins == guess {
          continue;
        }

        if compute_score(pins, guess) == score {
          self.possibilities.insert(pins);
        }
      }
    } else {
      let mut eliminated_posses = HashSet::new();

      for possibility in self.possibilities.iter() {
        if compute_score(guess, *possibility) != score {
          eliminated_posses.insert(*possibility);
        }
      }

      for poss in eliminated_posses.iter() {
        self.possibilities.remove(poss);
      }
    }

    println!("{} possible answers left", self.possibilities.len());
  }
}
