//
// Advent of Code 2020: Day 17
//

use std::time::Instant;
use std::collections::HashSet;
use itertools::iproduct;

const DIRS: [i16; 3] = [-1, 0, 1];

fn main() {
    let now = Instant::now();
    let mut grid: Grid = Grid::new("input.txt");

    grid.cycle(5);
    println!("Active cells in 3 dimensions: {}", grid.actives.len());
    println!("Time: {}ms", now.elapsed().as_millis());
    
    let mut grid: Grid = Grid::new("input.txt");
    grid.hyper_cycle(5);
    println!("Active cells in 4 dimensions: {}", grid.actives.len());
    println!("Time: {}ms", now.elapsed().as_millis());
}

struct Grid {
    actives: HashSet<(i16, i16, i16, i16)>,
    new_actives: HashSet<(i16, i16, i16, i16)>,
}

impl Grid {
    // Part 1 -- 3 dimensions
    fn cycle(&mut self, total_rounds: i16) {
        for round in 0..=total_rounds {
            self.new_actives.clear();
            for x in -10-round..10+round {
                for y in -10-round..10+round {
                    for z in -2-round..2+round {
                        let act_neighbors = self.active_neighbors(&(x,y,z,0));
                        if self.actives.contains(&(x,y,z,0)) && 
                            (act_neighbors == 2 || act_neighbors == 3) {
                                self.new_actives.insert((x,y,z,0));
                        } else if !self.actives.contains(&(x,y,z,0)) &&
                            act_neighbors == 3 {
                                self.new_actives.insert((x,y,z,0));
                            }

                     }
                 }
             }  
            self.actives = self.new_actives.clone();
        }
    }
    // Part 2 - 4 dimensions
    fn hyper_cycle(&mut self, total_rounds: i16) {
        for round in 0..=total_rounds {
            self.new_actives.clear();
            for x in -10-round..10+round {
                for y in -10-round..10+round {
                    for z in -2-round..2+round {
                        for w in -2-round..2+round {
                            let act_neighbors = self.active_neighbors(&(x,y,z,w));
                            if self.actives.contains(&(x,y,z,w)) && 
                                (act_neighbors == 2 || act_neighbors == 3) {
                                    self.new_actives.insert((x,y,z,w));
                            } else if !self.actives.contains(&(x,y,z,w)) &&
                                act_neighbors == 3 {
                                    self.new_actives.insert((x,y,z,w));
                                }
                        }
                     }
                 }
             }  
            self.actives = self.new_actives.clone();
        }
    }

    fn active_neighbors(&self, point: &(i16,i16,i16,i16)) -> i16 {
        let mut count: i16 = 0;
        let neighbors = iproduct!(&DIRS, &DIRS, &DIRS, &DIRS)
          .map(|(dx, dy, dz, dw)| (dx+point.0, dy+point.1, dz+point.2, dw+point.3))
          .filter(|p| p != point)
          .collect::<Vec<_>>();
        for neighbor in neighbors {
            if self.actives.contains(&neighbor) {
                count += 1;
            }
        }
        count
    }

    fn new(filename: &str) -> Grid { 
        let mut actives: HashSet<(i16, i16, i16, i16)> = HashSet::new();
        let input = std::fs::read_to_string(filename).unwrap();

        let field = input.split_whitespace()
            .collect::<Vec<&str>>();

        for (i, row) in field.iter().enumerate() {
            for (j, col) in row.chars().enumerate() {
                if col == '#' {
                    actives.insert((i as i16, j as i16, 0 as i16, 0 as i16));
                }
            }
        }
        Grid { actives, new_actives: HashSet::new() }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid: Grid = Grid::new("input_test.txt");
        grid.cycle(5);
        let active_cubes = grid.actives.len();

        assert_eq!(active_cubes, 112);
    }

    #[test]
    fn test_part2() {
        let mut grid: Grid = Grid::new("input_test.txt");
        grid.hyper_cycle(5);
        let active_cubes = grid.actives.len();

        assert_eq!(active_cubes, 848);
    }
}
