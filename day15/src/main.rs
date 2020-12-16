//
// Advent of Code 2020: Day 15
//

use std::time::Instant;

fn main()  {
    let now = Instant::now();
    let nums = vec!(0,5,4,1,10,14,7);

   // println!("Number spoken at round 2020 => {}", number_spoken(&nums, 2020));

    println!("Number spoke at round 30_000_000 => {}", number_spoken(&nums, 30_000_000));
    println!("Time: {}ms", now.elapsed().as_millis());
}

// Alternatively: Vec<Option<NonZeroU32>> => None = 0
// Using saturating_sub --> which gives back a 0 at lowest
fn number_spoken(numbers: &[u32], limit: usize) -> u32 {
    let mut history = vec![0u32; 1 + (limit as usize)];

    for (x, num) in numbers.iter().enumerate() {
        history[x] = *num;
    }
    for index in numbers.len()..limit {
        let previous = history[index - 1];

        let last = history[..index-1]
            .iter()
            .enumerate()
            .filter(|&(_, v)| *v == previous)
            .last();
        let current = last
            .map(|(last_idx, _)| index - (last_idx+1)).unwrap_or(0);

        history[index] = current as u32;
        
    }  
    history[limit-1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(number_spoken(&vec!(0, 3, 6), 2020), 436);
        assert_eq!(number_spoken(&vec!(1, 3, 2), 2020), 1);
        assert_eq!(number_spoken(&vec!(2, 1, 3), 2020), 10);
        assert_eq!(number_spoken(&vec!(1, 2, 3), 2020), 27);
        assert_eq!(number_spoken(&vec!(2, 3, 1), 2020), 78);
        assert_eq!(number_spoken(&vec!(3, 2, 1), 2020), 438);
        assert_eq!(number_spoken(&vec!(3, 1, 2), 2020), 1836);
    }

    #[test]
    fn test_part2() {
        assert_eq!(number_spoken(&vec!(0, 3, 6), 30_000_000), 175594);
    }
}
