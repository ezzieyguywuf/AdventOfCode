mod solutions;
use aoc2023::util;
use std::env;
use std::error::Error;

static DAYS: [i32; 3] = [1, 2, 3];

fn main() -> Result<(), Box<dyn Error>> {
  let mut args: Vec<String> = env::args().collect();
  let _program_name = args.remove(0);
  let which_days: Vec<i32> = match util::get_arg("day", &mut args) {
    Ok(days) => {
      let mut parsed_days: Vec<i32> = Vec::new();
      for day_string in days.split(',') {
        let day = day_string.parse::<i32>()?;

        if !DAYS.contains(&day) {
          return Err(
            util::Error::InvalidArgument(format!("Day must be one of {DAYS:?}, got {day}")).into(),
          );
        }
        parsed_days.push(day);
      }

      Ok::<Vec<i32>, util::Error>(parsed_days)
    }
    _ => Err(util::Error::InvalidArgument(format!("Unable to fetch '--day' flag")).into()),
  }?;

  for day in which_days {
    match day {
      1 => {
        let fname = util::get_arg("day01_data", &mut args)?;
        solutions::day01::part_a(&fname)?;
        solutions::day01::part_b(&fname)?;
      }
      2 => {
        let fname = util::get_arg("day02_data", &mut args)?;
        solutions::day02::part_a(&fname)?;
        solutions::day02::part_b(&fname)?;
      }
      3 => {
        let fname = util::get_arg("day03_data", &mut args)?;
        solutions::day03::solve(&fname, util::Which::Both)?;
      }
      n => println!("Sorry, don't know what to do with day '{n}' yet'"),
    }
  }

  if !args.is_empty() {
    println!("WARNING: these arguments were not used, {args:#?}");
  }

  Ok(())
}
