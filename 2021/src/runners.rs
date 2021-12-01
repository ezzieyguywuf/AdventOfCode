use std::{fs, io};

pub fn run_day_01() -> io::Result<()> {
  let file_data = fs::read_to_string("src/01_input.txt")?;
  let mut lines = file_data.lines().map(|val| {
    val
      .parse::<i32>()
      .expect(format!("Unable to parse value into integer: {}", val).as_str())
  });

  let mut count = 0;
  let mut prev = lines.next().expect("Unable to fetch next value");

  for val in lines {
    if val > prev {
      count = count + 1;
    }

    prev = val;
  }

  println!(
    "There were {} measurements greater than the previous",
    count
  );

  Ok(())
}
