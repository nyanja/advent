fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
  let mut lines = input.lines();
  let rules: Vec<Vec<i32>> = lines
    .by_ref()
    .take_while(|line| !line.is_empty())
    .map(|line| line.split('|').map(|n| n.trim().parse().unwrap()).collect())
    .collect();
  let updates: Vec<Vec<i32>> = lines
    .map(|line| line.split(',').map(|n| n.trim().parse().unwrap()).collect())
    .collect();
  (rules, updates)
}

fn is_ordered(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
  update.windows(2).all(|w| rules.contains(&w.to_vec()))
}

fn mid(vec: Vec<i32>) -> i32 {
  vec[vec.len() / 2]
}

pub fn solve_1(input: &str) -> i32 {
  let (rules, updates) = parse_input(input);
  updates.into_iter()
    .filter(|update| is_ordered(update, &rules))
    .map(mid)
    .sum()
}

pub fn solve_2(input: &str) -> i32 {
  let (rules, updates) = parse_input(input);
  updates.into_iter()
    .map(|mut update| {
      if is_ordered(&update, &rules) {
        return 0;
      }
      update.sort_by(|a, b| {
        match rules.contains(&vec![*a, *b]) {
          true  => std::cmp::Ordering::Less,
          false => std::cmp::Ordering::Greater,
        }
      });
      mid(update)
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;

use super::*;

  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 143);
  }

  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 123);
  }
}