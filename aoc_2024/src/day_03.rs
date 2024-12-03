use regex::Regex;


fn sum_muls(input: &str) -> i32 {
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  re.captures_iter(input)
    .map(|cap| { cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap() })
    .sum()
}

pub fn solve_1(input: &str) -> i32 {
  sum_muls(input)
}

pub fn solve_2(input: &str) -> i32 {
  let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
  let result = re.replace_all(input, "");
  sum_muls(&result)
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;

use super::*;

  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 161);
  }

  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 48);
  }
}