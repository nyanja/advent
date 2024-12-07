use std::collections::HashMap;

fn add(a: i64, b: i64) -> i64 { a + b }
fn mul(a: i64, b: i64) -> i64 { a * b }
fn con(a: i64, b: i64) -> i64 { format!("{}{}", a, b).parse().unwrap() }


fn combs<F>(fns: &[F], length: usize, memo: &mut HashMap<(usize, usize), Vec<Vec<F>>>) -> Vec<Vec<F>>
where
F: Copy,
{
  if memo.contains_key(&(fns.len(), length)) {
    return memo[&(fns.len(), length)].clone();
  } 

  let n = fns.len();
  let mut result = Vec::new();
  let total = n.pow(length as u32);
  
  for i in 0..total {
    let mut combo = Vec::new();
    let mut state = i;
    
    for _ in 0..length {
      let index = state % n; 
      combo.push(fns[index]);
      state /= n; 
    }
    result.push(combo);
  }
  memo.insert((n, length), result.clone());
  result
}

fn solve(fns: &[fn(i64, i64) -> i64], input: &str) -> i64 {
  let mut total = 0;
  let mut memo = HashMap::new();
  for line in input.lines() {
    let parts: Vec<&str> = line.split(": ").collect();
    let res: i64 = parts[0].parse().unwrap();
    let ops: Vec<i64> = parts[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    for combo in combs(fns, ops.len() - 1, &mut memo) {
      let mut result = ops[0];
      for (i, &op) in combo.iter().enumerate() {
        result = op(result, ops[i + 1]);
      }
      if result == res {
        total += res;
        break;
      }
    }
  }
  total
}


pub fn solve_1(input: &str) -> i64 {
  solve(&[add, mul], input)
}

pub fn solve_2(input: &str) -> i64 {
  solve(&[add, mul, con], input)
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;
  
  use super::*;
  
  #[test]
  fn test_solve_1() {
    let input = read_example();
    assert_eq!(solve_1(&input), 3749);
  }
  
  #[test]
  fn test_solve_2() {
    let input = read_example();
    assert_eq!(solve_2(&input), 11387);
  }
}