use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn day2() -> (String, String) {
  let file_path = Path::new("./inputs/day2");
  let file_as_string = fs::read_to_string(file_path).unwrap();
  let mut count_map: HashMap<i32, i32> = HashMap::new();
  let mut line_map: HashMap<char,i32> = HashMap::new();
  //iterate over lines
  for line in file_as_string.clone().lines() {
    line_map.clear();
    //count all chars
    for c in line.chars() {
      let counter = line_map.entry(c).or_insert(0);
      *counter += 1;
    }
    let mut dedup_values = line_map.values().collect::<Vec<&i32>>();
    dedup_values.sort();
    dedup_values.dedup();
    for counter in dedup_values.iter() {
      let existing_counter = count_map.entry(**counter).or_insert(0);
      *existing_counter += 1;
    }
  }
  let hash = count_map[&2] * count_map[&3];

  let mut letter: String = String::new();
  for line in file_as_string.lines() {
    for line2 in file_as_string.lines() {
      let (val, ch, idx) = char_diff(line, line2);
      //println!("Diff: {}", val);
      if val == 1 {
        //println!("Found single char diff.");
        match ch {
          Ok(_c) => {
            let (part_a, part_b) = line.split_at(idx);
            letter = part_a.to_string() + &part_b[1..].to_string();
          },
          Err(e) => eprintln!("error can't occur but was {}?", e),
        }
      }
    }
  }
  return (hash.to_string(), letter);
}

fn char_diff(a: &str, b: &str) -> (i32, Result<char, String>, usize) {
  let mut count = 0;
  let mut occurrence: usize = 0;
  let mut found = false;
  let mut letter: Result<char, String> = Err(String::from("no match."));
  let mut iter_a = a.char_indices();
  let mut iter_b = b.char_indices();
  let mut current = iter_a.next();
  while current != None {
    if found == false {
      occurrence = current.unwrap().0;
    }
    if current.unwrap().1 != iter_b.next().unwrap().1 {
      count += 1;
      letter = Ok(current.unwrap().1);
      found = true;
    }
    current = iter_a.next();
  }
  return (count, letter, occurrence);
}