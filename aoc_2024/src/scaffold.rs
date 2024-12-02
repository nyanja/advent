use std::fs;
use std::io::Result;

pub fn create_day_files(current_day: i32) -> Result<()> {
  let day_str = format!("{:02}", current_day);
  let example_path = format!("example/day_{}.txt", day_str);
  let input_path = format!("input/day_{}.txt", day_str);
  let src_path = format!("src/day_{}.rs", day_str);
  let draft_path = "src/draft.rs";

  // Create example and input files
  fs::File::create(&example_path)?;
  fs::File::create(&input_path)?;

  // Copy contents of draft.rs to the new src file
  fs::copy(draft_path, &src_path)?;

  Ok(())
}