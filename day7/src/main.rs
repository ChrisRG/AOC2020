//
// Advent of Code 2020: Day 7
//
#![feature(str_split_once)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let bags = input.lines()
        .map(|line| Bag::parse(line))
        .collect::<Vec<_>>();

    let target = "shiny gold";
    let mut possible_bags: Vec<&Bag> = Vec::new();
    let mut total_contained: u32 = 0;
    
    contains_color(&bags, target, &mut possible_bags); 
    contains_number(&bags, target, &mut total_contained);

    possible_bags.sort_by_key(|bag| bag.color);
    possible_bags.dedup_by_key(|bag| bag.color);

    println!("Number of possible bags for `{}`: {}", target, possible_bags.len());

    Ok(())
}

fn contains_color<'a>(bags: &'a Vec<Bag<'a>>, 
                      target: &str, 
                      final_list: &mut Vec<&'a Bag<'a>>) {
    for bag in bags
        .iter()
        .filter(|bag| bag.rules.iter().any(|rule| rule.color == target)) {
            final_list.push(bag);
            contains_color(&bags, bag.color, final_list);
        }
}

fn contains_number<'a>(bags: &'a Vec<Bag<'a>>,
                       target: &str,
                       final_count: &mut u32) {
    // TODO
}

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
    rules: Vec<BagRule<'a>>,
}

impl<'a> Bag<'a> {
    fn parse(line: &'a str) -> Bag {
        let (color, items) = line.split_once(" bags contain ").unwrap();
        let mut rules = Vec::new();

        if !items.starts_with("no") {
            for rule in items.split(", ") {
                let (number, color) = rule.split_once(" ").unwrap();
                let (color, _) = color.split_once(" bag").unwrap();

                let number = number.parse::<u32>().unwrap();

                rules.push(BagRule { number, color });
            }
        }
        
        Bag { color, rules }
    }
}

#[derive(Debug)]
struct BagRule<'a> {
    number: u32,
    color: &'a str,
}

