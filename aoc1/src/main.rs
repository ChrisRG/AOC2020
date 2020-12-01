// Advent of Code 2020: Day 1, Problem 1 
//
// Description:
// In a list of numbers find the two entries that sum to 2020 and then multiply those two numbers together.
//
// To run: cargo run
// 
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Standard method for opening a file and storing it as one string.
    // Lines separated by `\n` 
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // For a new vector of integers, split the contents string by newlines.
    // Iterate through and map each element with parse, converting to integers.
    // If parse succeeds, filter and pass on to be collected in the vector.
    let numbers: Vec<i32> = contents.split("\n")
        .filter_map(|w| w.parse().ok())
        .collect();
   
    two_numbers(numbers);

    Ok(())
}


// Part 1
fn two_numbers(numbers: Vec<i32>) {
    // Brute force algorithm:
    // Declare a mutable variable to be used together with each element in vector.
    let mut second_num: i32;

    for x in 0..numbers.len() {
        // For each element, subtract that number from 2020
        // And save difference in second_num.
        second_num = 2020 - numbers[x];

        // Pass through remaining elements: 
        for y in x+1..numbers.len() {
            // If second number found, print both numbers and their product.
            if numbers[y] == second_num {
                println!("Found pair: {} + {} = 2020", numbers[x], numbers[y]);
                println!("Product of pair: {}", numbers[x] * numbers[y]);
                break;
            }
        }
    }
}
