mod runners;
use std::io;

fn main() -> io::Result<()> {
  runners::day01::run_a();
  runners::day01::run_b();
  runners::day02::run_a();
  runners::day02::run_b();
  runners::day03::run_a();
  runners::day03::run_b();
  runners::day04::run_a();

  Ok(())
}
