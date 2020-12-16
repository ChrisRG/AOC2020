//
// Advent of Code: Day 16
//
#![feature(str_split_once)]

use std::time::Instant;

fn main() {
    let now = Instant::now();

    let (rules, my_ticket, near_tickets) = parse_data("input.txt");

    println!("Ticket scanning error rate: {}", invalid_fields(&near_tickets, &rules));
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[derive(Debug)]
struct Rule {
    name: String,
    range1: (u32, u32),
    range2: (u32, u32),
}

fn invalid_fields(tickets: &Vec<Vec<u32>>, rules: &[Rule]) -> u32 {
    let mut invalid_fields: Vec<u32> = Vec::new();
    for tick in tickets {
        for field in tick {
            if !is_valid(*field, &rules) { invalid_fields.push(*field) }
        }
    }
    return invalid_fields.iter().sum::<u32>();
}

fn is_valid(field: u32, rules: &[Rule]) -> bool {
   for rule in rules {
       if (rule.range1.0..=rule.range1.1).contains(&field)
           || (rule.range2.0..=rule.range2.1).contains(&field) {
               return true;
        }
    }
    false
}

fn parse_data(filename: &str) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut rules: Vec<Rule> = Vec::new();
    let mut near_tickets: Vec<Vec<u32>> = Vec::new();
    let splitted: Vec<&str> = input.split("\n\n").collect();

    for rule in splitted[0].lines() {
        let rname_split: Vec<&str> = rule.trim().split(':').collect();
        let rname = rname_split[0];
        let r_split = rname_split[1].split_whitespace().collect::<Vec<_>>();
        let r1 = r_split[0].trim().split_once('-').unwrap();
        let r2 = r_split[2].trim().split_once('-').unwrap();

        rules.push(Rule {
            name: rname[..rname.len()-1].to_string(),
            range1: (r1.0.parse::<u32>().unwrap(), r1.1.parse::<u32>().unwrap()),
            range2: (r2.0.parse::<u32>().unwrap(), r2.1.parse::<u32>().unwrap()),
        });
    }

    let my_ticket = splitted[1].lines().nth(1).unwrap().split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    for line in splitted[2].lines().skip(1) {
        let ticket = line.split(',')
            .map(|c| c.parse::<u32>().unwrap()) 
            .collect::<Vec<u32>>();
        near_tickets.push(ticket);
    }

    (rules, my_ticket, near_tickets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (rules, _, near_tickets) = parse_data("input_test.txt");

        assert_eq!(invalid_fields(&near_tickets, &rules), 71);
    }
    #[test]
    fn test_part2() {
        let (rules, my_ticket, near_tickets) = parse_data("input_test.txt");
        
        // TODO
    }
}
