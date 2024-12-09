use std::collections::{HashMap, HashSet};

type Coord = (i32, i32);

pub fn solve_1(input: &str) -> i32 {
  let mut ch_xy: HashMap<char, Vec<Coord>> = HashMap::new();
  let mut res: HashSet<Coord> = HashSet::new();
  let width: i32 = input.lines().next().unwrap().len() as i32;
  let height: i32 = input.lines().count() as i32;
  let within = |(x, y): Coord| x < width && y < height && x >= 0 && y >= 0;
  
  for (line_idx, line) in input.lines().enumerate() {
    for (char_idx, ch) in line.chars().enumerate() {
      if ch != '.' {
        ch_xy.entry(ch).or_insert_with(Vec::new).push((line_idx as i32, char_idx as i32));
      }
    }
  }
  
  for (_ch, coords) in &ch_xy {
    for &(x1, y1) in coords {
      for &(x2, y2) in coords {
        if (x1, y1) != (x2, y2) {
          let dx: i32 = x2 - x1;
          let dy: i32 = y2 - y1;
          let ex = (x2 + dx, y2 + dy);
          if within(ex) { res.insert(ex); }
        }
      }
    }
  }
  res.len() as i32
}

pub fn solve_2(input: &str) -> i32 {
  let mut ch_xy: HashMap<char, Vec<Coord>> = HashMap::new();
  let mut res: HashSet<Coord> = HashSet::new();
  let width: i32 = input.lines().next().unwrap().len() as i32;
  let height: i32 = input.lines().count() as i32;
  let within = |(x, y): Coord| x < width && y < height && x >= 0 && y >= 0;
  
  for (line_idx, line) in input.lines().enumerate() {
    for (char_idx, ch) in line.chars().enumerate() {
      if ch != '.' {
        ch_xy.entry(ch).or_insert_with(Vec::new).push((char_idx as i32, line_idx as i32));
      }
    }
  }
  
  for (_ch, coords) in &ch_xy {
    for &(x1, y1) in coords {
      for &(x2, y2) in coords {
        if (x1, y1) != (x2, y2) {
          res.insert((x1, y1));
          let dx: i32 = x2 - x1;
          let dy: i32 = y2 - y1;
          let mut ex = (x2 + dx, y2 + dy);
          while within(ex) {
            res.insert(ex);
            ex = (ex.0 + dx, ex.1 + dy);
          }
        }
      }
    }
  }
  res.len() as i32
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;
  
  use super::*;
  
  #[test]
  fn test_solve_1() {
    let input = read_example!(8);
    assert_eq!(solve_1(&input), 14);
  }
  
  #[test]
  fn test_solve_2() {
    let input = read_example!(8);
    assert_eq!(solve_2(&input), 34);
  }
}