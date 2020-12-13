// 
// Advent of Code 2020: Day 12
//

use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let directions = parse_data("input.txt"); 

    let boat_pos = move_boat(&directions);

    println!("Manhattan distance for moving boat: {}", calc_manhattan(boat_pos));

    let boat_way_pos = move_waypoint(&directions);

    println!("Manhattan distance for moving waypoint: {}", calc_manhattan(boat_way_pos));

    println!("Time: {:?} ms", now.elapsed().as_millis());

    Ok(())
}

fn calc_manhattan(coord: (i32, i32)) -> i32 {
    coord.0.abs() + coord.1.abs()
}

// Part 1
fn move_boat(directions: &Vec<(char, i32)>) -> (i32, i32) {
    let (mut x, mut y, mut r) = (0, 0, 90);   

    for (dir, val) in directions {
        match dir {
           'N' => y += val,
           'S' => y -= val,
           'E' => x += val,
           'W' => x -= val,
           'L' => r -= val,
           'R' => r += val,
           'F' => match r.rem_euclid(360) {
               0 => y += val,
               90 => x += val,
               180 => y -= val,
               270 => x -= val,
               _ => unreachable!()
           }
           _ => continue,
        }

    }
    (x, y)
}

// Part 2
fn rotate(x: i32, y: i32, deg: i32) -> (i32, i32) {
    match deg {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unreachable!(),
    }
}

fn move_waypoint(directions: &Vec<(char, i32)>) -> (i32, i32) {
    let (mut x, mut y) = (10, 1);
    let mut ship_pos = (0, 0);

    for (dir, val) in directions {
        match dir {
           'N' => y += val,
           'S' => y -= val,
           'E' => x += val,
           'W' => x -= val,
           'L' => {
               let (x2, y2) = rotate(x, y, *val);
               x = x2;
               y = y2;
           }
           'R' => {
               let (x2, y2) = rotate(x, y, 360 - *val);
               x = x2;
               y = y2;
           }
           'F' => {
               ship_pos.0 += x * val;
               ship_pos.1 += y * val; 
           },
               _ => unreachable!()
           }
        }
    (ship_pos.0, ship_pos.1)
}

// Data wrangling
fn parse_data(filename: &str) -> Vec<(char, i32)> {
    let input = std::fs::read_to_string(filename).unwrap();    
    
    input
        .split_whitespace()
        .map(|dir| parse_command(dir))
        .collect::<Vec<_>>()
}

fn parse_command(line: &str) -> (char, i32) {
    let (name, value) = line.split_at(1); 
    (name.chars().next().unwrap(), value.parse::<i32>().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
       let directions = parse_data("input_test.txt"); 

       let final_pos = move_boat(&directions);

       assert_eq!(calc_manhattan(final_pos), 25);
    }

    #[test]
    fn test_part2() {
        let directions = parse_data("input_test.txt");

        let final_pos = move_waypoint(&directions);

        assert_eq!(calc_manhattan(final_pos), 286);
    }
}
