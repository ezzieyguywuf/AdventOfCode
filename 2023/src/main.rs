use std::env;
use std::io;

static DAYS: [i32; 1] = [1];

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let _program_name = args.remove(0);
    let _which_days: Vec<i32> = match args.iter().position(|arg| arg.starts_with("--day")) {
        Some(i) => {
            let arg = args.remove(i);
            let trimmed = arg.trim_start_matches("--day");
            let days: String = if trimmed.chars().next().unwrap() == '=' {
                trimmed.trim_start_matches('=').to_string()
            } else {
                if args.len() <= i {
                    panic!("Please provide a comma-delimited list of days after --day")
                }
                args.remove(i)
            };

            days.split(",")
                .map(|day_string| {
                    println!("got day_string: {day_string}");
                    let day = day_string.parse::<i32>().unwrap();
                    if !DAYS.contains(&day) {
                        panic!("Day must be one of {DAYS:?}. Got {day_string}.");
                    }
                    day
                })
                .collect()
        }
        None => DAYS.to_vec(),
    };

    println!("WARNING: these arguments were not used, {args:#?}");
    Ok(())
}
