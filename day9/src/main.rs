//
// Advent of Code 2020: Day 9
//

use std::error::Error;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let window: usize = 25;

    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    for num in find_invalids(&numbers, window) {
        println!("Invalid: {}", num);
    }

    Ok(())
}

fn parse_num(number: &i64, window: &[i64]) -> bool {
    // Itertools.cartesian_product multiples all elements of two sets together
    // Here the window of numbers with itself => 2 element tuples of all numbers
    let result = window
        .iter()
        .cartesian_product(window)
        .find(|(l, r)| *l + *r == *number);
    if result == None { false } else { true }
}

// Part 1
fn find_invalids(numbers: &Vec<i64>, window: usize) -> Vec<i64> {
    let mut invalid_nums: Vec<i64> = Vec::new();

    for (x, num) in numbers.iter().enumerate() {
        if x < window {
            continue;
        }
        if !parse_num(num, &numbers[x-window..=x-1]) {
            invalid_nums.push(*num);
        }
    }
    invalid_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input_test.txt").unwrap();

        let numbers = input
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(find_invalids(&numbers, 5)[0], 127);
    }
}
