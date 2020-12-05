// 
// Advent of Code 2020: Day 5
//

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let passes = input.split("\n").collect::<Vec<&str>>();
    let mut boarding_passes: Vec<BoardingPass> = Vec::new();

    for pass in passes {
        if pass != "" {
            boarding_passes.push(BoardingPass::new(pass));
        }
    }

    boarding_passes.sort_by_key(|pass| pass.get_seat());
    let highest_seat = boarding_passes.last().unwrap().get_seat();

    // Part 1
    println!("Highest seat: {}", highest_seat);

    // Part 2
    let missing_seat = missing_numbers(boarding_passes);
    println!("Missing seat, probably yours: {}", missing_seat);

    Ok(())
}

struct BoardingPass {
    row: i32,
    col: i32,
}

impl BoardingPass {
    fn new(input: &str) -> Self {
        let split_input = input.split_at(input.len() - 3);
        BoardingPass{
            row: Self::binary_parse(split_input.0, 0.0, 127.0),
            col: Self::binary_parse(split_input.1, 0.0, 7.0),
        }
    }
    
    fn binary_parse(input: &str, mut lower: f32, mut upper: f32) -> i32 {
        let sequence: Vec<char> = input.chars().collect();

        for letter in sequence.iter() {
            let offset: f32 = (upper - lower) / 2.0;
            match letter {
                'F' | 'L' => upper = (upper - offset).floor(),
                'B' | 'R' => lower = (lower + offset).ceil(),
                _ => unreachable!(),
            }
        }   
        upper as i32 
    }

    fn get_seat(&self) -> i32 {
        self.row * 8 + self.col
    }
}

fn missing_numbers(mut input: Vec<BoardingPass>) -> i32 {
    let gap = input
        .windows(2)
        .find(|window| window[0].get_seat() + 1 != window[1].get_seat())
        .unwrap();
    gap[0].get_seat() + 1
}


