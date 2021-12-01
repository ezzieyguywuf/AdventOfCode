use std::{fs, io};

pub fn run_day_01() -> io::Result<()> {
  let file_data = fs::read_to_string("src/01_input.txt")?;

  for line in file_data.lines() {
    println!("{}", line);
  }

  Ok(())
}
