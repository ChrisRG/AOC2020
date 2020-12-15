//
// Advent of Code 2020: Day 114
//
#![feature(str_split_once)]

use std::error::Error;
use std::time::Instant;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let addrs = parse_data("input_test.txt");
    let mem: HashMap<i64, i64> = HashMap::new();

    println!("0b8");
    for elem in addrs{
        if !elem.starts_with("X") {
            println!("{} -> {:b}", elem, elem.parse::<i64>().unwrap());
        }
    }
   // let word_size = 36 bits
   //   most significant, 2^35 on left, least significant, 2^0 on right
    // mem[8] = 11 => address 8, value 11
    // Apply bitmask before writing to memory
    // 0 or 1 overwritten, X leaves unchangeds
    // u64 = 0x ...Did 
    Ok(())
}

fn to_bin(decimal: &str)  {
}
fn parse_data(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut ops: Vec<String> = Vec::new();

    for line in input.split("mask = ").collect::<Vec<&str>>() {
        for (x, elem) in line.split("\n").enumerate() {
            if x == 0 && elem.len() > 0 {
                ops.push(elem.to_string());
            }
            if x >= 1 && elem.len() > 0 {
                let (addr, val) = elem.split_once(" = ").unwrap();
                let addr = &addr[4..addr.len()-1];

                ops.push(addr.to_string());
                ops.push(val.to_string());
            }
        }
    }
    ops
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {

    }
}
