use std::fs;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

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

pub fn day3() -> (String, String) {
  let file_path = Path::new("./inputs/day3");
  let file_as_string = fs::read_to_string(file_path).unwrap();
  let re = Regex::new(r"#|\s@\s|,|:\s|x").unwrap();
  let mut squares: Vec<Square> = Vec::new();
  for line in file_as_string.lines() {
    let mut raw: Vec<&str> = re.split(line).collect();
    //println!("Line: {:?}", raw);
    raw.remove(0);
    let numbers: Vec<u32> = raw.iter().map(|v| v.parse().unwrap()).collect();
    let s: Square = Square{id: numbers[0],
    x0: numbers[1],
    y0: numbers[2],
    x1: numbers[3]+numbers[1],
    y1: numbers[4]+numbers[2],
    taint: false};
    squares.push(s);
  };
  let mut world = vec![vec![vec![0u32; 0]; 1000]; 1000];
  for square in squares.iter() {
    for x in square.x0..square.x1 {
      for y in square.y0..square.y1 {
        world[x as usize][y as usize].push(square.id);
      }
    }
  }
  let mut multi_count = 0;
  for x in 0..999 {
    for y in 0..999 {
      if world[x as usize][y as usize].len() > 1 {
        multi_count += 1;
      }
    }
  }
  for square in squares.iter_mut() {
    for x in square.x0..square.x1 {
      for y in square.y0..square.y1 {
        if world[x as usize][y as usize].len() > 1 {
          square.taint = true;
        }
      }
    }
  }
  let mut untainted_id = 0;
  for square in squares.iter() {
    if !square.taint {
      untainted_id = square.id;
    }
  }
  return (multi_count.to_string(), untainted_id.to_string());
}

struct Square {
  id: u32,
  x0: u32,
  y0: u32,
  x1: u32,
  y1: u32,
  taint: bool,
}