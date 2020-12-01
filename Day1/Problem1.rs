// Advent of Code 2020: Day 1, Problem 1 
//
// In a list of numbers find the two entries that sum to 2020 and then multiply those two numbers together.
//
use std::io::Read;

fn main(){
   let mut file = std::fs::File::open("input_P1.txt").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   print!("{}", contents);
}
