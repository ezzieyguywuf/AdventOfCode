use std::env;

static DAYS: [i32; 1] = [1];

fn get_arg(target: &str, args: &mut Vec<String>) -> Result<String, Error> {
    let mut target_arg: String = String::from("--");
    target_arg.push_str(target);

    args.iter()
        .position(|arg| arg.starts_with(&target_arg))
        .map_or_else(
            || {
                println!("Please provide a --{target} argument");
                Err(Error::MissingArgument)
            },
            |i| {
                let arg = args.remove(i).trim_start_matches(&target_arg).to_string();
                let value: String = if arg.starts_with('=') {
                    arg.trim_start_matches('=').to_string()
                } else {
                    if args.len() <= i {
                        println!("Please provide a value for the {target} argument");
                        return Err(Error::InvalidArgument);
                    }
                    args.remove(i)
                };
                Ok(value)
            },
        )
}

#[derive(Debug)]
enum Error {
    InvalidArgument,
    MissingArgument,
}

fn main() -> Result<(), Error> {
    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.remove(0);
    let which_days: Vec<i32> = get_arg("day", &mut args).map(|days| {
        let mut parsed_days: Vec<i32> = Vec::new();
        for day_string in days.split(',') {
            let day = match day_string.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Could not parse {day_string} into an integer.");
                    return Err(Error::InvalidArgument);
                }
            };
            if !DAYS.contains(&day) {
                println!("Day must be one of {DAYS:?}. Got {day}.");
                return Err(Error::InvalidArgument);
            }
            parsed_days.push(day);
        }

        Ok(parsed_days)
    })??;

    for day in which_days {
        match day {
            1 => {
                let fname = get_arg("day01_data", &mut args)?;
                println!("Got fname: {fname:?}");
            }
            n => println!("Sorry, don't know what to do with day '{n}' yet'"),
        }
    }

    if args.len() > 0 {
        println!("WARNING: these arguments were not used, {args:#?}");
    }

    Ok(())
}
