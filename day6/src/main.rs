// 
// Advent of Code 2020: Day 6
//

use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let groups = input.split("\n\n").collect::<Vec<_>>();

    let final_count = groups
        .iter()
        .map(|group| calculate_answers(group))
        .fold(0, |acc, x| acc + x);

    println!("Total answers for {} groups: {}", groups.len(), final_count);

    Ok(())
}

fn calculate_answers(input: &str) -> i32 {
    let lines = input.split_whitespace().collect::<Vec<_>>();
    let mut answers: HashSet<char> = HashSet::new();
    let merged: Vec<char>  = lines
        .iter()
        .map(|s| s.chars())
        .flatten().collect();

    for letter in merged {
        answers.insert(letter);
    }

    answers.len() as i32
}
