use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use chrono::prelude::*;

pub fn day4() -> (String, String) {
  let file_path = Path::new("./inputs/day4");
  let file_as_string = fs::read_to_string(file_path).unwrap();
  let re = Regex::new(r"\[|\]\s").unwrap();
  let fmt = "%Y-%m-%d %H:%M";
   for line in file_as_string.lines().take(1) {
    let mut raw: Vec<&str> = re.split(line).collect();
    let res = NaiveDateTime::parse_from_str(raw[1], fmt).unwrap();
    //parse event type and id
    //push event to vector
   }
   //order event vector
   //collect all events into per-id-per-minute sleep slots
   //sum up total sleep per id
   //for highest sleep id, find max #per-minute slots "asleep"
   //return id * per-minute slot

  return (String::new(), String::new());
}

struct Event {
  time: NaiveDateTime,
  id: i32,
  event: String,
}