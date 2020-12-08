//
// Advent of Code 2020: Day 8
//
#![feature(str_split_once)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let commands: Vec<(&str, i32)> = input
        .lines()
        .map(|line| parse_command(line))
        .collect();
   
    println!("Accumulator before repeating commands: {}", accumulator(commands));
    Ok(())

}

// Part 1
fn accumulator(commands: Vec<(&str, i32)>) -> i32{
    let mut index: i32 = 0; 
    let mut indices_visited: Vec<bool> = vec![false; commands.len()];
    let mut accumulator: i32 = 0;

    while !indices_visited[index as usize] {
        indices_visited[index as usize] = true;
        match commands[index as usize].0 {
            "acc" => accumulator += commands[index as usize].1,
            "jmp" => index += commands[index as usize].1 - 1,
            "nop" => {},
            _ => unreachable!(),
        }
        index += 1;

        if index as usize == commands.len() {
            break;
        }
    }
    accumulator
}

// Data Wrangling
fn parse_command(line: &str) -> (&str, i32) {
    let (operation, argument) = line.split_once(" ").unwrap();
    (operation, argument.parse::<i32>().unwrap())
}

