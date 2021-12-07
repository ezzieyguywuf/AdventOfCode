use advent_of_code::util::*;
use std::collections::HashMap;
use std::iter;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn run_a() {
  let lines = file_to_lines("data/05_input.txt");
  let mut map: HashMap<Point, u32> = HashMap::new();

  lines
    .map(|line| {
      line
        .split("->")
        .map(|val| val.trim().parse::<Point>().unwrap())
        .collect::<Vec<_>>()
    })
    .for_each(|points| update_map(&mut map, &points[0], &points[1]));

  let amt = map.iter().filter(|(_, v)| **v >= 2).fold(0, |mut acc, _| {
    acc += 1;
    acc
  });
  println!("total: {}", amt);
}

pub fn run_b() {
  let lines = file_to_lines("data/05_input.txt");
  let mut map: HashMap<Point, u32> = HashMap::new();

  lines
    .map(|line| {
      line
        .split("->")
        .map(|val| val.trim().parse::<Point>().unwrap())
        .collect::<Vec<_>>()
    })
    .for_each(|points| update_map_with_diag(&mut map, &points[0], &points[1]));

  let amt = map.iter().filter(|(_, v)| **v >= 2).fold(0, |mut acc, _| {
    acc += 1;
    acc
  });
  // for (k, v) in map.iter() {
  //   println!("{:?}: {:?}", k, v);
  // }
  println!("total: {}", amt);
}

fn update_map(map: &mut HashMap<Point, u32>, p1: &Point, p2: &Point) {
  if p1.x == p2.x || p1.y == p2.y {
    add_line(map, p1, p2);
  }
}

fn update_map_with_diag(map: &mut HashMap<Point, u32>, p1: &Point, p2: &Point) {
  add_line(map, p1, p2);
}

fn get_orientation(p1: &Point, p2: &Point) -> Orientation {
  if p1 == p2 {
    Orientation::Overlap
  } else if p1.x == p2.x && p2.y > p1.y {
    Orientation::VerticalUp
  } else if p1.x == p2.x && p2.y < p1.y {
    Orientation::VerticalDown
  } else if p1.y == p2.y && p2.x > p1.x {
    Orientation::HorizontalRight
  } else if p1.y == p2.y && p2.x < p1.x {
    Orientation::HorizontalLeft
  } else if p1.x < p2.x && p1.y < p2.y {
    Orientation::DiagonalUpRight
  } else if p1.x < p2.x && p1.y > p2.y {
    Orientation::DiagonalDownRight
  } else if p1.x > p2.x && p1.y < p2.y {
    Orientation::DiagonalUpLeft
  } else if p1.x > p2.x && p1.y > p2.y {
    Orientation::DiagonalDownLeft
  } else {
    unreachable!();
  }
}

fn add_line(map: &mut HashMap<Point, u32>, p1: &Point, p2: &Point) {
  let orient = get_orientation(p1, p2);
  let xs: Vec<u32>;
  let ys: Vec<u32>;

  match orient {
    Orientation::HorizontalRight => {
      xs = (p1.x..p2.x + 1).collect();
      ys = iter::repeat(p1.y).take(xs.len()).collect();
    }
    Orientation::HorizontalLeft => {
      xs = (p2.x..p1.x + 1).collect();
      ys = iter::repeat(p1.y).take(xs.len()).collect();
    }
    Orientation::VerticalUp => {
      ys = (p1.y..p2.y + 1).collect();
      xs = iter::repeat(p1.x).take(ys.len()).collect();
    }
    Orientation::VerticalDown => {
      ys = (p2.y..p1.y + 1).collect();
      xs = iter::repeat(p1.x).take(ys.len()).collect();
    }
    Orientation::Overlap => {
      xs = vec![p1.x];
      ys = vec![p1.y];
    }
    Orientation::DiagonalUpRight => {
      xs = (p1.x..p2.x + 1).collect();
      ys = (p1.y..p2.y + 1).collect();
    }
    Orientation::DiagonalDownLeft => {
      xs = (p2.x..p1.x + 1).collect();
      ys = (p2.y..p1.y + 1).collect();
    }
    Orientation::DiagonalUpLeft => {
      xs = (p2.x..p1.x + 1).collect();
      ys = (p1.y..p2.y + 1).rev().collect();
    }
    Orientation::DiagonalDownRight => {
      xs = (p1.x..p2.x + 1).collect();
      ys = (p2.y..p1.y + 1).rev().collect();
    }
  }

  // println!("{:?} -> {:?}, {:?}", p1, p2, orient);
  for (x, y) in xs.iter().zip(ys) {
    // println!("  ({:?},{:?})", x, y);
    add_point(map, &Point { x: *x, y });
  }
}

fn add_point(map: &mut HashMap<Point, u32>, p: &Point) {
  let amt = map.entry(*p).or_insert(0);
  *amt += 1;
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
  x: u32,
  y: u32,
}

#[derive(Debug)]
enum Orientation {
  VerticalUp,
  VerticalDown,
  HorizontalRight,
  HorizontalLeft,
  Overlap,
  DiagonalUpRight,
  DiagonalDownRight,
  DiagonalUpLeft,
  DiagonalDownLeft,
}

impl FromStr for Point {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coords: Vec<&str> = s.split(',').collect();
    let x_parsed = coords[0].parse::<u32>()?;
    let y_parsed = coords[1].parse::<u32>()?;
    Ok(Point {
      x: x_parsed,
      y: y_parsed,
    })
  }
}
