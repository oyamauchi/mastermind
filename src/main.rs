mod picker;
mod pins;
mod score;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

use argparse::{ArgumentParser, Store, StoreTrue};
use random::Source;

use crate::picker::{MinMaxPicker, Picker};
use crate::pins::{Pins, COLORS};
use crate::score::compute_score;
use crate::score::Score;

fn read_score_interactively() -> Score {
  let input = stdin();
  let mut output = stdout();
  let mut buf = String::new();

  output.write(b"Black: ").unwrap();
  output.flush().unwrap();

  input.read_line(&mut buf).unwrap();
  let black: u8 = buf.trim().parse().unwrap();

  buf.clear();
  output.write(b"White: ").unwrap();
  output.flush().unwrap();

  input.read_line(&mut buf).unwrap();
  let white: u8 = buf.trim().parse().unwrap();

  (black, white)
}

fn main() {
  let mut interactive = false;
  let mut run_count = 1;
  let mut const_seed = false;

  {
    let mut parser = ArgumentParser::new();
    parser.refer(&mut interactive).add_option(
      &["--interactive"],
      StoreTrue,
      "Read scores from stdin",
    );
    parser.refer(&mut run_count).add_option(
      &["-c", "--count"],
      Store,
      "How many times to run the algorithm",
    );
    parser.refer(&mut const_seed).add_option(
      &["--const-seed"],
      StoreTrue,
      "Whether to use a constant RNG seed (vs. read from /dev/urandom)",
    );
    parser.parse_args_or_exit();
  }

  let mut rng = if !interactive {
    let mut rng = random::default();

    if !const_seed {
      let mut urandom = File::open("/dev/urandom").unwrap();
      let mut buffer1 = [0_u8; 8];
      let mut buffer2 = [0_u8; 8];

      urandom.read_exact(&mut buffer1).unwrap();
      urandom.read_exact(&mut buffer2).unwrap();
      rng = rng.seed([u64::from_ne_bytes(buffer1), u64::from_ne_bytes(buffer2)]);
    }

    Some(rng)
  } else {
    None
  };

  while run_count > 0 {
    run_count -= 1;

    let mut picker = MinMaxPicker::new();
    let mut guess_count = 0;
    let answer = match rng {
      Some(ref mut r) => Some(Pins::new(
        r.read::<u8>() % COLORS,
        r.read::<u8>() % COLORS,
        r.read::<u8>() % COLORS,
        r.read::<u8>() % COLORS,
      )),
      None => None,
    };

    loop {
      let guess = picker.next_guess();
      println!("Guess: {:?}", guess);
      guess_count += 1;

      let score = match answer {
        Some(ans) => compute_score(guess, ans),
        None => read_score_interactively(),
      };
      println!("Score: {:?}", score);

      if score == (4, 0) {
        println!("win in {}!", guess_count);
        break;
      }

      picker.score_for_guess(guess, score);
    }
  }
}
