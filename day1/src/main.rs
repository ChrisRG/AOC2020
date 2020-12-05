// Advent of Code 2020: Day 1
//
// Part 1 Problem:
// In a list of numbers find the two entries that sum to 2020 and then multiply those two numbers together.
//
// Part 2 Problem:
// With the same list of numbers, find the three entries that sum to 2020 and output their
// product.
// 
use std::io::Read;
use std::error::Error;
use itertools::Itertools;

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
   
    two_numbers(&numbers);
    three_numbers(&numbers);
    two_combination(&numbers);
    Ok(())
}

// Subsequent refactoring in a more functional style
fn two_combination(numbers: &Vec<i32>) { 
    let result = numbers
        .iter()
        .combinations(2)
        .find(|window| window[0] + window[1] == 2020)
        .unwrap();

    println!("Combination of two: {:?}", result);
}




// Part 1
fn two_numbers(numbers: &Vec<i32>) {
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
                println!("Part 1: Two numbers");
                println!("{} + {} = 2020", numbers[x], numbers[y]);
                println!("Product: {}", numbers[x] * numbers[y]);
                break;
            }
        }
    }
}

// Part 2
fn three_numbers(numbers: &Vec<i32>) {
    // Same structure as two_numbers
    // Even uglier!
    let mut mid_sum: i32;

    let mut third_num: i32;

    for x in 0..numbers.len() {
        mid_sum = 2020 - numbers[x];

        for y in x+1..numbers.len() {
            if mid_sum - numbers[y] > 0 {
                third_num = mid_sum - numbers[y];
                for z in y+1..numbers.len() {
                    if numbers[z] == third_num {
                        println!("Part 2: Three numbers");
                        println!("Numbers: {}, {}, {}", numbers[x], numbers[y], numbers[z]);
                        println!("Product: {}", numbers[x] * numbers[y] * numbers[z]);
                        break;
                    }
                }
            }
        }
    }
}
