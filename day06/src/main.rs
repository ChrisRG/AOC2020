// 
// Advent of Code 2020: Day 6
//

use std::error::Error;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let groups = input.split("\n\n").collect::<Vec<_>>();

    let anyone_count = groups
        .iter()
        .map(|group| answers_from_anyone(group))
        .fold(0, |acc, x| acc + x);

    let everyone_count = groups
        .iter()
        .map(|group| answers_from_everyone(group))
        .fold(0, |acc, x| acc + x);

    println!("Total unique answers for {} groups: {}", groups.len(), anyone_count);

    println!("Total unique shared answers: {}", everyone_count);
    Ok(())
}

fn answers_from_anyone(input: &str) -> i32 {
    let group = input.split_whitespace().collect::<Vec<_>>();
    let total  = group
        .iter()
        .map(|s| s.chars())
        .flatten().collect::<HashSet<_>>()
        .len();

    total as i32
}

fn answers_from_everyone(input: &str) -> i32 {
    let group = input.split_whitespace().collect::<Vec<_>>();
    let mut group_sets = group
        .into_iter()
        .map(|person| {
            person.chars().collect::<HashSet<_>>() });

    let mut final_set = group_sets.next().unwrap();

    for set in group_sets {
        final_set = final_set.intersection(&set).copied().collect();
    }
    final_set.len() as i32
}
