use std::{fs, io};

fn file_to_vec<F, T>(fname: &str, f: F) -> io::Result<std::vec::Vec<T>>
where
  F: Fn(&str) -> T,
{
  let mut out = std::vec::Vec::new();

  for val in fs::read_to_string(fname)?.lines().map(f) {
    out.push(val);
  }

  Ok(out)
}

fn file_to_ints(fname: &str) -> io::Result<std::vec::Vec<i32>> {
  file_to_vec(fname, |x| {
    x.parse::<i32>()
      .expect(format!("Unable to parse value into integer: {}", x).as_str())
  })
}

pub fn run_day_01a() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0];

  for val in &data[1..] {
    if *val > prev {
      count = count + 1;
    }

    prev = *val;
  }

  println!(
    "Day 01a: There were {} measurements greater than the previous",
    count
  );

  Ok(())
}

pub fn run_day_01b() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0] + data[1] + data[2];
  let sub = &data[1..];

  for (i, val) in sub.iter().enumerate() {
    if i + 2 < sub.len() {
      let cur = val + sub[i + 1] + sub[i + 2];
      if cur > prev {
        count = count + 1;
      }

      prev = cur;
    }
  }

  println!(
    "Day 01b: There were {} windows greater than the previous",
    count
  );

  Ok(())
}
