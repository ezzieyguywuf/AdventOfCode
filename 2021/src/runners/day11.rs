use advent_of_code::util::*;

const INPUT_FILE: &str = "data/test_day11.txt";

pub fn run_a() {
  let matrix = parse();
  for c in matrix.data {
    println!("{}", c);
  }
  println!("day10a: ans = {}", 42);
}

pub fn run_b() {
  println!("day10b: ans = {}", 42);
}

fn parse() -> Matrix {
  let lines = file_to_lines(INPUT_FILE);
  let mut data: Vec<u32> = Vec::new();
  let mut rows = 0;

  for line in lines {
    rows += 1;
    for c in line.chars() {
      let val = c
        .to_digit(10)
        .unwrap_or_else(|| panic!("unable to parse {} into a u32", c));
      data.push(val);
    }
  }

  let cols = data.len() / rows;
  Matrix { cols, rows, data }
}
