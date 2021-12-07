use advent_of_code::util::*;
use std::collections::HashMap;

pub fn run_a() {
  let mut data = parse();
  // println!("parsed");
  // for i in 0..9 {
  //   let val = data.get(&i).unwrap_or(&0);
  //   println!("{}: {}", i, val);
  // }
  run_simulation(80, &mut data);

  // println!("\nsimulated");
  // for i in 0..9 {
  //   let val = data.get(&i).unwrap_or(&0);
  //   println!("{}: {}", i, val);
  // }
  println!("day06a: ans = {}", data.values().sum::<u64>());
}

pub fn run_b() {
  let mut data = parse();
  run_simulation(256, &mut data);
  println!("day06a: ans = {}", data.values().sum::<u64>());
}

fn parse() -> HashMap<u32, u64> {
  let vals = file_line_to_uints("data/06_input.txt");

  let mut map: HashMap<u32, u64> = HashMap::new();
  for val in vals {
    let amt = map.entry(val).or_insert(0);
    *amt += 1;
  }

  map
}

fn run_simulation(days: u32, data: &mut HashMap<u32, u64>) {
  for _ in 0..days {
    tick(data);
  }
}

fn tick(data: &mut HashMap<u32, u64>) {
  let mut prev: u64 = 0;
  for day in (0..9).rev() {
    if day > 0 {
      let today = data.entry(day).or_insert(0);
      std::mem::swap(&mut *today, &mut prev);
    } else {
      let tmp;
      {
        let today = data.entry(day).or_insert(0);
        tmp = *today;
        *today = prev;
      }
      *data.entry(6).or_insert(0) += tmp;
      if tmp > 0 {
        *data.entry(8).or_insert(1) += tmp;
      }
    }
  }
}
