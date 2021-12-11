mod runners;
use std::{io, time};

fn timer(f: impl Fn()) {
  let now = time::Instant::now();
  f();
  println!("  runtime: {}ms", now.elapsed().as_millis());
}

fn main() -> io::Result<()> {
  timer(runners::day01::run_a);
  timer(runners::day01::run_b);
  timer(runners::day02::run_a);
  timer(runners::day02::run_b);
  timer(runners::day03::run_a);
  timer(runners::day03::run_b);
  timer(runners::day04::run_a);
  timer(runners::day04::run_b);
  timer(runners::day05::run_a);
  timer(runners::day05::run_b);
  timer(runners::day06::run_a);
  timer(runners::day06::run_b);
  timer(runners::day07::run_a);
  // timer(runners::day07::run_b);
  timer(runners::day08::run_a);
  timer(runners::day08::run_b);
  timer(runners::day09::run_a);
  timer(runners::day09::run_b);
  timer(runners::day10::run_a);
  timer(runners::day10::run_b);

  Ok(())
}
