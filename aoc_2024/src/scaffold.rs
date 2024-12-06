use std::fs;
use std::io::Result;
use std::io::Write;

pub fn create_day_files(current_day: i32) -> Result<()> {
  let day_str = format!("{:02}", current_day);
  let example_path = format!("example/day_{}.txt", day_str);
  let input_path = format!("input/day_{}.txt", day_str);
  let src_path = format!("src/day_{}.rs", day_str);
  let draft_path = "draft.txt";

  fs::File::create(&example_path)?;
  
  let url = format!("https://adventofcode.com/2024/day/{}/input", current_day);
  let client = reqwest::blocking::Client::new();
  let cookie = fs::read_to_string("input/cookie.txt")?;
  let response = client.get(&url)
    .header("Cookie", cookie)
    .send()
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?
    .text()
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
  let mut input_file = fs::File::create(&input_path)?;
  write!(input_file, "{}", response)?;

  fs::copy(draft_path, &src_path)?;

  Ok(())
}