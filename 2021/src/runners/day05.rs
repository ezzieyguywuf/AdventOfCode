use advent_of_code::util::*;
use std::cmp::Ordering;
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
    .for_each(|points| update_map_a(&mut map, &points[0], &points[1]));

  let amt = map.iter().filter(|(_, v)| **v >= 2).fold(0, |mut acc, _| {
    acc += 1;
    acc
  });
  println!("total: {}", amt);
}

pub fn run_b() {
  println!("Hello day05, part b!")
}

fn update_map_a(map: &mut HashMap<Point, u32>, p1: &Point, p2: &Point) {
  if p1.x == p2.x {
    match p1.y.cmp(&p2.y) {
      Ordering::Less => add_line(Orientation::Horizontal, map, p1, p2),
      Ordering::Greater => add_line(Orientation::Horizontal, map, p2, p1),
      Ordering::Equal => add_point(map, p1),
    }
  } else if p1.y == p2.y {
    match p1.x.cmp(&p2.x) {
      Ordering::Less => add_line(Orientation::Vertical, map, p1, p2),
      Ordering::Greater => add_line(Orientation::Vertical, map, p2, p1),
      Ordering::Equal => add_point(map, p1),
    }
  }
}

fn add_line(orient: Orientation, map: &mut HashMap<Point, u32>, p1: &Point, p2: &Point) {
  let xs: Vec<u32>;
  let ys: Vec<u32>;

  match orient {
    Orientation::Vertical => {
      xs = (p1.x..p2.x + 1).collect();
      ys = iter::repeat(p1.y).take(xs.len()).collect();
    }
    Orientation::Horizontal => {
      ys = (p1.y..p2.y + 1).collect();
      xs = iter::repeat(p1.x).take(ys.len()).collect();
    }
  }

  for (x, y) in xs.iter().zip(ys) {
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

enum Orientation {
  Vertical,
  Horizontal,
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
