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
            println!("{}", pass);
            boarding_passes.push(BoardingPass::new(pass));
        }
    }

    boarding_passes.sort_by_key(|pass| pass.get_seat());
    let highest_seat = boarding_passes.last().unwrap().get_seat();
    println!("Highest seat: {}", highest_seat);

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
            row: Self::binary_parse(split_input.0, 127.0),
            col: Self::binary_parse(split_input.1, 7.0),
        }
    }
    
    fn binary_parse(input: &str, mut upper: f32) -> i32 {
        let mut lower: f32 = 0.0;
        let code: Vec<char> = input.chars().collect();
        let mut parsed = 0;

        for (x, letter) in code.iter().enumerate() {
            let offset: f32 = (upper - lower) / 2.0; 
            if *letter == 'F' || *letter == 'L' {
                if x == code.len()-1 {
                    parsed = lower as i32;
                } else {
                    upper = (upper - offset).floor();
                }
            } else if *letter == 'B' || *letter == 'R' {
                if x == code.len() - 1 {
                    parsed = upper as i32;
                } else {
                lower = (lower + offset).ceil();
                }
            }
        }
        parsed
    }

    fn get_seat(&self) -> i32 {
        self.row * 8 + self.col
    }
}


