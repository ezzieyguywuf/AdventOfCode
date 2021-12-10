use advent_of_code::util::*;

pub fn run_a() {
  let data = parse();
  let lower = *data.iter().min().unwrap_or_else(|| panic!("empty input"));
  let upper = *data.iter().max().unwrap_or_else(|| panic!("empty input"));
  let mid = (lower + upper) / 2;
  let mut old_range = SearchRange { lower, upper, mid };
  let mut old_cost = get_cost(mid, &data);

  loop {
    let (new_range, new_cost) = find_new_mid(&old_range, &data, get_cost);
    if new_cost > old_cost {
      break;
    } else {
      old_range = new_range;
      old_cost = new_cost;
    }
  }

  for val in old_range.lower..old_range.upper {
    let cost = get_cost(val, &data);
    if cost < old_cost {
      old_cost = cost
    }
  }

  println!("day07a: ans = {}", old_cost);
}

pub fn _run_b() {
  let data = parse();
  let lower = *data.iter().min().unwrap_or_else(|| panic!("empty input"));
  let upper = *data.iter().max().unwrap_or_else(|| panic!("empty input"));
  let mid = (lower + upper) / 2;
  let mut old_range = SearchRange { lower, upper, mid };
  let mut old_cost = _get_cost_expensive(mid, &data);

  loop {
    let (new_range, new_cost) = find_new_mid(&old_range, &data, _get_cost_expensive);
    if new_cost > old_cost {
      break;
    } else {
      old_range = new_range;
      old_cost = new_cost;
    }
  }

  for val in old_range.lower..old_range.upper + 1 {
    let cost = _get_cost_expensive(val, &data);
    if cost < old_cost {
      old_cost = cost
    }
  }

  println!("day07b: ans = {}", old_cost);
}

fn parse() -> Vec<u32> {
  file_line_to_uints("data/07_input.txt")
}

fn get_cost(target: u32, data: &[u32]) -> u32 {
  let mut cost = 0;
  for val in data {
    if target > *val {
      cost += target - val;
    } else {
      cost += val - target;
    }
  }

  cost
}

fn _get_cost_expensive(target: u32, data: &[u32]) -> u32 {
  let mut cost = 0;
  for val in data {
    if target > *val {
      cost += (0..(target - val) + 1).sum::<u32>();
    } else {
      cost += (0..(val - target) + 1).sum::<u32>();
    }
  }

  cost
}

#[derive(Debug)]
struct SearchRange {
  lower: u32,
  upper: u32,
  mid: u32,
}

fn find_new_mid(
  range: &SearchRange,
  data: &[u32],
  f: impl Fn(u32, &[u32]) -> u32,
) -> (SearchRange, u32) {
  let lower_mid = (range.lower + range.mid) / 2;
  let upper_mid = (range.mid + range.upper) / 2;
  let lower_cost = f(lower_mid, data);
  let upper_cost = f(upper_mid, data);

  if lower_cost < upper_cost {
    (
      SearchRange {
        lower: range.lower,
        upper: range.mid,
        mid: lower_mid,
      },
      lower_cost,
    )
  } else {
    (
      SearchRange {
        lower: range.mid,
        upper: range.upper,
        mid: upper_mid,
      },
      upper_cost,
    )
  }
}
