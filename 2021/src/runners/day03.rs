use advent_of_code::util::*;

struct Column {
  zeroes: u32,
  ones: u32,
}

pub fn run_a() {
  let lines = file_to_lines("data/03_input.txt");
  let mut columns: Vec<Column> = Vec::new();

  for line in lines {
    for (col, c) in line.chars().rev().enumerate() {
      if columns.len() <= col {
        columns.push(Column { zeroes: 0, ones: 0 });
      }

      match c {
        '0' => columns[col].zeroes += 1,
        '1' => columns[col].ones += 1,
        _ => panic!("Unrecognized character '{}' - should be '0' or '1'", c),
      }
    }
  }

  let mut epsilon = 0;
  let mut gamma = 0;

  for (col, Column { zeroes, ones }) in columns.iter().enumerate() {
    if ones > zeroes {
      gamma += u32::pow(2, col.try_into().unwrap());
    } else {
      epsilon += u32::pow(2, col.try_into().unwrap());
    }
  }

  println!(
    "day03a: epsilon = {}, gamma = {}, ans = {}",
    epsilon,
    gamma,
    epsilon * gamma
  );
}
