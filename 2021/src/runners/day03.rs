use advent_of_code::util::*;

struct Column {
  zeroes: u32,
  ones: u32,
}

// Calculates how many zeroes and ones are in each column of the rows
fn summarize_rows<T>(rows: T) -> Vec<Column>
where
  T: Iterator<Item = String>,
{
  let mut columns: Vec<Column> = Vec::new();

  for line in rows {
    // We reverse this so that the far-right column is the zeroeth column. This
    // helps when we're calculating the binary value later
    for (col, c) in line.chars().rev().enumerate() {
      if columns.len() <= col {
        columns.push(Column { zeroes: 0, ones: 0 });
      }

      match c {
        '0' => columns[col].zeroes += 1,
        '1' => columns[col].ones += 1,
        _ => panic!("Unrecognized character '{}' - should be '0' or '1'", c),
      }
    }
  }

  columns
}

pub fn run_a() {
  let lines = file_to_lines("data/03_input.txt");
  let columns = summarize_rows(lines);

  let mut epsilon = 0;
  let mut gamma = 0;

  for (col, Column { zeroes, ones }) in columns.iter().enumerate() {
    // since the column refers to the "two"s place (per the reverse above), we
    // can directly calculate how much to add
    if ones > zeroes {
      gamma += u32::pow(2, col.try_into().unwrap());
    } else {
      epsilon += u32::pow(2, col.try_into().unwrap());
    }
  }

  println!(
    "day03a: epsilon = {}, gamma = {}, ans = {}",
    epsilon,
    gamma,
    epsilon * gamma
  );
}

fn filter_answer<F>(size: usize, mut vals: Vec<String>, f: F) -> String
where
  F: Fn(u32, u32) -> char,
{
  for col in (0..size).rev() {
    let new_columns = summarize_rows(vals.iter().cloned());

    let Column { zeroes, ones } = new_columns[col];
    let val = f(zeroes, ones);

    vals = vals
      .iter()
      .filter(|line| line.chars().rev().nth(col) == Some(val))
      .cloned()
      .collect::<Vec<String>>();

    if vals.len() == 1 {
      break;
    }
  }

  vals
    .pop()
    .expect("There should have been at least one value left...")
}

fn binary_text_to_int(input: &str) -> i32 {
  let mut out = 0;

  for (col, val) in input.chars().rev().enumerate() {
    if val == '1' {
      out += i32::pow(2, col.try_into().unwrap());
    }
  }

  out
}

pub fn run_b() {
  let fname = "data/03_input.txt";
  let lines = file_to_lines(fname);
  let columns = summarize_rows(lines);
  let o2_filter = |zeroes, ones| {
    if zeroes > ones {
      '0'
    } else {
      '1'
    }
  };
  let co2_filter = |zeroes, ones| {
    if zeroes <= ones {
      '0'
    } else {
      '1'
    }
  };

  let o2_lines = file_to_lines(fname).collect::<Vec<String>>();
  let co2_lines = file_to_lines(fname).collect::<Vec<String>>();

  let o2_val = filter_answer(columns.len(), o2_lines, o2_filter);
  let co2_val = filter_answer(columns.len(), co2_lines, co2_filter);

  let o2_dec = binary_text_to_int(o2_val.as_str());
  let co2_dec = binary_text_to_int(co2_val.as_str());

  println!(
    "day03b: o2_val = {}, co2_val = {}, ans = {}",
    o2_dec,
    co2_dec,
    o2_dec * co2_dec
  );
}
