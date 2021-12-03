use advent_of_code::util::*;

enum Direction {
  Forward,
  Up,
  Down,
}

struct SubmarineMovement {
  dir: Direction,
  amt: i32,
}

fn make_direction(input: &str) -> SubmarineMovement {
  let (raw_dir, raw_amt) = input
    .split_once(' ')
    .unwrap_or_else(|| panic!("Unable to split the string {}", input));

  let amt = raw_amt
    .parse::<i32>()
    .unwrap_or_else(|_| panic!("Unable to parse {} int an i32", raw_amt));

  match raw_dir {
    "forward" => SubmarineMovement {
      dir: Direction::Forward,
      amt,
    },
    "down" => SubmarineMovement {
      dir: Direction::Down,
      amt,
    },
    "up" => SubmarineMovement {
      dir: Direction::Up,
      amt,
    },
    _ => panic!("I do not know about the {} direction", raw_dir),
  }
}

pub fn run_a() {
  let data = file_to_vec("data/02_input.txt", make_direction);
  let mut pos = 0;
  let mut depth = 0;

  for movement in data {
    match movement.dir {
      Direction::Forward => pos += movement.amt,
      Direction::Up => depth -= movement.amt,
      Direction::Down => depth += movement.amt,
    }
  }

  println!(
    "Day 02a: pos = {}, depth = {}, ans = {}",
    pos,
    depth,
    pos * depth
  );
}

pub fn run_b() {
  let data = file_to_vec("data/02_input.txt", make_direction);
  let mut pos = 0;
  let mut depth = 0;
  let mut aim = 0;

  for movement in data {
    match movement.dir {
      Direction::Forward => {
        pos += movement.amt;
        depth += aim * movement.amt;
      }
      Direction::Up => aim -= movement.amt,
      Direction::Down => aim += movement.amt,
    }
  }

  println!(
    "Day 02b: pos = {}, depth = {}, aim = {}, ans = {}",
    pos,
    depth,
    aim,
    pos * depth
  );
}
