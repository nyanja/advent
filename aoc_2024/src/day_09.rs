
fn checksum(res: &Vec<i64>) -> i64 {
  res.iter().enumerate().map(|(i, &val)| val * i as i64).sum()
}

pub fn solve_1(input: &str) -> i64 {
  let mut vs: Vec<i64> = Vec::new();
  let mut ds: Vec<bool> = Vec::new();
  let mut res: Vec<i64> = Vec::new();
  let mut i = 0;
  for chunk in input.lines().next().unwrap().chars().collect::<Vec<_>>().chunks(2) {
    if chunk.len() == 2 {
      let (v, s) = (chunk[0].to_digit(10).unwrap() as i64, chunk[1].to_digit(10).unwrap() as i64);
      for _ in 0..v { vs.push(i); ds.push(true); }
      for _ in 0..s { ds.push(false); }
    } else {
      let v = chunk[0].to_digit(10).unwrap() as i64;
      for _ in 0..v { vs.push(i); ds.push(true) }
    }
    i += 1;
  }
  while vs.len() > 0 {
    if ds.remove(0) {
      res.push(vs.remove(0));
    } else {
      res.push(vs.pop().unwrap());
    }
  }
  checksum(&res)
}

pub fn solve_2(input: &str) -> i64 {
  0
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_solve_1() {
    let var_name = aoc_2024::read_example!(9);
    let input = var_name;
    assert_eq!(solve_1(&input), 1928);
  }

  #[test]
  fn test_solve_2() {
    let input = aoc_2024::read_example!(9);
    assert_eq!(solve_2(&input), 0);
  }
}