use advent_of_code::util::*;

pub fn run_a() {
  let matrix = parse();
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
        node.incoming.push(Coord {
          row: matrix.rows,
          col: matrix.cols,
        });
      }

      // for the first and last columns, count the left and right (respectively)
      // walls as incoming
      if col == 0 || col == matrix.cols - 1 {
        let node: &mut Node = graph.get_mut(graph_index).unwrap();
        node.incoming.push(Coord {
          row: matrix.rows,
          col: matrix.cols,
        });
      }

      if let Some(right_val) = right {
        if val < right_val {
          let node: &mut Node = graph.get_mut(graph_index).unwrap();
          node.incoming.push(right_coord);
        } else if val > right_val {
          // unwrap should be safe since right_val was Some
          let right_index = matrix.make_index(&right_coord).unwrap();
          graph
            .get_mut(right_index)
            .unwrap()
            .incoming
            .push(coord.clone());
        }
      }

      if let Some(down_val) = down {
        let node: &mut Node = graph.get_mut(graph_index).unwrap();
        if val < down_val {
          node.incoming.push(down_coord);
        } else if val > down_val {
          // unwrap should be safe since right_val was Some
          let down_index = matrix.make_index(&down_coord).unwrap();
          graph
            .get_mut(down_index)
            .unwrap()
            .incoming
            .push(coord.clone());
        }
      }
    }
  }

  let mut ans = 0;
  for node in graph {
    if node.incoming.len() == 4 {
      println!("node with val {} is local minimum", node.value);
      ans += node.value + 1;
    }
  }
  println!("day09a: ans = {}", ans);
}

pub fn run_b() {
  println!("day09b: ans = {}", 42);
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
  let mut first = true;
  let mut rows = 0;
  let mut cols = 0;

  for line in lines {
    rows += 1;
    for c in line.chars() {
      if first {
        cols += 1;
      }
      let val = c
        .to_digit(10)
        .unwrap_or_else(|| panic!("unable to parse {} into a u32", c));
      data.push(val);
    }

    if first {
      first = false;
    }
  }

  Matrix { cols, rows, data }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
  row: usize,
  col: usize,
}

#[derive(Debug)]
struct Node {
  value: u32,
  incoming: Vec<Coord>,
}

#[derive(Debug)]
struct Matrix {
  cols: usize,
  rows: usize,
  data: Vec<u32>,
}

impl Matrix {
  fn get_cell(&self, coord: &Coord) -> Option<&u32> {
    self.make_index(coord).map(|i| &self.data.as_slice()[i])
  }

  fn make_index(&self, coord: &Coord) -> Option<usize> {
    if coord.row >= self.rows || coord.col >= self.cols {
      return None;
    }
    Some(self.cols * coord.row + coord.col)
  }
}
