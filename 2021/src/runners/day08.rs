use advent_of_code::util::*;
use std::collections::HashMap;

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
  let datas = parse();
  let mut ans = 0;
  for ExperimentData {
    experiments,
    results,
  } in datas
  {
    let combos: HashMap<Arrangement, String> = calculate_combos(&experiments);
    let signals: HashMap<char, Signal> = solve_signals(&experiments, &combos);

    let mut amt = 0;
    const BASE: u32 = 10;
    for (n, result) in results.iter().rev().enumerate() {
      let val: u32 = decode(result, &signals);
      amt += val * BASE.pow(n.try_into().unwrap());
    }

    ans += amt;
  }

  println!("day08b: ans = {}", ans);
}

fn decode(value: &str, signals: &HashMap<char, Signal>) -> u32 {
  let segments = value
    .chars()
    .map(|c| signals.get(&c).unwrap())
    .copied()
    .collect::<Vec<Signal>>();

  if is_permutation(&segments, &ZERO) {
    return 0;
  } else if is_permutation(&segments, &ONE) {
    return 1;
  } else if is_permutation(&segments, &TWO) {
    return 2;
  } else if is_permutation(&segments, &THREE) {
    return 3;
  } else if is_permutation(&segments, &FOUR) {
    return 4;
  } else if is_permutation(&segments, &FIVE) {
    return 5;
  } else if is_permutation(&segments, &SIX) {
    return 6;
  } else if is_permutation(&segments, &SEVEN) {
    return 7;
  } else if is_permutation(&segments, &EIGHT) {
    return 8;
  } else if is_permutation(&segments, &NINE) {
    return 9;
  }
  panic!("{:?} -> {:?} cannot decode these signals", value, segments)
}

fn is_permutation(input: &[Signal], number: &[Signal]) -> bool {
  if input.len() != number.len() {
    return false;
  }

  input
    .iter()
    .all(|signal| number.iter().any(|val| val == signal))
}

const ZERO: [Signal; 6] = [
  Signal::TopRight,
  Signal::BotRight,
  Signal::BotBot,
  Signal::BotLeft,
  Signal::TopLeft,
  Signal::TopTop,
];

const ONE: [Signal; 2] = [Signal::TopRight, Signal::BotRight];

const TWO: [Signal; 5] = [
  Signal::TopTop,
  Signal::TopRight,
  Signal::TopBot,
  Signal::BotLeft,
  Signal::BotBot,
];

const THREE: [Signal; 5] = [
  Signal::TopTop,
  Signal::TopRight,
  Signal::TopBot,
  Signal::BotRight,
  Signal::BotBot,
];

const FOUR: [Signal; 4] = [
  Signal::TopLeft,
  Signal::TopRight,
  Signal::TopBot,
  Signal::BotRight,
];

const FIVE: [Signal; 5] = [
  Signal::TopTop,
  Signal::TopLeft,
  Signal::TopBot,
  Signal::BotRight,
  Signal::BotBot,
];

const SIX: [Signal; 6] = [
  Signal::TopTop,
  Signal::TopLeft,
  Signal::TopBot,
  Signal::BotRight,
  Signal::BotBot,
  Signal::BotLeft,
];

const SEVEN: [Signal; 3] = [Signal::TopTop, Signal::TopRight, Signal::BotRight];

const EIGHT: [Signal; 7] = [
  Signal::TopTop,
  Signal::TopLeft,
  Signal::TopRight,
  Signal::TopBot,
  Signal::BotRight,
  Signal::BotBot,
  Signal::BotLeft,
];

const NINE: [Signal; 6] = [
  Signal::TopTop,
  Signal::TopLeft,
  Signal::TopRight,
  Signal::TopBot,
  Signal::BotRight,
  Signal::BotBot,
];

fn solve_signals(
  experiments: &[String; 10],
  combos: &HashMap<Arrangement, String>,
) -> HashMap<char, Signal> {
  let mut signals: HashMap<char, Signal> = HashMap::new();
  // these unwraps _should_ be safe if our callers are behaving themselves...
  let top = combos
    .get(&Arrangement::Top)
    .unwrap()
    .chars()
    .next()
    .unwrap();
  let one = combos.get(&Arrangement::One).unwrap();
  let hil = combos.get(&Arrangement::HiL).unwrap();
  let lol = combos.get(&Arrangement::LoL).unwrap();
  let target_five_combo = |val: &&String| val.len() == 5 && val.chars().any(|c| c == top);
  let has_substr =
    |target: &String, substr: &String| substr.chars().all(|c| target.chars().any(|d| d == c));
  let solve = |signals: &mut HashMap<char, Signal>,
               experiment: &String,
               left: (char, Signal),
               right: (char, Signal)| {
    if experiment.chars().any(|c| c == left.0) {
      signals.insert(left.0, left.1);
      signals.insert(right.0, right.1);
    } else {
      signals.insert(left.0, right.1);
      signals.insert(right.0, left.1);
    }
  };

  signals.insert(top, Signal::TopTop);
  while signals.len() < 7 {
    for experiment in experiments.iter().filter(target_five_combo) {
      if has_substr(experiment, hil) && !has_value(&signals, Signal::TopRight) {
        // HiL + Top _must_ be a "5" on the 7-segment display. Thus, this
        // gives us enough information to solve the two signals in One
        let left_one = one.chars().next().unwrap();
        let right_one = one.chars().nth(1).unwrap();
        solve(
          &mut signals,
          experiment,
          (left_one, Signal::BotRight),
          (right_one, Signal::TopRight),
        );
      } else if has_substr(experiment, lol) && !has_value(&signals, Signal::TopLeft) {
        // LoL + Top _must_ be a "2" on the 7-segment display. Thus, this
        // gives us enough information to solve the signals for One _and_ HiL
        if !has_value(&signals, Signal::TopRight) {
          let left_one = one.chars().next().unwrap();
          let right_one = one.chars().nth(1).unwrap();
          solve(
            &mut signals,
            experiment,
            (left_one, Signal::TopRight),
            (right_one, Signal::BotRight),
          );
        }
        let left_hil = hil.chars().next().unwrap();
        let right_hil = hil.chars().nth(1).unwrap();
        solve(
          &mut signals,
          experiment,
          (left_hil, Signal::TopBot),
          (right_hil, Signal::TopLeft),
        );
      } else if has_substr(experiment, one) && !has_value(&signals, Signal::BotLeft) {
        if !has_value(&signals, Signal::TopLeft) {
          let left_hil = hil.chars().next().unwrap();
          let right_hil = hil.chars().nth(1).unwrap();
          solve(
            &mut signals,
            experiment,
            (left_hil, Signal::TopBot),
            (right_hil, Signal::TopLeft),
          );
        }
        if !has_value(&signals, Signal::BotLeft) {
          let left_lol = lol.chars().next().unwrap();
          let right_lol = lol.chars().nth(1).unwrap();
          solve(
            &mut signals,
            experiment,
            (left_lol, Signal::BotBot),
            (right_lol, Signal::BotLeft),
          );
        }
      }
    }
  }

  signals
}

fn has_key<K, V>(map: &HashMap<K, V>, check: K) -> bool
where
  K: Eq,
{
  map.keys().any(|val| *val == check)
}

fn has_value<K, V>(map: &HashMap<K, V>, check: V) -> bool
where
  V: Eq,
{
  map.values().any(|val| *val == check)
}

fn calculate_combos(experiments: &[String; 10]) -> HashMap<Arrangement, String> {
  let mut combos: HashMap<Arrangement, String> = HashMap::new();

  // I don't know if this outer while is strictly necessary, but in case the
  // order of our input ends up requiring us to go through more than once.
  while combos.len() < 4 {
    for experiment in experiments {
      let has_top = has_key(&combos, Arrangement::Top);
      let has_one = has_key(&combos, Arrangement::One);
      let has_hil = has_key(&combos, Arrangement::HiL);

      if experiment.len() == 2 && !has_one {
        combos.insert(Arrangement::One, experiment.to_string());
      } else if experiment.len() == 3 && has_one {
        let one_vals = combos.get(&Arrangement::One).unwrap();
        let val = experiment
          .chars()
          .filter(|c| !one_vals.chars().any(|val| val == *c))
          .collect::<String>();
        combos.insert(Arrangement::Top, val);
      } else if experiment.len() == 4 && has_one {
        let one_vals = combos.get(&Arrangement::One).unwrap();
        let vals = experiment
          .chars()
          .filter(|c| !one_vals.chars().any(|val| val == *c))
          .collect::<String>();

        combos.insert(Arrangement::HiL, vals);
      } else if experiment.len() == 7 && has_hil && has_top && has_one {
        let hil_vals = combos.get(&Arrangement::HiL).unwrap();
        let one_vals = combos.get(&Arrangement::One).unwrap();
        // these unwraps are safe due to the logic encapsulated in the if/then
        // boolean expressions
        let top_val = combos
          .get(&Arrangement::Top)
          .unwrap()
          .chars()
          .next()
          .unwrap();
        let vals = experiment
          .chars()
          .filter(|c| !one_vals.chars().any(|val| val == *c || *c == top_val))
          .filter(|c| !hil_vals.chars().any(|val| val == *c))
          .collect::<String>();
        combos.insert(Arrangement::LoL, vals);
      }

      if combos.len() == 4 {
        return combos;
      }
    }
  }

  combos
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

// These correspond to the various fundamental arrangements of the 7-digit
// display that can be used to solve the problem. In the diagrams below, the "a"
// designates the segments that each combination comprises of.
//
//
//   "Top"     "One"    "HiL"      "LoL"
//
//   aaaa      ----      ----      ----
//  |    |    |    a    a    |    |    |
//  |    |    |    a    a    |    |    |
//   ----      ----      aaaa      ----
//  |    |    |    a    |    |    a    |
//  |    |    |    a    |    |    a    |
//   ----      ----      ----      aaaa
#[derive(Debug, PartialEq, Eq, Hash)]
enum Arrangement {
  Top,
  One,
  HiL,
  LoL,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
enum Signal {
  TopLeft,
  TopTop,
  TopRight,
  TopBot,
  BotLeft,
  BotRight,
  BotBot,
}
