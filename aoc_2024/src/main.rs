use chrono::Datelike;

mod day_01;

fn main() {
    let day = chrono::Utc::now().day();
    match day {
        1 => {
            day_01::solve_1();
            day_01::solve_2(); 
        },
        _ => println!("Day not implemented!"),
    }
}