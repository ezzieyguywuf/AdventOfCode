use aoc2023::util;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub fn solve(fname: &str, which: util::Which) -> Result<(), Box<dyn Error>> {
  let lines = util::read_file(fname)?.flatten();

  for line in lines {
    println!("Got lin: {line}");
  }

  Ok(())
}
