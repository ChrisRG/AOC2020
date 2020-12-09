//
// Advent of Code 2020: Day 9
//

use std::error::Error;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let numbers = parse_data("input.txt");
    let window: usize = 25;

    println!("Invalid: {}", find_invalid(&numbers, window));

    println!("Encryption weakness: {}", find_weakness(&numbers, window).unwrap());

    Ok(())
}

fn parse_data(file_name: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(file_name).unwrap();
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect();
    numbers
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
fn find_invalid(numbers: &Vec<i64>, window: usize) -> i64 {
    let mut invalid_num: i64 = 0;

    for (x, num) in numbers.iter().enumerate() {
        if x < window {
            continue;
        }
        if !parse_num(num, &numbers[x-window..=x-1]) {
            invalid_num = *num;
        }
    }
    invalid_num
}

// Part 2
fn find_weakness(numbers: &Vec<i64>, window: usize) -> Option<i64> {
    let weak_num = find_invalid(&numbers, window);

    let mut num_series: Vec<i64> = Vec::new();

    for (x, num) in numbers.iter().enumerate() {
        num_series.push(*num);

        for next_num in numbers[x+1..].iter() {
            num_series.push(*next_num);
            let sum = num_series.iter().sum::<i64>();

            if sum == weak_num {
                num_series.sort();
                return Some(num_series.first().unwrap() 
                            + num_series.last().unwrap());

            } else if sum > weak_num {
                num_series.clear();
                break;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let numbers = parse_data("input_test.txt");
        assert_eq!(find_invalid(&numbers, 5), 127);
    }

    #[test]
    fn test_part2() {
        let numbers = parse_data("input_test.txt");
        assert_eq!(find_weakness(&numbers, 5).unwrap(), 62);
    }
}
