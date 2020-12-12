// 
// Advent of Code 2020: Day 11
//

use std::error::Error;
use std::time::Instant;

static DIRS: [(i64, i64); 8] = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];

fn main() -> Result<(), Box<dyn Error>> {
    let mut field = parse_data("input.txt");

    let now = Instant::now();

    println!("Final count with adjacent seats: {} occupied", &field.update(false));

    // Since I originally made this with a struct, we need to reset the data
    // Not very efficient, but now can't be bothered to change it
    field = parse_data("input.txt");
    println!("Final count with line of sight seats: {} occupied", &field.update(true));
    println!("Time: {:?}ms", now.elapsed().as_millis());           
    
    Ok(())
}

struct Field {
    height: usize,
    width: usize,
    area: Vec<Vec<char>>,
}

impl Field {
    fn update(&mut self, in_view: bool) -> usize {
        let mut swap = Vec::new();
        
        loop { 
            swap.clear();
            for y in 0..self.height {
                for x in 0..self.width {
                    if !in_view {
                        if self.area[y][x] != '.' && self.to_swap(y, x) {
                            swap.push((y, x));
                        }
                    } else {
                        if self.area[y][x] != '.' && self.to_swap_inview(y, x) {
                            swap.push((y, x));
                        }
                    }
                }
            }

            for &(y, x) in &swap {
                self.area[y][x] = if self.area[y][x] == 'L' { '#' } else { 'L' };
            }

            if swap.is_empty() { break; }
        }
        self.area
            .iter()
            .flat_map(|row| row.iter()).filter(|&&c| c == '#')
            .count()
    }

// Part 1
// Refactored the functionality learn how to implement a more elegant and "rusty" solution found on r/rust
// Here:  https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/11.rs
    fn to_swap(&self, row: usize, col: usize) -> bool {
        let mut neighbors = DIRS.iter()
            .map(|(dy, dx)| (row as i64 + dy, col as i64 + dx))
            .filter_map(|(y,x)| self.area.get(y as usize).and_then(|col| col.get(x as usize)));
        
        match self.area[row][col] {
            'L' => neighbors.all(|&c| c != '#'),
            '#' => neighbors.filter(|&&c| c == '#').count() >= 4,
            _ => unreachable!()
        }
    }

// Part 2
// This part is almost entirely based on the repo mentioned above
    fn to_swap_inview(&self, y: usize, x: usize) -> bool {
        let mut neighbors = DIRS.iter()
            .filter_map(|&dir| self.find_neighbors(dir, (y, x)));

        match self.area[y][x] {
            'L' => neighbors.all(|c| c != '#'),
            '#' => neighbors.filter(|&c| c == '#').count() >= 5,
            _ => unreachable!()
        }
    }

    fn find_neighbors(&self, (dy, dx): (i64, i64), (y, x): (usize, usize)) -> Option<char> {
        let (mut y, mut x) = (y as i64, x as i64);

        loop {
            y += dy;
            x += dx;
            
            match self.area.get(y as usize).and_then(|col| col.get(x as usize)) {
                Some('.') => {},
                Some(&c) => return Some(c),
                None => break,
            }
        }
        None
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
        .collect::<Vec<Vec<char>>>();

    Field {
        height: field.len(),
        width: field[0].len(),
        area,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut field = parse_data("input_test.txt");

        assert_eq!(field.update(false), 37);
    }

    #[test]
    fn test_part2() {
        let mut field = parse_data("input_test.txt");

        assert_eq!(field.update(true), 26);
    }
}
