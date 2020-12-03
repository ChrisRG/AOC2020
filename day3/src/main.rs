//
// Advent of Code: Day 3
//
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let area = parse_lines(parse_file("input.txt"));
    let starting_pos = (0, 0);
    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut product: i64 = 1;

    for slope in slopes.iter() {
        let encounters = calculate_tree_encounters(&area, starting_pos, *slope);
        println!("Number of trees encountered for slope: [Right {}, Down {}] => {}", 
                 slope.1, slope.0, encounters);
        product = product * encounters as i64;
    }
    println!("Final product of trees hit: {}", product);
    Ok(())
}

fn calculate_tree_encounters(area: &Vec<Vec<char>>, start: (i32, i32), slope: (i32, i32)) -> i32 {
    let height = (area.len() as i32) - 1;
    let width = (area[0].len() as i32) - 1;
    let mut encounters = 0;
    let mut pos = start;

    for _ in start.0..height {
        // Set position (y, x) according to slope, break if at bottom
        if pos.0 >= height {
            break;
        } else { 
            pos.0 = pos.0 + slope.0;
        }
        // If we hit the right edge, loop back to the left: 
        //      (positionX + slopeX) - width - 1 [since it starts at 0]
        if pos.1 + slope.1 > width {
            pos.1 = ((pos.1 + slope.1) - width) - 1;
        } else {
            pos.1 = pos.1 + slope.1;
        }
        // Did we encounter a tree?
        if area[pos.0 as usize][pos.1 as usize] == '#' {
            encounters += 1;
        }
    }
    encounters
}

// Data wrangling
// Convert 1D array of strings into 2D array of chars
fn parse_lines(input: Vec<String>) -> Vec<Vec<char>> {
    let area = input
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    area
}

fn parse_file(file_name: &str) -> Vec<String> {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines()
        .filter_map(|w| Some(w.to_string()))
        .collect::<Vec<String>>()

}
