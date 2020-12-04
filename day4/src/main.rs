// 
// Advent of Code 2020: Day 4
//
use std::error::Error;
use std::collections::HashMap;
use itertools::Itertools;

static FIELDS: [&str; 7]  = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let passports = input.split("\n\n").collect::<Vec<&str>>();

    let parsable_passports = passports.iter()
        .filter_map(|&pass| parse_passport(pass))
        .collect::<Vec<_>>();

    let valid_passports = parsable_passports.iter()
        .filter(|&pass| is_valid(pass))
        .collect::<Vec<_>>();

    println!("Total passports: {}", passports.len());
    println!("Total correctly structured passports: {}", parsable_passports.len());
    println!("Total structured and valid passports: {}", valid_passports.len());
    Ok(())
}

// Part 1
fn parse_passport(s: &str) -> Option<HashMap<&str, &str>> {
    let passport = s.split_whitespace()
        .flat_map(|x| x.split(':'))
        .tuples()
        .collect::<HashMap<_, _>>();

    let missing_field = FIELDS.iter()
        .any(|code| !passport.contains_key(code));

    if missing_field { return None; }
    Some(passport)
}

// Part 2
fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap()),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap()),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap()),
        "hgt" => valid_hgt(&v),
        "hcl" => &v[..1] == "#" && v[1..].chars().all(|c| c.is_ascii_hexdigit()),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => unreachable!() 
    })
}

fn valid_hgt(input: &str) -> bool { 
    let index: usize = input.len() - 2;
    let height = &input.split_at(index);
    match height.1 {
        "cm" => (150..=193).contains(&height.0.parse().unwrap()),
        "in" => (59..=76).contains(&height.0.parse().unwrap()),
        _ => false
    }
}



