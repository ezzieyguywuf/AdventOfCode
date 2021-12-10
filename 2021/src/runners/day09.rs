use advent_of_code::util::*;

pub fn run_a() {
  let matrix = parse();
  for row in 0..matrix.rows {
    for col in 0..matrix.cols {
      let val = matrix.get_cell(row, col).unwrap();
      print!("{}", val);
    }
    println!();
  }
  println!("day09a: ans = {}", 42);
}

pub fn run_b() {
  println!("day09b: ans = {}", 42);
}

// sample input:
// 12345\n
// 67890
//
// sample output:
// Matrix { rows: 2, cols: 5, data: [1,2,3,4,5,6,7,8,9,0] }
fn parse() -> Matrix {
  let lines = file_to_lines("data/test.txt");
  let mut data: Vec<u32> = Vec::new();
  let mut first = true;
  let mut rows = 0;
  let mut cols = 0;

  for line in lines {
    rows += 1;
    for c in line.chars() {
      if first {
        cols += 1;
      }
      let val = c
        .to_digit(10)
        .unwrap_or_else(|| panic!("unable to parse {} into a u32", c));
      data.push(val);
    }

    if first {
      first = false;
    }
  }

  Matrix { cols, rows, data }
}

struct Matrix {
  cols: usize,
  rows: usize,
  data: Vec<u32>,
}

impl Matrix {
  fn get_cell(&self, row: usize, col: usize) -> Option<&u32> {
    if row >= self.rows || col >= self.cols {
      return None;
    }
    let index = self.cols * row + col;

    Some(&self.data.as_slice()[index])
  }
}
