use advent_of_code::util::*;

const INPUT_FILE: &str = "data/test_day11.txt";

pub fn run_a() {
  let mut matrix = parse();
  let mut ans = 0;

  println!("befor...");
  for row in 0..10 {
    for col in 0..10 {
      let index = 10 * row + col;
      let val = matrix.data[index];
      print!("{}, ", val);
    }
    println!();
  }
  for step in 0..4 {
    println!("  step {}", step);
    ans = tick(&mut matrix);
  }

  println!("after....");
  for row in 0..10 {
    for col in 0..10 {
      let index = 10 * row + col;
      let val = matrix.data[index];
      print!("{}, ", val);
    }
    println!();
  }

  println!("day10a: ans = {}", ans);
}

fn tick(matrix: &mut Matrix) -> u32 {
  let mut to_flash: Vec<usize> = Vec::new();
  let mut has_flashed: Vec<usize> = Vec::new();
  let mut flashed = 0;

  for (index, c) in matrix.data.iter_mut().enumerate() {
    *c += 1;
    if *c > 9 && !has_flashed.contains(&index) {
      to_flash.push(index);
      flashed += 1;
    }
  }

  loop {
    println!("to_flash: {:?}", to_flash);
    flashed += flash(matrix, &mut to_flash, &mut has_flashed);
    println!("has_flashed: {:?}", has_flashed);

    if to_flash.is_empty() {
      break;
    }
  }

  for flashed in &has_flashed {
    matrix.data[*flashed] = 0;
  }

  flashed
}

pub fn flash(matrix: &mut Matrix, to_flash: &mut Vec<usize>, has_flashed: &mut Vec<usize>) -> u32 {
  let mut new_flash: Vec<usize> = Vec::new();
  let mut flashed: u32 = 0;
  while let Some(index) = to_flash.pop() {
    has_flashed.push(index);
    for neighbor in get_surrounding(index) {
      let val = matrix.data.get_mut(neighbor).unwrap();
      *val += 1;
      if *val > 9 && !has_flashed.contains(&neighbor) {
        new_flash.push(neighbor);
        flashed += 1;
      }
    }
  }

  to_flash.append(&mut new_flash);
  flashed
}

//    0    1    2    3    4    5    6    7    8    9
//    10   11   12   13   14   15   16   17   18   19
//    20   21   22   23   24   25   26   27   28   29
pub fn get_surrounding(index: usize) -> Vec<usize> {
  let row = index / 10;
  let col = index - 10 * row;
  let mut indices: Vec<usize> = Vec::new();

  for row_adder in -1..2 {
    for col_adder in -1..2 {
      if row_adder == 0 && col_adder == 0 {
        continue;
      }
      let new_row: i32 = i32::try_from(row).unwrap() + row_adder;
      let new_col: i32 = i32::try_from(col).unwrap() + col_adder;
      if is_valid(new_row) && is_valid(new_col) {
        indices.push((10 * new_row + new_col).try_into().unwrap());
      }
    }
  }
  indices
}

pub fn is_valid(val: i32) -> bool {
  (0..10).contains(&val)
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
