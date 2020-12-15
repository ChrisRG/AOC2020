//
// Advent of Code 2020: Day 15
//

use std::time::Instant;

fn main()  {
    let now = Instant::now();
    let nums = vec!(0,5,4,1,10,14,7);
    let last_round: usize = 2020;

    println!("Number spoken at round {} => {}", last_round, number_spoken(&nums, last_round));

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn number_spoken(numbers: &[usize], limit: usize) -> usize {
    let mut history: Vec<usize> = numbers.to_vec();
    for _ in numbers.len()..limit {
        let previous = history.last().unwrap();

        let last = history[..history.len() -1]
            .iter()
            .enumerate()
            .filter(|&(_, v)| v == previous)
            .last();
        
        let current = last
            .map(|(index, _)| history.len() - (index+1)).unwrap_or(0);

        history.push(current);

    }   
    return *history.last().unwrap();
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
}
