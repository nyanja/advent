use chrono::Datelike;

pub fn read_input(day: i32) -> String {
  std::fs::read_to_string(format!("input/day_{:02}.txt", day))
  .expect("Failed to read input file")
}

pub fn read_example() -> String {
  let day = get_current_day(None);
  std::fs::read_to_string(format!("example/day_{:02}.txt", day))
  .expect("Failed to read example file")
}

pub fn get_current_day(args: Option<i32>) -> i32 {
  if let Some(day) = args {
    day
  } else {
    let today = chrono::Utc::now().day();
    today as i32
  }
}