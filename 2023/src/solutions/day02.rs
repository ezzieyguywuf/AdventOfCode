use aoc2023::util;
use std::io;

pub fn part_a(fname: &str) -> io::Result<()> {
  let lines = util::read_file(fname)?;
  let tot: u32 = lines
    .into_iter()
    .filter_map(|line| match parse_game(&line.as_ref().ok()?) {
      Ok(game) => {
        if game
          .rolls
          .iter()
          .all(|roll| roll.blue <= 14 && roll.red <= 12 && roll.green <= 13)
        {
          Some(game.which)
        } else {
          None
        }
      }
      _ => {
        eprintln!("Unable to parse '{line:?}' into a Game");
        None
      }
    })
    .sum();

  println!("Day 02, part a, {tot}");

  Ok(())
}

pub fn part_b(fname: &str) -> io::Result<()> {
  let lines = util::read_file(fname)?;
  let mut tot = 0;
  for line in lines.flatten() {
    match parse_game(&line) {
      Ok(game) => {
        let minimums = find_minimum_cubes(&game);
        tot += minimums.red * minimums.blue * minimums.green;
      }
      _ => {
        eprintln!("Unable to parse '{line:?}' into a Game");
        // None
      }
    }
  }

  println!("Day 02, part b, {tot}");
  Ok(())
}

fn find_minimum_cubes(game: &Game) -> Roll {
  let mut red = 0;
  let mut blue = 0;
  let mut green = 0;

  for roll in &game.rolls {
    if roll.red > red {
      red = roll.red;
    }
    if roll.blue > blue {
      blue = roll.blue;
    }
    if roll.green > green {
      green = roll.green;
    }
  }

  Roll { red, blue, green }
}

fn parse_game(line: &str) -> io::Result<Game> {
  match line.split_once(':') {
    Some((prefix, rolls_str)) => {
      let which = match prefix.strip_prefix("Game ") {
        Some(stripped) => parse_int(stripped),
        None => {
          eprintln!("Unable to strip prefix from {prefix}");
          Err(std::io::ErrorKind::InvalidInput.into())
        }
      }?;

      let mut rolls: Vec<Roll> = vec![];
      for roll in rolls_str.trim().split(';') {
        rolls.push(parse_roll(roll)?);
      }

      Ok(Game { which, rolls })
    }
    None => {
      eprintln!("Could not split line '{line}' on ':'");
      Err(std::io::ErrorKind::InvalidInput.into())
    }
  }
}

fn parse_roll(roll: &str) -> io::Result<Roll> {
  let mut blue = 0;
  let mut red = 0;
  let mut green = 0;

  for color in roll.trim().split(',') {
    match color.trim().split_once(' ') {
      Some((n, "blue")) => blue = parse_int(n)?,
      Some((n, "red")) => red = parse_int(n)?,
      Some((n, "green")) => green = parse_int(n)?,
      Some((_, color)) => {
        eprintln!("Do not know how to parse color {color}");
        return Err(std::io::ErrorKind::InvalidInput.into());
      }
      _ => {
        eprintln!("Error parsing roll {roll}");
        return Err(std::io::ErrorKind::InvalidInput.into());
      }
    }
  }

  Ok(Roll { blue, red, green })
}

fn parse_int(n: &str) -> io::Result<u32> {
  match n.parse::<u32>() {
    Ok(val) => Ok(val),
    Err(_) => {
      eprintln!("Unable to parse {n} into an integer");
      Err(std::io::ErrorKind::InvalidInput.into())
    }
  }
}

#[derive(Debug)]
struct Roll {
  blue: u32,
  red: u32,
  green: u32,
}

#[derive(Debug)]
struct Game {
  which: u32,
  rolls: Vec<Roll>,
}
