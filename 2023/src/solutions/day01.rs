use aoc2023::util;
use std::io;

pub fn part_a(fname: &str) -> io::Result<()> {
  let lines = util::read_file(fname)?;
  let mut total = 0;
  for line in lines {
    if let Ok(input) = line {
      let tens_digit = find_digit(&mut input.chars(), None);
      let ones_digit = find_digit(&mut input.chars().rev(), None);

      total += 10 * tens_digit + ones_digit;
    } else {
      println!("Unable to read line.");
      return Err(io::ErrorKind::InvalidInput.into());
    }
  }

  println!("Day01, part a: {total}");
  Ok(())
}

pub fn part_b(fname: &str) -> io::Result<()> {
  let lines = util::read_file(fname)?;
  let mut total = 0;
  for line in lines {
    if let Ok(input) = line {
      let tens_digit = find_digit(&mut input.chars(), Some(Direction::Forward));
      let ones_digit = find_digit(&mut input.chars().rev(), Some(Direction::Backward));

      total += 10 * tens_digit + ones_digit;
    } else {
      println!("Unable to read line.");
      return Err(io::ErrorKind::InvalidInput.into());
    }
  }

  println!("Day01, part b: {total}");
  Ok(())
}

fn find_digit(characters: &mut dyn Iterator<Item = char>, direction: Option<Direction>) -> u32 {
  let mut states: Vec<NumberState> = Vec::new();

  for character in characters {
    if let Some(digit) = character.to_digit(10) {
      return digit;
    }

    if let Some(ref dir) = direction {
      states = states
        .into_iter()
        .filter_map(|mut state| {
          if check_state(character, &mut state) {
            return Some(state);
          }
          None
        })
        .collect();
      states.append(&mut find_new_states(&character, dir));

      for state in &states {
        let target = match dir {
          Direction::Forward => number_to_string(&state.number).len() - 1,
          Direction::Backward => 0,
        };
        if state.index == target {
          return number_to_u32(&state.number);
        }
      }
    }
  }

  panic!("Unable to find digit.");
}

fn find_new_states(character: &char, dir: &Direction) -> Vec<NumberState> {
  let mut new_states: Vec<NumberState> = Vec::new();

  match dir {
    Direction::Forward => match character {
      'z' => new_states.push(NumberState::new(Number::Zero)),
      'o' => {
        new_states.push(NumberState::new(Number::One));
      }
      't' => {
        new_states.push(NumberState::new(Number::Two));
        new_states.push(NumberState::new(Number::Three));
      }
      'f' => {
        new_states.push(NumberState::new(Number::Four));
        new_states.push(NumberState::new(Number::Five));
      }
      's' => {
        new_states.push(NumberState::new(Number::Six));
        new_states.push(NumberState::new(Number::Seven));
      }
      'e' => {
        new_states.push(NumberState::new(Number::Eight));
      }
      'n' => {
        new_states.push(NumberState::new(Number::Nine));
      }
      _ => (),
    },
    Direction::Backward => match character {
      'o' => {
        new_states.push(NumberState::new_reversed(Number::Zero));
        new_states.push(NumberState::new_reversed(Number::Two));
      }
      't' => {
        new_states.push(NumberState::new_reversed(Number::Eight));
      }
      'e' => {
        new_states.push(NumberState::new_reversed(Number::One));
        new_states.push(NumberState::new_reversed(Number::Three));
        new_states.push(NumberState::new_reversed(Number::Five));
        new_states.push(NumberState::new_reversed(Number::Nine));
      }
      'r' => new_states.push(NumberState::new_reversed(Number::Four)),
      'x' => new_states.push(NumberState::new_reversed(Number::Six)),
      'n' => {
        new_states.push(NumberState::new_reversed(Number::Seven));
      }
      _ => (),
    },
  }

  new_states
}

fn number_to_string(number: &Number) -> String {
  match number {
    Number::Zero => String::from("zero"),
    Number::One => String::from("one"),
    Number::Two => String::from("two"),
    Number::Three => String::from("three"),
    Number::Four => String::from("four"),
    Number::Five => String::from("five"),
    Number::Six => String::from("six"),
    Number::Seven => String::from("seven"),
    Number::Eight => String::from("eight"),
    Number::Nine => String::from("nine"),
  }
}

// First increments/decrements the index appropriately, with bounds check.
// Assuming the index is still in-bounds, checks if the character matches.
// Returns true if it does, false for any other reason
fn check_state(c: char, state: &mut NumberState) -> bool {
  let number_string = number_to_string(&state.number);

  match state.direction {
    Direction::Forward => {
      if state.index == number_string.len() - 1 {
        return false;
      }
      state.index += 1;
    }
    Direction::Backward => {
      if state.index == 0 {
        return false;
      }
      state.index -= 1;
    }
  }

  if c == number_string.chars().nth(state.index).unwrap() {
    return true;
  }

  false
}

#[derive(Debug)]
enum Number {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}

fn number_to_u32(number: &Number) -> u32 {
  match number {
    Number::Zero => 0,
    Number::One => 1,
    Number::Two => 2,
    Number::Three => 3,
    Number::Four => 4,
    Number::Five => 5,
    Number::Six => 6,
    Number::Seven => 7,
    Number::Eight => 8,
    Number::Nine => 9,
  }
}

#[derive(Debug)]
enum Direction {
  Forward,
  Backward,
}

#[derive(Debug)]
struct NumberState {
  number: Number,
  index: usize,
  direction: Direction,
}

impl NumberState {
  fn new(number: Number) -> NumberState {
    NumberState {
      number,
      index: 0,
      direction: Direction::Forward,
    }
  }

  fn new_reversed(number: Number) -> NumberState {
    let index = number_to_string(&number).len() - 1;
    NumberState {
      number,
      index,
      direction: Direction::Backward,
    }
  }
}
