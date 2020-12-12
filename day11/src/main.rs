// 
// Advent of Code 2020: Day 11
//

use std::error::Error;
use std::fmt;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let mut field = parse_data("input.txt");

    let now = Instant::now();

    while field.deltas > 0 {
        field.update();
    }
    println!("Final count: {} seats occupied", &field.count_occupied());
    println!("Time: {:?}ms", now.elapsed().as_millis());                           
    
    Ok(())
}

struct Field {
    height: i32,
    width: i32,
    area: Vec<char>,
    deltas: i32,
}

impl Field {
    fn get_index(&self, row: i32, col: i32) -> usize {
        (row * self.width + col) as usize 
    }

    fn neighbor_count(&self, row: i32, col: i32) -> u8 {
        let mut count = 0;

        for window_row in row-1..=row+1 {
            for window_col in col-1..=col+1 {
                if window_row < 0 || window_row >= self.height {
                    continue;
                } else if window_col < 0 || window_col >= self.width {
                    continue;
                } else if window_row == row && window_col == col {
                    continue;
                } else {
                    let index = self.get_index(window_row, window_col);
                    if self.area[index] == '#' {
                        count += 1;
                    }
                }

            }
        }
        count
    }

    fn update(&mut self) {
        let mut next_area = self.area.clone();
        let mut changes = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let place = self.area[index];
                let neighbor_count = self.neighbor_count(row, col);

                let next_place: char;
                match (place, neighbor_count) {
                    ('L', 0) => { next_place = '#'; changes += 1 },
                    ('#', x) if x >= 4 => { next_place = 'L'; changes += 1 },
                    (otherwise, _) => next_place = otherwise
                };

                next_area[index] = next_place;
            }
        }
        self.area = next_area;
        self.deltas = changes;
    }

    fn count_occupied(&self) -> u32 {
        let mut count = 0;
        for cell in self.area.iter() {
            match cell { 
                '#' => count += 1,
                _ => continue,
            }
        }
        count
    }
}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.area.as_slice().chunks(self.width as usize) {
            for &place in line {
                write!(f, "{}", place)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


// Data wrangling
fn parse_data(filename: &str) -> Field {
    let input = std::fs::read_to_string(filename).unwrap();
    let field = input
        .split_whitespace()
        .collect::<Vec<&str>>();

    let area = field
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<_>>();

    Field {
        height: field.len() as i32,
        width: field[0].len() as i32,
        area,
        deltas: 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut field = parse_data("input_test.txt");

        while field.deltas > 0 {
            field.update();
        }

        assert_eq!(field.count_occupied(), 37);
    }
}
