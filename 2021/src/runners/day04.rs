use advent_of_code::util::*;
pub use std::iter;

fn parse_numbers<I: Iterator<Item = String>>(iter: I) -> Vec<u32> {
  iter
    .take_while(|elem| *elem != "")
    .collect::<String>()
    .split(',')
    .map(|elem| {
      elem
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Unable to parse '{}' into u32", elem))
    })
    .collect::<Vec<_>>()
}

pub fn run_a() {
  let lines = file_to_lines("data/04_input.txt");
  let numbers = parse_numbers(lines);

  for number in numbers {
    println!("Got number: {}", number);
  }
}
