use aoc_2024::read_input;
use std::env;
use std::time::Instant;

// mod day_01;
// mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
mod day_14;
mod scaffold;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 && args[1] == "new" {
    let day:i32 = if args.len() > 2 {
      args[2].parse::<i32>().unwrap_or_else(|_| aoc_2024::get_current_day(None))
    } else {
      aoc_2024::get_current_day(None)
    };
    if let Err(e) = scaffold::create_day_files(day) {
      eprintln!("Error creating day files: {:?}", e);
    }
    return;
  }

  // let day = aoc_2024::get_current_day(None);
  let day = 14;
  let start = Instant::now();
  let result_1 = day_14::solve_1(&read_input(day));
  let duration_1 = start.elapsed();
  println!("Result of solve_1: {:?} (took {:?})", result_1, duration_1);

  let start = Instant::now();
  let result_2 = day_14::solve_2(&read_input(day));
  let duration_2 = start.elapsed();
  println!("Result of solve_2: {:?} (took {:?})", result_2, duration_2);
}

