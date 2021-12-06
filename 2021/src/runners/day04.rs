use advent_of_code::util::*;
pub use std::{fmt, iter};

struct Board {
  // None means the player has scored on that cell
  data: Vec<Option<u32>>,
}

impl fmt::Debug for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for row in 0..5 {
      for col in 0..5 {
        let index = make_index(row, col);
        match self.data[index] {
          Some(val) => f.write_fmt(format_args!("{:^3}", val))?,
          None => f.write_fmt(format_args!("{:^3}", "X"))?,
        };
        if col == 4 {
          f.write_str("\n")?;
        }
      }
    }

    f.write_str("")
  }
}

fn get_board(iter: &mut impl Iterator<Item = String>) -> Board {
  let mut out: Board = Board { data: Vec::new() };

  for line in iter {
    if line.is_empty() {
      return out;
    } else {
      let parsed = line.split_ascii_whitespace().map(|el| {
        el.parse::<u32>()
          .unwrap_or_else(|_| panic!("Unable to convert {} to u32. line = {}", el, line))
      });

      for item in parsed {
        out.data.push(Some(item));
      }
    }
  }

  out
}

fn parse_numbers(iter: &mut impl Iterator<Item = String>) -> Vec<u32> {
  iter
    .take_while(|elem| !elem.is_empty())
    .collect::<String>()
    .split(',')
    .map(|elem| {
      elem
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Unable to parse '{}' into u32", elem))
    })
    .collect::<Vec<u32>>()
}

fn update_board(board: &mut Board, val: u32) {
  for number in &mut board.data {
    if *number == Some(val) {
      number.take();
    }
  }
}

fn make_index(row: usize, col: usize) -> usize {
  row * 5 + col
}

fn column_wins(board: &Board, col: usize) -> bool {
  for row in 0..5 {
    let index = make_index(row, col);
    if board.data[index].is_some() {
      return false;
    }
  }

  true
}

fn row_wins(board: &Board, row: usize) -> bool {
  for col in 0..5 {
    let index = make_index(row, col);
    if board.data[index].is_some() {
      return false;
    }
  }

  true
}

fn board_wins(board: &Board) -> bool {
  // first check columns
  for col in 0..5 {
    if column_wins(board, col) {
      return true;
    }
  }

  for row in 0..5 {
    if row_wins(board, row) {
      return true;
    }
  }

  false
}

pub fn run_a() {
  let mut lines = file_to_lines("data/04_input.txt").peekable();
  let numbers = parse_numbers(&mut lines);

  let mut boards: Vec<Board> = Vec::new();
  while lines.peek() != None {
    boards.push(get_board(&mut lines));
  }

  for number in numbers.iter() {
    for mut board in &mut boards {
      update_board(&mut board, *number);
      if board_wins(board) {
        let sum = board
          .data
          .iter()
          .map(|val| val.unwrap_or(0))
          .fold(0, |a, b| a + b);
        println!("sum = {}, number = {}, ans = {}", sum, number, sum * number);
        return;
      }
    }
  }
}
