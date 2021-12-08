use advent_of_code::util::*;

pub fn run_a() {
  let datas = parse();
  let lens = vec![2, 3, 4, 7];
  let mut count = 0;

  for ExperimentData { results, .. } in datas {
    for result in results {
      if lens.contains(&result.len()) {
        count += 1;
      }
    }
  }
  println!("day08a: ans = {}", count);
}

pub fn run_b() {
  println!("day08b: ans = {}", 42);
}

fn parse() -> Vec<ExperimentData> {
  let lines = file_to_lines("data/08_input.txt");
  lines
    .map(|line| {
      let (raw_experiments, raw_results) = line
        .split_once(" | ")
        .unwrap_or_else(|| panic!("Invalid input line '{}'. Does not contain ' | '", line));

      const INIT: String = String::new();
      let mut experiments = [INIT; 10];
      let mut results = [INIT; 4];

      for (spot, value) in experiments
        .iter_mut()
        .zip(raw_experiments.split_ascii_whitespace())
      {
        *spot = value.to_owned();
      }

      for (spot, value) in results.iter_mut().zip(raw_results.split_ascii_whitespace()) {
        *spot = value.to_owned();
      }

      ExperimentData {
        experiments,
        results,
      }
    })
    .collect::<Vec<_>>()
}

struct ExperimentData {
  experiments: [String; 10],
  results: [String; 4],
}
