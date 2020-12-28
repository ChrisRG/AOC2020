// 
// Advent of Code 2020: Day 19
//

#![feature(str_split_once)]

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let now = Instant::now();
    let options = rule_options(parse_rules("test_rules.txt"), 0);
    let messages = parse_messages("test_messages.txt");
    let num_messages = check_matches(options, messages);

    println!("Number of messages matching rule 0: {}", num_messages);
    println!("Time: {}ms", now.elapsed().as_millis());
}


fn check_matches(options: Vec<String>, messages: Vec<String>) -> u32 {
    let num_matches: u32 = 0;
    // TODO
    // Go through available options from input
    // Check how many messages are equal 
    num_matches
}

fn rule_options(rules: HashMap<i32, Vec<Vec<String>>>, rule_num: i32) -> Vec<String> {
    let mut options: Vec<String> = Vec::new();
    // TODO
    // Check each "node" in the given rule
    // If node isn't alphabetical ('a' or 'b') then keep linking
    // Build up from base nodes until the top, then push String into Vec
    options
}

// Data Wrangling
fn parse_rules(filename: &str) -> HashMap<i32, Vec<Vec<String>>> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut rule_map: HashMap<i32, Vec<Vec<String>>> = HashMap::new();
    
    for line in input.lines() {
        let splitted = line.split_once(": ").unwrap();
        let rule_name = splitted.0.parse::<i32>().unwrap();
        let mut rules: Vec<Vec<String>> = Vec::new();

        for item in splitted.1.split(" | ") {
            rules.push(item
                       .split([' ', '"'].as_ref())
                       .map(|item| item.to_string())
                       .filter(|item| item != "")
                       .collect::<Vec<String>>());
        }
        rule_map.insert(rule_name, rules);
    }
    rule_map
}

fn parse_messages(filename: &str) -> Vec<String> {
    let input = std::fs::read_to_string(filename).unwrap();

    input.split_whitespace()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules() {
        let rules = parse_rules("test_rules.txt");

        println!("Testing rule parser: ");
        for rule in rules {
            println!("{:?}", rule);
        }
    }

    #[test]
    fn test_parse_messages() {
        let messages = parse_messages("test_messages.txt");

        println!("Testing message parser: ");
        for message in messages {
            println!("{:?}", message);
        }
    }

    fn test_rule() {
        // TODO
    }

    fn test_matches() {
        // TODO
    }

    fn test_part1() {
        let options = rule_options(parse_rules("test_rules.txt"), 0);
        let messages = parse_messages("test_messages.txt");
        let num_messages = check_matches(options, messages);

        println!("Number of messages matching rule 0: {}", num_messages);
    }
}
