// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, 2 reports are safe.


fn is_safe(numbers: &Vec<i32>) -> bool {
  numbers.windows(2).all(|w| 1 <= (w[1] - w[0]) && (w[1] - w[0]) <= 3)
    || numbers.windows(2).all(|w| -1 >= (w[1] - w[0]) && (w[1] - w[0]) >= -3)
}

pub fn solve_1(input: &str) -> i32 {
  let mut counter = 0;
  for line in input.lines() {
    let numbers: Vec<i32> = line.split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect();
    
    if is_safe(&numbers) {
      counter += 1;
    }
  }
  counter
}


// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
// Thanks to the Problem Dampener, 4 reports are actually safe!

pub fn solve_2(input: &str) -> i32 {
  let mut counter = 0;
  for line in input.lines() {
    let numbers: Vec<i32> = line.split_whitespace()
      .filter_map(|s| s.parse().ok())
      .collect();
    
    if is_safe(&numbers) {
      counter += 1;
      continue;
    }

    for i in 0..numbers.len() {
      let mut modified_numbers = numbers.clone();
      modified_numbers.remove(i);
      if is_safe(&modified_numbers) {
        counter += 1;
        break;
      }
    }
  }
  counter
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;
  
  use super::*;
  
  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 2);
  }
  
  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 4);
  }
}