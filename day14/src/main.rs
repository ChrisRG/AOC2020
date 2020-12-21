//
// Advent of Code 2020: Day 114
//
#![feature(str_split_once)]
#![feature(assoc_char_funcs)]

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let commands = parse_data("input.txt");

    let final_sum: u64 =  initialize(commands)
        .values()
        .sum();

    println!("Sum of values in memory: {}", final_sum);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn initialize(commands: Vec<String>) -> HashMap<i32, u64> {
    let mut memory: HashMap<i32, u64> = HashMap::new();
    let mut mask: String = String::new();
    for comm in commands {
        if comm.len() == 36 {
            mask = comm;
        } else {
            let (addr, val) = comm.split_once("=").unwrap();
            memory.insert(addr.parse::<i32>().unwrap(),
                apply_mask(&mask, &val));
        } 
    }
    memory
}

fn apply_mask(mask: &str, val_str: &str) -> u64 {
    let mut bits: Vec<char> = to_binary(val_str);

    for (idx, char) in mask.chars().enumerate() {
        match char {
            '1' => bits[idx] = '1',
            '0' => bits[idx] = '0',
            _ => {},
        }
    }
    let new_bin = bits.into_iter()
        .collect::<String>();

    u64::from_str_radix(&new_bin, 2).unwrap()
}

fn to_binary(number: &str) -> Vec<char> {
    let mut binary: Vec<char> = Vec::new();
    let mut decimal = number.parse::<u32>().unwrap();

    while decimal != 0 {
        binary.push(char::from_digit(decimal % 2, 10).unwrap());
        decimal = decimal / 2;
    }

    binary.reverse();

    while binary.len() != 36 {
        binary.insert(0, '0');
    }
    binary
}

fn parse_data(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut commands: Vec<String> = Vec::new();

    for line in input.split("mask = ").collect::<Vec<&str>>() {
        for (x, elem) in line.split("\n").enumerate() {
            if x == 0 && elem.len() > 0 {
                commands.push(elem.to_string());
            }
            if x >= 1 && elem.len() > 0 {
                let (addr, val) = elem.split_once(" = ").unwrap();
                let addr = &addr[4..addr.len()-1];

                commands.push(addr.to_string() + "=" + &val.to_string());
            }
        }
    }
    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let commands = parse_data("input_test.txt");

        let final_sum: u64 =  initialize(commands)
            .values()
            .sum();

        assert_eq!(final_sum, 165);
    }

}
