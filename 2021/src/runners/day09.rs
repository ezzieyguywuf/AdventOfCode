use advent_of_code::util::*;

pub fn run_a() {
  let data = parse();
  for n in data {
    println!("{}", n);
  }
  println!("day09a: ans = {}", 42);
}

pub fn run_b() {
  println!("day09b: ans = {}", 42);
}

fn parse() -> Vec<u32> {
  let lines = file_to_lines("data/test.txt");
  let mut out: Vec<u32> = Vec::new();

  for line in lines {
    for c in line.chars() {
      let val = c
        .to_digit(10)
        .unwrap_or_else(|| panic!("unable to parse {} into a u32", c));
      out.push(val);
    }
  }

  out
}
