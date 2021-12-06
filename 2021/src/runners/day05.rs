use advent_of_code::util::*;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run_a() {
  let lines = file_to_lines("data/test.txt");

  for line in lines {
    let val = line.split_ascii_whitespace().take(1).collect::<String>();
    println!("{:?}", val.parse::<Point>().unwrap());
  }
}

pub fn run_b() {
  println!("Hello day05, part b!")
}

#[derive(Debug)]
struct Point {
  x: u32,
  y: u32,
}

impl FromStr for Point {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coords: Vec<&str> = s.split(',').collect();
    let x_parsed = coords[0].parse::<u32>()?;
    let y_parsed = coords[1].parse::<u32>()?;
    Ok(Point {
      x: x_parsed,
      y: y_parsed,
    })
  }
}
