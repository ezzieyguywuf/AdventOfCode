mod solutions;
use aoc2023::util;
use std::env;
use std::io;

static DAYS: [i32; 2] = [1, 2];

fn main() -> io::Result<()> {
  let mut args: Vec<String> = env::args().collect();
  let _program_name = args.remove(0);
  let which_days: Vec<i32> = util::get_arg("day", &mut args).map(|days| {
    let mut parsed_days: Vec<i32> = Vec::new();
    for day_string in days.split(',') {
      let day = match day_string.parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
          println!("Could not parse {day_string} into an integer.");
          return Err(io::ErrorKind::InvalidInput);
        }
      };
      if !DAYS.contains(&day) {
        println!("Day must be one of {DAYS:?}. Got {day}.");
        return Err(io::ErrorKind::InvalidInput);
      }
      parsed_days.push(day);
    }

    Ok(parsed_days)
  })??;

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
      }
      n => println!("Sorry, don't know what to do with day '{n}' yet'"),
    }
  }

  if !args.is_empty() {
    println!("WARNING: these arguments were not used, {args:#?}");
  }

  Ok(())
}
