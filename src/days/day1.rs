use std::fs;
use std::path::Path;
use std::collections::HashSet;

pub fn day1() -> (String, String) {
  //let inputLines = parseInputToLines("inputs/day1");
  let file_path = Path::new("./inputs/day1");
  let file_as_string = fs::read_to_string(file_path);
  let mut final_frequency: i32 = 0;
  let file_as_string = file_as_string.unwrap();
  for line in file_as_string.lines() {
    final_frequency = final_frequency + line.parse::<i32>().unwrap();
  }
  let mut current_frequency: i32 = 0;
  let mut frequencies: HashSet<i32> = HashSet::new();
  let mut lines = file_as_string.lines().cycle();
  let mut duplicate = false;
  while !duplicate {
    //println!("{}", line);
    current_frequency = current_frequency + lines.next().unwrap().parse::<i32>().unwrap();
    duplicate = !frequencies.insert(current_frequency);
  }
  return (final_frequency.to_string(), current_frequency.to_string());
}