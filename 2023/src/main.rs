use std::env;

static DAYS: [i32; 1] = [1];

fn get_arg(target: &str, args: &mut Vec<String>) -> Option<String> {
    let mut target_arg: String = String::from("--");
    target_arg.push_str(target);

    args.iter()
        .position(|arg| arg.starts_with(&target_arg))
        .map(|i| {
            let arg = args.remove(i);
            arg.trim_start_matches(&target_arg).to_string()
        })
}

#[derive(Debug)]
enum Error {
    InvalidArgument,
    MissingArgument,
}

fn main() -> Result<(), Error> {
    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.remove(0);
    let which_days: Vec<i32> = match args.iter().position(|arg| arg.starts_with("--day")) {
        Some(i) => {
            let arg = get_arg("day", &mut args).unwrap();
            let days: String = if arg.chars().next().unwrap() == '=' {
                arg.trim_start_matches('=').to_string()
            } else {
                if args.len() <= i {
                    println!("Please provide a comma-delimited list of days after --day");
                    return Err(Error::InvalidArgument);
                }
                args.remove(i)
            };

            let mut parsed_days: Vec<i32> = Vec::new();
            for day_string in days.split(',') {
                let maybe_day = day_string.parse::<i32>();
                if maybe_day.is_err() {
                    println!("Could not parse {day_string} into an integer.");
                    return Err(Error::InvalidArgument);
                };
                let day = maybe_day.unwrap();
                if !DAYS.contains(&day) {
                    println!("Day must be one of {DAYS:?}. Got {day}.");
                    return Err(Error::InvalidArgument);
                }
                parsed_days.push(day);
            }

            parsed_days
        }
        None => DAYS.to_vec(),
    };

    for day in which_days {
        match day {
            1 => {
                let fname = get_arg("day01_data", &mut args);
                if fname.is_none() {
                    println!("Please provide a --day01_data flag");
                    return Err(Error::MissingArgument);
                };
                println!("Got fname: {fname:?}");
            }
            n => println!("Sorry, don't know what to do with day '{n}' yet'"),
        }
    }

    println!("WARNING: these arguments were not used, {args:#?}");

    Ok(())
}
