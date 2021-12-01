use std::{fs, io};

fn file_to_ints(fname: &str) -> io::Result<std::vec::Vec<i32>> {
  let file_data = fs::read_to_string(fname)?;
  let lines = file_data.lines().map(|val| {
    val
      .parse::<i32>()
      .expect(format!("Unable to parse value into integer: {}", val).as_str())
  });
  let mut out = std::vec::Vec::new();

  for val in lines {
    out.push(val);
  }

  return Ok(out);
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
  let data = file_to_ints("src/02_input.txt")?;

  let mut count = 0;
  let mut prev = data[0] + data[1] + data[2];
  let sub = &data[1..];

  for (i, val) in sub.iter().enumerate() {
    if i + 2 < sub.len() {
      let cur = val + sub[i + 1] + sub[i + 2];
      if cur > prev {
        count = count + 1;
      }

      println!(
        "For i = {}, prev = {}, cur = {}, count = {}",
        i, prev, cur, count
      );
      prev = cur;
    }
  }

  println!(
    "Day 01b: There were {} windows greater than the previous",
    count
  );

  Ok(())
}
