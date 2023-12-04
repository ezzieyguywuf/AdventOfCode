use aoc2023::util;
use std::io;

pub fn part_a(fname: &str) -> io::Result<()> {
  let lines = util::read_file(fname)?;
  let mut total = 0;
  for line in lines {
    if let Ok(input) = line{
      let processed = process_line(&input);
      total += processed;
    } else {
      println!("Unable to read line.");
      return Err(io::ErrorKind::InvalidInput.into());
    }
  }

  println!("Day01, part a: {total}");
  Ok(())
}

pub fn process_line(line: &str) -> u32 {
  let mut tens_digit = None;
  let mut ones_digit = None;

  for character in line.chars() {
    if let Some(digit) = character.to_digit(10) {
      tens_digit = Some(digit);
      break;
    }
  }

  for character in line.chars().rev() {
    if let Some(digit) = character.to_digit(10) {
      ones_digit = Some(digit);
      break;
    }
  }

  10 * tens_digit.unwrap() + ones_digit.unwrap()
}
