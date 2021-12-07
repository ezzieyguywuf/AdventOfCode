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
  println!("day06a: ans = {}", data.values().sum::<u32>());
}

pub fn run_b() {
  // let mut vals = parse();
  // run_simulation(10, &mut vals);
  // println!("day06b: ans = {}", vals.len());
  println!("day06b: hello!")
}

fn parse() -> HashMap<u32, u32> {
  let line = file_to_lines("data/test.txt").collect::<String>();
  let vals = line.split(',').map(|val| {
    val
      .parse::<u32>()
      .unwrap_or_else(|_| panic!("Unable to parse {:?} into u32", val))
  });

  let mut map = HashMap::new();
  for val in vals {
    let amt = map.entry(val).or_insert(0);
    *amt += 1;
  }

  map
}

fn run_simulation(days: u32, data: &mut HashMap<u32, u32>) {
  for _ in 0..days {
    tick(data);
  }
}

fn tick(data: &mut HashMap<u32, u32>) {
  let mut prev: u32 = 0;
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
