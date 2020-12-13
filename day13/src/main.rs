//
// Advent of Code 2020: Day 13
//

use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let buses = parse_data("input.txt");
    let time: i64 = 1000303;

    let (bus_id, minutes) = calculate_next(time, buses);
    println!("Bus ID: {} | Minutes to wait: {} | Total: {}",
             bus_id, minutes, bus_id*minutes);

    println!("Time: {:?} ms", now.elapsed().as_millis());

    Ok(())
}

// Part 1
fn calculate_next(time: i64, buses: Vec<i64>) -> (i64, i64) {
    let mut bus_minutes: Vec<(i64, i64)> = Vec::new();
    for bus in buses {
        bus_minutes.push((bus, bus - time % bus));
    }
    let next_bus = bus_minutes
        .iter()
        .min_by_key(|(_, time)| time).unwrap();
    *next_bus
}

// Data wrangling
fn parse_data(filename: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(filename).unwrap();

    let mut buses: Vec<i64> = input
        .split([',', '\n', ' '].as_ref())
        .filter(|&c| c != "x" && c != "")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    buses.sort();
    buses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let time: i64 = 939;
        let buses = parse_data("input_test.txt");

        let (bus_id, minutes) = calculate_next(time, buses);

        assert_eq!(bus_id*minutes, 295);
    }
}
