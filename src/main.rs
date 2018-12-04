mod days;
extern crate regex;
extern crate chrono;
use days::*;

fn main() {
    println!("Day 1 Results: {:?}", day1::day1());
    println!("Day 2 Results: {:?}", day2::day2());
    println!("Day 3 Results: {:?}", day3::day3());
    println!("Day 4 Results: {:?}", day4::day4());
}
