pub fn read_input(day: i32) -> String {
  std::fs::read_to_string(format!("input/day_{:02}.txt", day))
  .expect("Failed to read input file")
}
