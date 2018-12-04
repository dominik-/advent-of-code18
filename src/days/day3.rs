use std::fs;
use std::path::Path;
use regex::Regex;

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