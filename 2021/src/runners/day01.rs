use advent_of_code::util::*;

pub fn run_a() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0];

  for val in &data[1..] {
    if *val > prev {
      count += 1;
    }

    prev = *val;
  }

  println!(
    "Day 01a: There were {} measurements greater than the previous",
    count
  );

  Ok(())
}

pub fn run_b() -> io::Result<()> {
  let data = file_to_ints("src/01_input.txt")?;

  let mut count = 0;
  let mut prev = data[0] + data[1] + data[2];
  let sub = &data[1..];

  for (i, val) in sub.iter().enumerate() {
    if i + 2 < sub.len() {
      let cur = val + sub[i + 1] + sub[i + 2];
      if cur > prev {
        count += 1;
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
