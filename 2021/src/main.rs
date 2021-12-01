mod runners;

use std::io;

fn main() -> io::Result<()> {
  runners::run_day_01a()?;
  runners::run_day_01b()?;

  Ok(())
}
