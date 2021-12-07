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
  runners::day04::run_b();
  runners::day05::run_a();
  runners::day05::run_b();
  runners::day06::run_a();
  runners::day06::run_b();
  runners::day07::run_a();
  runners::day07::run_b();

  Ok(())
}
