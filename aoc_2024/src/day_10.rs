use std::collections::HashSet;

fn trailhead_score_1(res: &mut HashSet<Vec<i32>>, grid: &Vec<Vec<i32>>, start: Vec<i32>, width: usize, height: usize) {
  let current = grid[start[0] as usize][start[1] as usize];
  
  let directions = vec![ (-1, 0), (1, 0), (0, -1), (0, 1) ];
  
  for direction in directions {
    let new_x = start[0] + direction.0;
    let new_y = start[1] + direction.1;
    
    if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
      let new_value = grid[new_x as usize][new_y as usize];
      if new_value == current + 1 {
        if new_value == 9 {
          res.insert(vec![new_x, new_y]);
        } else { 
          trailhead_score_1(res, grid, vec![new_x, new_y], width, height);
        }
      }
    }
  }
}


pub fn solve_1(input: &str) -> i32 {
  let lines: Vec<&str> = input.lines().collect();
  let mut grid = vec![];
  let mut zeros: Vec<Vec<i32>> = vec![];
  let width = lines[0].chars().count();
  let height = lines.len();
  
  for line in lines {
    let mut row: Vec<i32> = vec![];
    for num in line.chars() {
      let parsed = num.to_digit(10).unwrap() as i32;
      if parsed == 0 {
        zeros.push(vec![grid.len() as i32, row.len() as i32]);
      }
      row.push(parsed);
    }
    grid.push(row);
  }
  
  zeros
  .iter()
  .map(|zero| {
    let mut res = HashSet::new();
    trailhead_score_1(&mut res, &grid, zero.clone(), width, height);
    res.len() as i32
  })
  .sum()
}


fn trailhead_score_2(grid: &Vec<Vec<i32>>, start: Vec<i32>, width: usize, height: usize) -> i32 {
  let current = grid[start[0] as usize][start[1] as usize];
  
  let directions = vec![ (-1, 0), (1, 0), (0, -1), (0, 1) ];
  let mut sum = 0;
  
  for direction in directions {
    let new_x = start[0] + direction.0;
    let new_y = start[1] + direction.1;
    
    if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
      let new_value = grid[new_x as usize][new_y as usize];
      if new_value == current + 1 {
        sum += if new_value == 9 {
          1
        } else { 
          trailhead_score_2(grid, vec![new_x, new_y], width, height)
        }
      }
    }
  }
  
  sum
}

pub fn solve_2(input: &str) -> i32 {
  let lines: Vec<&str> = input.lines().collect();
  let mut grid = vec![];
  let mut zeros: Vec<Vec<i32>> = vec![];
  let width = lines[0].chars().count();
  let height = lines.len();
  
  for line in lines {
    let mut row: Vec<i32> = vec![];
    for num in line.chars() {
      let parsed = num.to_digit(10).unwrap() as i32;
      if parsed == 0 {
        zeros.push(vec![grid.len() as i32, row.len() as i32]);
      }
      row.push(parsed);
    }
    grid.push(row);
  }
  
  zeros
  .iter()
  .map(|zero| trailhead_score_2(&grid, zero.clone(), width, height))
  .sum()
}

#[cfg(test)]
mod tests {
  use aoc_2024::read_example;
  
  use super::*;
  
  #[test]
  fn test_solve_1() {
    let input = read_example!(10);
    assert_eq!(solve_1(&input), 36);
  }
  
  #[test]
  fn test_solve_2() {
    let input = read_example!(10);
    assert_eq!(solve_2(&input), 81);
  }
}