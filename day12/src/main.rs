// 
// Advent of Code 2020: Day 12
//

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
   let directions = parse_data("input.txt"); 

   let final_pos = move_boat(&mut (0, 0, 90), directions);

   println!("Manhattan distance: {}", calc_manhattan(final_pos));

   Ok(())
}

fn calc_manhattan(coord: (i32, i32, i32)) -> i32 {
    coord.0.abs() + coord.1.abs()
}

fn move_boat((x, y, r): &mut (i32, i32, i32), directions: Vec<(char, i32)>) -> (i32, i32, i32) {

    for (dir, val) in directions {
        println!("X: {}, Y: {}, Rot: {}", x, y, r);
        println!("Dir: {}, val: {}", dir, val);
        match dir {
           'N' => *y += val,
           'S' => *y -= val,
           'E' => *x += val,
           'W' => *x -= val,
           'L' => *r -= val,
           'R' => *r += val,
           'F' => match r.rem_euclid(360) {
               0 => *y += val,
               90 => *x += val,
               180 => *y -= val,
               270 => *x -= val,
               _ => unreachable!()
           }
           _ => continue,
        }

    }
    println!("Final => X: {}, Y: {}, Rot: {}", x, y, r);
    (*x, *y, *r)

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

       let final_pos = move_boat(&mut (0, 0, 90), directions);

       assert_eq!(calc_manhattan(final_pos), 25);
    }
}
