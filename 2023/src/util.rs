use std::fs::File;
use std::{fs, io, io::BufRead};

#[derive(Debug)]
pub enum Error {
  InvalidArgument,
  MissingArgument,
}

pub fn read_file(fname: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
  let file = fs::File::open(fname)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn get_arg(target: &str, args: &mut Vec<String>) -> io::Result<String> {
  let mut target_arg: String = String::from("--");
  target_arg.push_str(target);

  Ok(
    args
      .iter()
      .position(|arg| arg.starts_with(&target_arg))
      .map_or_else(
        || {
          println!("Please provide a --{target} argument");
          Err(io::ErrorKind::InvalidInput)
        },
        |i| {
          let arg = args.remove(i).trim_start_matches(&target_arg).to_string();
          let value: String = if arg.starts_with('=') {
            arg.trim_start_matches('=').to_string()
          } else {
            if args.len() <= i {
              println!("Please provide a value for the {target} argument");
              return Err(io::ErrorKind::InvalidInput);
            }
            args.remove(i)
          };
          Ok(value)
        },
      )?,
  )
}
