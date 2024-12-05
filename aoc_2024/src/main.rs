use aoc_2024::read_input;
use std::env;
use std::time::Instant;

// mod day_01;
// mod day_02;
// mod day_03;
// mod day_04;
mod day_05;
mod scaffold;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 && args[1] == "new" {
    let day = aoc_2024::get_current_day(None);
    if let Err(e) = scaffold::create_day_files(day) {
      eprintln!("Error creating day files: {:?}", e);
    }
    return;
  }

  let day = aoc_2024::get_current_day(None);
  let start = Instant::now();
  let result_1 = day_05::solve_1(&read_input(day));
  let duration_1 = start.elapsed();
  let start = Instant::now();
  let result_2 = day_05::solve_2(&read_input(day));
  let duration_2 = start.elapsed();
    
  println!("Result of solve_1: {:?} (took {:?})", result_1, duration_1);
  println!("Result of solve_2: {:?} (took {:?})", result_2, duration_2);
}

