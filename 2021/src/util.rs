pub use std::{fs, io};

pub fn file_to_vec<F, T>(fname: &str, f: F) -> io::Result<Vec<T>>
where
  F: Fn(&str) -> T,
{
  Ok(fs::read_to_string(fname)?.lines().map(f).collect())
}

pub fn file_to_ints(fname: &str) -> io::Result<Vec<i32>> {
  file_to_vec(fname, |x| {
    x.parse::<i32>()
      .unwrap_or_else(|_| panic!("Unable to parse value into integer: {}", x))
  })
}
