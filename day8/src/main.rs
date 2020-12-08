//
// Advent of Code 2020: Day 8
//
#![feature(str_split_once)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut commands: Vec<(&str, i32)> = input
        .lines()
        .map(|line| parse_command(line))
        .collect();

    let run_until_repeat = run(&commands).1;

    println!("Accumulator before repeating commands: {}", run_until_repeat);
    
    run_swapping(&mut commands);
    Ok(())
}

// Part 1
fn run(commands: &Vec<(&str, i32)>) -> (Vec<usize>, i32) {
    let mut index: usize = 0; 
    let mut indices_visited: Vec<usize> = Vec::with_capacity(commands.len());
    let mut accumulator: i32 = 0;

    while !indices_visited.contains(&index) {
        indices_visited.push(index);
        match commands[index].0 {
            "acc" => accumulator += commands[index].1,
            "jmp" => index += commands[index].1 as usize - 1,
            "nop" => { },
            _ => unreachable!(),
        }
        index += 1;

        if index == commands.len() {
            break;
        }
    }
    (indices_visited, accumulator)
}

// Part 2
fn swap_jmpnop(command: &mut (&str, i32)) {
    match command.0 {
       "jmp" => command.0 = "nop",
       "nop" => command.0 = "jmp",
       _ => unreachable!(),
    }
}

fn run_swapping(commands: &mut Vec<(&str, i32)>) {
    for visited in run(&commands).0 {

       if commands[visited].0 == "jmp" || commands[visited].0 == "nop" {

           swap_jmpnop(&mut commands[visited]);
           let (indices, acc) = run(&commands);

           if indices[indices.len()-1] as usize == commands.len()-1 {
               println!("Accumulator by end after swapping: {}", acc);
               break;
           }

           swap_jmpnop(&mut commands[visited]);
       }
    }
}

// Data Wrangling
fn parse_command(line: &str) -> (&str, i32) {
    let (operation, argument) = line.split_once(" ").unwrap();
    (operation, argument.parse::<i32>().unwrap())
}

