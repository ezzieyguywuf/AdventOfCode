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

pub fn run_day_01() -> io::Result<()> {
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
    "There were {} measurements greater than the previous",
    count
  );

  Ok(())
}
