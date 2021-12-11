pub use std::{fs, io, io::BufRead, iter};

pub fn file_to_lines(fname: &str) -> impl Iterator<Item = String> {
  let file = fs::File::open(fname).unwrap_or_else(|_| panic!("Unable to open file: {}", fname));
  io::BufReader::new(file)
    .lines()
    .map(|val| val.expect("Unable to read line from file"))
}

pub fn file_to_vec<F, T>(fname: &str, f: F) -> Vec<T>
where
  F: Fn(&str) -> T,
{
  file_to_lines(fname).map(|s| f(s.as_str())).collect()
}

pub fn file_to_ints(fname: &str) -> Vec<i32> {
  file_to_vec(fname, |x| {
    x.parse::<i32>()
      .unwrap_or_else(|e| panic!("Unable to parse value into integer: {}", e))
  })
}

pub fn file_line_to_uints(fname: &str) -> Vec<u32> {
  let line = file_to_lines(fname).collect::<String>();
  line
    .split(',')
    .map(|val| {
      val
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Unable to parse {:?} into u64", val))
    })
    .collect()
}

#[derive(Debug)]
struct Matrix {
  cols: usize,
  rows: usize,
  data: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
struct Coord {
  row: usize,
  col: usize,
}

impl Matrix {
  fn get_cell(&self, coord: &Coord) -> Option<&u32> {
    self.make_index(coord).map(|i| &self.data.as_slice()[i])
  }

  fn make_index(&self, coord: &Coord) -> Option<usize> {
    if coord.row >= self.rows || coord.col >= self.cols {
      return None;
    }
    Some(self.cols * coord.row + coord.col)
  }

  fn out_of_bounds_index(&self) -> usize {
    self.data.len()
  }
}
