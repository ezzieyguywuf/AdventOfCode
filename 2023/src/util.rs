use std::fmt;
use std::fs::File;
use std::{fs, io, io::BufRead};

#[derive(Debug)]
pub enum Error {
  InvalidArgument(String),
  MissingArgument(String),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::InvalidArgument(msg) => write!(f, "{msg}"),
      Error::MissingArgument(msg) => write!(f, "{msg}"),
    }
  }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Which {
  PartA,
  PartB,
  Both,
}

pub fn parse_int(n: &str) -> io::Result<u32> {
  match n.parse::<u32>() {
    Ok(val) => Ok(val),
    Err(_) => {
      eprintln!("Unable to parse {n} into an integer");
      Err(std::io::ErrorKind::InvalidInput.into())
    }
  }
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
