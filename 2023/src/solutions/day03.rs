use aoc2023::util;
use std::collections::HashSet;
use std::error::Error;

pub fn solve(fname: &str, _which: util::Which) -> Result<(), Box<dyn Error>> {
  let lines = util::read_file(fname)?.flatten();
  let mut part_numbers: Vec<PartNumber> = Vec::new();
  let mut symbol_locations: HashSet<Coord> = HashSet::new();
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
        _symbol => {
          if !digits.is_empty() {
            let part_number = parse_part_number(row, col - 1, &digits)?;
            part_numbers.push(part_number);
            digits = Vec::new();
          }
          update_symbol_locations_set(row, col, &mut symbol_locations)
        }
      }
    }
    if !digits.is_empty() {
      let part_number = parse_part_number(row, line.len() - 1, &digits)?;
      part_numbers.push(part_number);
    }
  }

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
