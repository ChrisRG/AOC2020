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
        let hits = calculate_trees_hit(&area, starting_pos, *slope);
        println!("Number of trees hit for slope: [Right {}, Down {}] => {}", 
                 slope.1, slope.0, hits);
        product = product * hits as i64;
    }
    println!("Final product of trees hit: {}", product);
    Ok(())
}

fn calculate_trajectory(height: i32, width: i32, start: (i32, i32), slope: (i32, i32)) -> Vec<(i32, i32)> {
    let mut pos_list = Vec::new();
    let mut pos = start;
    for _ in start.0..height {
        if pos.0 >= height {
            break;
        }
        pos.0 = pos.0 + slope.0;
        if pos.1 + slope.1 > width {
            pos.1 = ((pos.1 + slope.1) - width) - 1;
        } else {
            pos.1 = pos.1 + slope.1;
        }
        pos_list.push(pos);
    }
    pos_list
}

fn calculate_trees_hit(area: &Vec<Vec<char>>, start: (i32, i32), slope: (i32, i32)) -> i32 {
    let pos_list: Vec<(i32, i32)> = 
        calculate_trajectory(
            (area.len() as i32) - 1, 
            (area[0].len() as i32) - 1,
            start, slope);

    let mut hits = 0;
    for pos in pos_list {
       if area[pos.0 as usize][pos.1 as usize] == '#' {
            hits += 1;
       }
    }
    hits
}

// Data wrangling
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
