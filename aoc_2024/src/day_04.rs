use std::collections::HashMap;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
  Horizontal,
  Vertical,
  DiagonalLeft,
  DiagonalRight,
  ReverseHorizontal,
  ReverseVertical,
  ReverseDiagonalLeft,
  ReverseDiagonalRight,
}

const DIRECTIONS: [Direction; 4] = [
  Direction::Horizontal,
  Direction::Vertical,
  Direction::DiagonalLeft,
  Direction::DiagonalRight,
];

const REVERSE_DIRECTIONS: [Direction; 4] = [
  Direction::ReverseHorizontal,
  Direction::ReverseVertical,
  Direction::ReverseDiagonalLeft,
  Direction::ReverseDiagonalRight,
];

type Matches = HashMap<(usize, usize, char, Direction), bool>;

fn prev_symbol(symbol: char, direction: Direction) -> char {
  match direction {
    Direction::Horizontal | Direction::Vertical | Direction::DiagonalLeft | Direction::DiagonalRight => match symbol {
      'M' => 'X',
      'A' => 'M',
      'S' => 'A',
      _ => symbol,
    },
    Direction::ReverseHorizontal | Direction::ReverseVertical | Direction::ReverseDiagonalLeft | Direction::ReverseDiagonalRight => match symbol {
      'A' => 'S',
      'M' => 'A',
      'X' => 'M',
      _ => symbol,
    },
  }
}

fn has_match(matches: &Matches, i: usize, j: usize, symbol: char, direction: Direction) -> bool {
  let prev = prev_symbol(symbol, direction);
  match direction {
    Direction::Horizontal | Direction::ReverseHorizontal => {
      if j > 0 {
        matches.get(&(i, j - 1, prev, direction)).copied().unwrap_or(false)
      } else {
        false
      }
    }
    Direction::Vertical | Direction::ReverseVertical => {
      if i > 0 {
        matches.get(&(i - 1, j, prev, direction)).copied().unwrap_or(false)
      } else {
        false
      }
    }
    Direction::DiagonalLeft | Direction::ReverseDiagonalLeft => {
      if i > 0 && j > 0 {
        matches.get(&(i - 1, j - 1, prev, direction)).copied().unwrap_or(false)
      } else {
        false
      }
    }
    Direction::DiagonalRight | Direction::ReverseDiagonalRight => {
      if i > 0 && j + 1 < matches.len() {
        matches.get(&(i - 1, j + 1, prev, direction)).copied().unwrap_or(false)
      } else {
        false
      }
    }
  }
}


pub fn solve_1(input: &str) -> i32 {
  let mut matches: Matches = HashMap::new();
  let mut total = 0;

  for (i, line) in input.lines().enumerate() {
    for (j, symbol) in line.chars().enumerate() {
      match symbol {
        'X' => {
          // Init a match
          for direction in DIRECTIONS.iter() {
            matches.insert((i, j, 'X', *direction), true);
          }
          // Check for M's
          // Finish a reverse match
          for direction in REVERSE_DIRECTIONS.iter() {
            if has_match(&matches, i, j, symbol, *direction) {
              total += 1;
            }
          }
        },
        'M' | 'A' => {
          for direction in DIRECTIONS.iter().chain(REVERSE_DIRECTIONS.iter()) {
            if has_match(&matches, i, j, symbol, *direction) {
              matches.insert((i, j, symbol, *direction), true);
            }
          }
        },
        'S' => {
          // Init a reverse match
          for direction in REVERSE_DIRECTIONS.iter() {
            matches.insert((i, j, 'S', *direction), true);
          }
          // Check for A's
          // Finish a match
          for direction in DIRECTIONS.iter() {
            if has_match(&matches, i, j, symbol, *direction) {
              total += 1;
            }
          }
        },
        _ => {}
      }
    }
  }
  total
}


pub fn solve_2(input: &str) -> i32 {
  let mut total = 0;
  let allowed_diags = [
    ['M', 'M', 'S', 'S'],
    ['M', 'S', 'S', 'M'],
    ['S', 'M', 'M', 'S'],
    ['S', 'S', 'M', 'M'],
  ];
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  for i in 1..grid.len() - 1 {
    for j in 1..grid[i].len() - 1 {
      if grid[i][j] == 'A' {
        let diags = [
          grid[i - 1][j - 1],
          grid[i - 1][j + 1],
          grid[i + 1][j + 1],
          grid[i + 1][j - 1],
        ];
        if allowed_diags.contains(&diags) {
          total += 1;
        }
      }
    }
  }
  total
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;

use super::*;

  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 18);
  }

  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 9);
  }
}