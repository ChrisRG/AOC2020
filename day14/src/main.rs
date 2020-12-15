//
// Advent of Code 2020: Day 114
//
#![feature(str_split_once)]

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let addrs = parse_data("input_test.txt");
    let mem: HashMap<i64, i64> = HashMap::new();

}

fn to_bin(decimal: &str)  {
    // TODO
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
