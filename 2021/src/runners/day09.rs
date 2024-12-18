use advent_of_code::util::*;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

pub fn run_a() {
  let matrix = parse();
  let mut ans = 0;
  let graph = make_graph(&matrix);
  let indices = get_low_spots(&graph);

  for i in indices {
    // unwrap should be safe since the index came from get_low_spots...
    let node = graph.get(i).unwrap();
    ans += node.value + 1;
  }
  println!("day09a: ans = {}", ans);
}

pub fn run_b() {
  let matrix = parse();
  let graph = make_graph(&matrix);
  let low_spots = get_low_spots(&graph);
  let mut basin_sizes: Vec<u32> = Vec::new();

  for i in low_spots {
    let basin_size = get_basin_size(i, &graph);
    basin_sizes.push(basin_size);
  }

  basin_sizes.sort_unstable();
  let ans: u32 = basin_sizes.iter().rev().take(3).product();

  println!("day09b: ans = {}", ans);
}

fn get_basin_size(which: usize, graph: &[Node]) -> u32 {
  let mut size: u32 = 0;
  let mut seen: HashSet<usize> = HashSet::from([which]);
  let mut queue: VecDeque<usize> = VecDeque::from([which]);

  while let Some(visit) = queue.pop_front() {
    size += 1;

    // since the walls are represented by Matrix::out_of_bounds_index, we can't
    // arbitrarily unwrap here
    if let Some(node) = graph.get(visit) {
      for neighbor_index in &node.incoming {
        if let Some(neighbor) = graph.get(*neighbor_index) {
          if neighbor.value < 9 && !seen.contains(neighbor_index) {
            seen.insert(*neighbor_index);
            queue.push_back(*neighbor_index);
          }
        }
      }
    }
  }

  size
}

fn get_low_spots(graph: &[Node]) -> Vec<usize> {
  let mut out: Vec<usize> = Vec::new();
  for (i, node) in graph.iter().enumerate() {
    if node.incoming.len() == 4 {
      out.push(i);
    }
  }

  out
}

fn make_graph(matrix: &Matrix) -> Vec<Node> {
  let mut graph: Vec<Node> = Vec::new();

  for value in &matrix.data {
    graph.push(Node {
      value: *value,
      incoming: Vec::new(),
    });
  }

  for row in 0..matrix.rows {
    for col in 0..matrix.cols {
      let coord = Coord { row, col };
      // these two should always be valid, thus unwrap is ok
      let val = matrix.get_cell(&coord).unwrap();
      let graph_index = matrix.make_index(&coord).unwrap();
      {
        let node: &mut Node = graph.get_mut(graph_index).unwrap();
        node.value = *val;
      }

      // these two may be invalid
      let right_coord = Coord { row, col: col + 1 };
      let down_coord = Coord { row: row + 1, col };
      let right = matrix.get_cell(&right_coord);
      let down = matrix.get_cell(&down_coord);

      // for the first and last rows , count the upper and lower (respectively)
      // walls as incoming
      if row == 0 || row == matrix.rows - 1 {
        let node: &mut Node = graph.get_mut(graph_index).unwrap();
        node.incoming.push(matrix.out_of_bounds_index());
      }

      // for the first and last columns, count the left and right (respectively)
      // walls as incoming
      if col == 0 || col == matrix.cols - 1 {
        let node: &mut Node = graph.get_mut(graph_index).unwrap();
        node.incoming.push(matrix.out_of_bounds_index());
      }

      if let Some(right_val) = right {
        match val.cmp(right_val) {
          Ordering::Less => {
            let node: &mut Node = graph.get_mut(graph_index).unwrap();
            node.incoming.push(matrix.make_index(&right_coord).unwrap());
          }
          Ordering::Greater => {
            // unwrap should be safe since right_val was Some
            let right_index = matrix.make_index(&right_coord).unwrap();
            graph
              .get_mut(right_index)
              .unwrap()
              .incoming
              .push(matrix.make_index(&coord).unwrap());
          }
          _ => (),
        }
      }

      if let Some(down_val) = down {
        match val.cmp(down_val) {
          Ordering::Less => {
            let node: &mut Node = graph.get_mut(graph_index).unwrap();
            node.incoming.push(matrix.make_index(&down_coord).unwrap());
          }
          Ordering::Greater => {
            // unwrap should be safe since right_val was Some
            let down_index = matrix.make_index(&down_coord).unwrap();
            graph
              .get_mut(down_index)
              .unwrap()
              .incoming
              .push(matrix.make_index(&coord).unwrap());
          }
          _ => (),
        }
      }
    }
  }

  graph
}

// sample input:
// 12345\n
// 67890
//
// sample output:
// Matrix { rows: 2, cols: 5, data: [1,2,3,4,5,6,7,8,9,0] }
fn parse() -> Matrix {
  let lines = file_to_lines("data/09_input.txt");
  let mut data: Vec<u32> = Vec::new();
  let mut rows = 0;

  for line in lines {
    rows += 1;
    for c in line.chars() {
      let val = c
        .to_digit(10)
        .unwrap_or_else(|| panic!("unable to parse {} into a u32", c));
      data.push(val);
    }
  }

  let cols = data.len() / rows;
  Matrix { cols, rows, data }
}

#[derive(Debug)]
struct Node {
  value: u32,
  incoming: Vec<usize>,
}
