use advent_of_code::util::*;
pub use std::iter;

#[derive(Debug)]
struct Board {
  data: Vec<u32>,
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
        out.data.push(item);
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

pub fn run_a() {
  let mut lines = file_to_lines("data/04_input.txt").peekable();
  let numbers = parse_numbers(&mut lines);

  for number in numbers.iter().take(4) {
    println!("Got number: {}", number);
  }

  let mut boards: Vec<Board> = Vec::new();
  while lines.peek() != None {
    let board = get_board(&mut lines);
    boards.push(board);
  }

  println!("Got boards: {:?}", boards);
}
