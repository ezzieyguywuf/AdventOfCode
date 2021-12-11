use advent_of_code::util::*;

const INPUT_FILE: &str = "data/test.txt";

pub fn run_a() {
  let lines = file_to_lines(INPUT_FILE);
  let mut stack: Vec<char> = Vec::new();
  let mut ans = 0;

  for line in lines {
    for delim in line.chars() {
      if is_opening(delim) {
        stack.push(delim);
      } else {
        let left = stack.pop().unwrap();

        if !is_pair(left, delim) {
          ans += get_score(delim);
        }
      }
    }
  }

  println!("day09a: ans = {}", ans);
}

pub fn run_b() {
  println!("day09b: ans = {}", 42);
}

fn is_opening(c: char) -> bool {
  c == '[' || c == '(' || c == '<' || c == '{'
}

fn is_pair(left: char, right: char) -> bool {
  if left == '[' {
    right == ']'
  } else if left == '{' {
    right == '}'
  } else if left == '(' {
    right == ')'
  } else if left == '<' {
    right == '>'
  } else {
    panic!("The character {} is not part of this puzzle", left)
  }
}

fn get_score(c: char) -> u32 {
  if c == ')' {
    3
  } else if c == ']' {
    57
  } else if c == '}' {
    1197
  } else if c == '>' {
    25137
  } else {
    panic!("{} does not seem to be scorable", c)
  }
}
