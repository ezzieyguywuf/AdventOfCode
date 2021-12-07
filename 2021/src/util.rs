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
