use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn turn_right(&self) -> Direction {
    match self {
      Direction::Up    => Direction::Right,
      Direction::Down  => Direction::Left,
      Direction::Left  => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }
}

type Coord = (i32, i32);

fn track(mut point: Coord, obstacles: &HashMap<Coord, bool>, width: usize, height: usize) -> HashMap<Coord, bool> {
  let mut direction = Direction::Up;
  let mut visited: HashMap<Coord, bool> = HashMap::new();
  let mut turns: HashMap<(Coord, Direction), bool> = HashMap::new();
  loop {
    let next = match direction {
      Direction::Up => (point.0, point.1 - 1),
      Direction::Down => (point.0, point.1 + 1),
      Direction::Left => (point.0 - 1, point.1),
      Direction::Right => (point.0 + 1, point.1),
    };

    if obstacles.contains_key(&next) {
      if turns.contains_key(&(point, direction.clone())) {
        visited.clear();
        break;
      }
      turns.insert((point, direction.clone()), true);
      direction = direction.turn_right();
    } else {
      visited.insert(point, true);
      point = next;
    }

    if next.0 < 0 || next.1 < 0 || next.0 >= width as i32 || next.1 >= height as i32 {
      break;
    }
  }
  visited
}

pub fn solve_1(input: &str) -> i32 {
  let mut obstacles: HashMap<Coord, bool> = HashMap::new();
  let mut point: Coord = (0, 0);
  let height = input.lines().count();
  let width = input.lines().next().unwrap().len();
  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      if ch == '#' {
        obstacles.insert((x as i32, y as i32), true);
      } else if ch == '^' {
        point = (x as i32, y as i32);
      }
    }
  }
  track(point, &obstacles, width, height).len() as i32
}

pub fn solve_2(input: &str) -> i32 {
  let mut obstacles: HashMap<Coord, bool> = HashMap::new();
  let mut point: Coord = (0, 0);
  let mut count = 0;
  let height = input.lines().count();
  let width = input.lines().next().unwrap().len();
  for (y, line) in input.lines().enumerate() {
    for (x, ch) in line.chars().enumerate() {
      if ch == '#' {
        obstacles.insert((x as i32, y as i32), true);
      } else if ch == '^' {
        point = (x as i32, y as i32);
      }
    }
  }
  let visited = track(point, &obstacles, width, height);
  for empty in visited.keys() {
    let mut obstacles = obstacles.clone();
    obstacles.insert(*empty, true);
    let result = track(point, &obstacles, width, height);
    if result.len() == 0 {
      count += 1;
    }
  }
  count
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;

use super::*;

  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 41);
  }

  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 6);
  }
}