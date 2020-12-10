// 
// Advent of Code 2020: Day 10
//

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_data("input.txt");

    let product =
        jolt_difference(&input, 1) * jolt_difference(&input, 3);

   println!("Product of 1-jolt and 3-jolt differences: {}", product);
    Ok(())
}

fn parse_data(filename: &str) -> Vec<i32> {
    let input = std::fs::read_to_string(filename).unwrap();
    
    let mut jolts = input.split_whitespace()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    jolts.sort();
    jolts
}

// Part 1
fn jolt_difference(input: &[i32], target: i32) -> i32 {
    let mut valid_jolts: Vec<i32> = Vec::new();

    for (x, jolt) in input.iter().enumerate() {
        if x == 0 && *jolt == target {
            valid_jolts.push(*jolt);
        }
        if x == input.len()-1 {
            break;
        }
        if input[x+1] - jolt == target {
            valid_jolts.push(*jolt)
        }
    }
    if target == 3 {
        valid_jolts.push(*input.last().unwrap());
    }
    return valid_jolts.len() as i32;
}

// Part 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_data("input_test.txt");

        assert_eq!(jolt_difference(&input, 1), 22);
        assert_eq!(jolt_difference(&input, 3) + 1, 10);
    }

}
