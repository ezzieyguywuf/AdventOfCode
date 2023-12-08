use aoc2023::util;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn solve(fname: &str, which: util::Which) -> Result<(), Box<dyn Error>> {
  let lines = util::read_file(fname)?.flatten();
  let mut part_numbers: Vec<PartNumber> = Vec::new();
  let mut symbol_locations: HashSet<Coord> = HashSet::new();
  let mut gears = HashMap::new();

  for (row, line) in lines.enumerate() {
    let mut digits: Vec<char> = Vec::new();
    for (col, c) in line.chars().enumerate() {
      match c {
        '.' => {
          if !digits.is_empty() {
            let part_number = parse_part_number(row, col - 1, &digits)?;
            part_numbers.push(part_number);
            digits = Vec::new();
          }
        }
        '0'..='9' => digits.push(c),
        symbol => {
          if !digits.is_empty() {
            let part_number = parse_part_number(row, col - 1, &digits)?;
            part_numbers.push(part_number);
            digits = Vec::new();
          }
          update_symbol_locations_set(row, col, &mut symbol_locations);

          if symbol == '*' {
            let coord = Coord { row, col };
            gears.insert(coord, Vec::new());
          }
        }
      }
    }
    if !digits.is_empty() {
      let part_number = parse_part_number(row, line.len() - 1, &digits)?;
      part_numbers.push(part_number);
    }
  }

  if which == util::Which::Both || which == util::Which::PartA {
    let part_a: u32 = part_numbers
      .iter()
      .filter_map(|part_number| {
        for coord in &part_number.coords {
          if symbol_locations.contains(&coord) {
            return Some(part_number.number);
          }
        }

        None
      })
      .sum();
    println!("Day 03, part a, total: {part_a}");
  }

  if which == util::Which::Both || which == util::Which::PartB {
    for part_number in part_numbers {
      let mut coords_to_check = HashSet::new();

      for coord in &part_number.coords {
        let mut row_vals = vec![coord.row, coord.row + 1];
        if coord.row > 0 {
          row_vals.push(coord.row - 1);
        }

        let mut col_vals = vec![coord.col, coord.col + 1];
        if coord.col > 0 {
          col_vals.push(coord.col - 1);
        }

        for &row in &row_vals {
          for &col in &col_vals {
            coords_to_check.insert(Coord { row, col });
          }
        }
      }

      for coord in &coords_to_check {
        if let Some(n) = gears.get_mut(&coord) {
          n.push(part_number.number);
        }
      }
    }
    let part_b: u32 = gears
      .iter()
      .filter_map(|(_coord, numbers)| {
        if numbers.len() == 2 {
          return Some(numbers[0] * numbers[1]);
        }
        return None;
      })
      .sum();

    println!("Day 03, part b, total: {part_b}");
  }

  Ok(())
}

fn update_symbol_locations_set(row: usize, col: usize, symbol_locations: &mut HashSet<Coord>) {
  let mut row_vals = vec![row, row + 1];
  if row > 0 {
    row_vals.push(row - 1);
  }
  let mut col_vals = vec![col, col + 1];
  if col > 0 {
    col_vals.push(col - 1);
  }

  for row_val in &row_vals {
    for col_val in &col_vals {
      symbol_locations.insert(Coord {
        row: *row_val,
        col: *col_val,
      });
    }
  }
}

fn parse_part_number(row: usize, col: usize, digits: &Vec<char>) -> util::Result<PartNumber> {
  let mut coords = Vec::new();
  for n in 0..digits.len() {
    let coord = Coord { row, col: col - n };
    coords.push(coord);
  }

  let number = util::parse_int(&digits.into_iter().collect::<String>())?;

  Ok(PartNumber { number, coords })
}

#[derive(Debug)]
struct PartNumber {
  number: u32,
  coords: Vec<Coord>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coord {
  row: usize,
  col: usize,
}
