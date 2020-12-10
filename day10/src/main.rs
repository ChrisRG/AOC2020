// 
// Advent of Code 2020: Day 10
//

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_data("input.txt");

    let product =
        jolt_difference(&input, 1).len() * jolt_difference(&input, 3).len();

    let total_combos =
        permutations(&input);

    println!("Product of 1-jolt and 3-jolt differences: {}", product);
   
    println!("Total distinct ways to arrange adapters: {}", total_combos);
    Ok(())
}

fn parse_data(filename: &str) -> Vec<i32> {
    let input = std::fs::read_to_string(filename).unwrap();
    
    let mut jolts = input.split_whitespace()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    // Need to add 0 and max adapter + 3
    jolts.push(0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();
    jolts
}

// Part 1
fn jolt_difference(input: &[i32], target: i32) -> Vec<i32> {
    let mut valid_jolts: Vec<i32> = Vec::new();

    for (x, jolt) in input.iter().enumerate() {
        if x+1 == input.len() {
            break;
        }
        if input[x+1] - jolt == target {
            valid_jolts.push(*jolt);
        }
    }
    valid_jolts
}

// Part 2
// Dynamic programming still boggles my mind.
// I took this from several solutions I found with much smarter people on rust discord.
fn permutations(numbers: &[i32]) -> u64 {
    let mut paths = vec![0u64; numbers.len()];
	paths[0] = 1;

	for i in 0..numbers.len() {
		let current_num = numbers[i];
		let current_paths = paths[i];

		for offset in 1..=3 {
			if let Some(&x) = numbers.get(i + offset) {
				if x <= current_num + 3 {
					paths[i + offset] += current_paths;
				}
			}
		}
	}

	return *paths.last().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse_data("input_test.txt");

        assert_eq!(jolt_difference(&input, 1).len(), 22);
        assert_eq!(jolt_difference(&input, 3).len(), 10);
    }

    #[test]
    fn test_part2() {
        let input = parse_data("input_test.txt");

        assert_eq!(permutations(&input), 19208);
    }   
}
