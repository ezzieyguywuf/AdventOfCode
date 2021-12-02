use std::{fs, io};

fn file_to_vec<F, T>(fname: &str, f: F) -> io::Result<std::vec::Vec<T>>
where
  F: Fn(&str) -> T,
{
  let mut out = std::vec::Vec::new();

  for val in fs::read_to_string(fname)?.lines().map(f) {
    out.push(val);
  }

  Ok(out)
}

fn file_to_ints(fname: &str) -> io::Result<std::vec::Vec<i32>> {
  file_to_vec(fname, |x| {
    x.parse::<i32>()
      .expect(format!("Unable to parse value into integer: {}", x).as_str())
  })
}

pub fn run_day_01a() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0];

  for val in &data[1..] {
    if *val > prev {
      count = count + 1;
    }

    prev = *val;
  }

  println!(
    "Day 01a: There were {} measurements greater than the previous",
    count
  );

  Ok(())
}

pub fn run_day_01b() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0] + data[1] + data[2];
  let sub = &data[1..];

  for (i, val) in sub.iter().enumerate() {
    if i + 2 < sub.len() {
      let cur = val + sub[i + 1] + sub[i + 2];
      if cur > prev {
        count = count + 1;
      }

      prev = cur;
    }
  }

  println!(
    "Day 01b: There were {} windows greater than the previous",
    count
  );

  Ok(())
}

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
    .expect(format!("Unable to split the string {}", input).as_str());

  let amt = raw_amt
    .parse::<i32>()
    .expect(format!("Unable to parse {} int an i32", raw_amt).as_str());

  match raw_dir {
    "forward" => {
      return SubmarineMovement {
        dir: Direction::Forward,
        amt: amt,
      }
    }
    "down" => {
      return SubmarineMovement {
        dir: Direction::Down,
        amt: amt,
      }
    }
    "up" => {
      return SubmarineMovement {
        dir: Direction::Up,
        amt: amt,
      }
    }
    _ => panic!("I do not know about the {} direction", raw_dir),
  }
}

pub fn run_day_02a() -> io::Result<()> {
  let data = file_to_vec("src/02_input.txt", make_direction)?;
  let mut pos = 0;
  let mut depth = 0;

  for movement in data {
    match movement.dir {
      Direction::Forward => pos = pos + movement.amt,
      Direction::Up => depth = depth - movement.amt,
      Direction::Down => depth = depth + movement.amt,
    }
  }

  println!(
    "Day 02a: pos = {}, depth = {}, ans = {}",
    pos,
    depth,
    pos * depth
  );
  Ok(())
}

pub fn run_day_02b() -> io::Result<()> {
  let data = file_to_vec("src/02_input.txt", make_direction)?;
  let mut pos = 0;
  let mut depth = 0;
  let mut aim = 0;

  for movement in data {
    match movement.dir {
      Direction::Forward => {
        pos = pos + movement.amt;
        depth = depth + aim * movement.amt;
      }
      Direction::Up => aim = aim - movement.amt,
      Direction::Down => aim = aim + movement.amt,
    }
  }

  println!(
    "Day 02b: pos = {}, depth = {}, aim = {}, ans = {}",
    pos,
    depth,
    aim,
    pos * depth
  );
  Ok(())
}
