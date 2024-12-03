use aoc_2024::read_input;
use std::env;

mod day_01;
mod day_02;
mod day_03;
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
    let result_1 = day_03::solve_1(&read_input(day));
    let result_2 = day_03::solve_2(&read_input(day));
    println!("Result of solve_1: {:?}", result_1);
    println!("Result of solve_2: {:?}", result_2);
    
}

