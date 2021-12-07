use advent_of_code::util::*;

pub fn run_a() {
  let mut vals = parse();
  run_simulation(80, &mut vals);
  println!("day06a: ans = {}", vals.len());
}

pub fn run_b() {
  println!("Hello, day06 part b!")
}

fn parse() -> Vec<u32> {
  let lines = file_to_lines("data/06_input.txt");
  lines
    .collect::<String>()
    .split(',')
    .map(|val| {
      val
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("Unable to parse {:?} into u32", val))
    })
    .collect()
}

fn run_simulation(days: u32, data: &mut Vec<u32>) {
  for _ in 0..days {
    let mut new_vals = tick(data);
    data.append(&mut new_vals);
  }
}

fn tick(data: &mut Vec<u32>) -> Vec<u32> {
  let mut out = Vec::new();
  for i in 0..data.len() {
    let val = &mut data[i];
    if *val == 0 {
      *val = 6;
      out.push(8);
    } else {
      *val -= 1;
    }
  }

  out
}
