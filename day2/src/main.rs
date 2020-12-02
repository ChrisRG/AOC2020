// Advent of Code 2020: Day 2 
//
// Part 1 Problem:
//  Each line gives the password policy and then the password. 
//  The password policy indicates the lowest and highest number of times 
//  a given letter must appear for the password to be valid. 
//  For example, `1-3 a` means that the password must contain `a` at least 
//  1 time and at most 3 times.  
//
// Part 2 Problem:
//  Each policy actually describes two positions in the password, 
//  where 1 means the first character, 2 means the second character, 
//  and so on. (Be careful; Toboggan Corporate Policies have no concept of 
//  "index zero"!) Exactly one of these positions must contain the given 
//  letter. Other occurrences of the letter are irrelevant for the purposes 
//  of policy enforcement.

use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = parse_file("input.txt");
    let passwords: Vec<Password> = parse_lines(input); 
    let mut old_count = 0;
    let mut new_count = 0;

    for password in passwords {
       if old_validate_password(&password) {
           old_count += 1;
       }
       if new_validate_password(&password) {
           new_count += 1;
       }
    }
    println!("Part 1: Final count of valid passwords in old system: {}", old_count);
    println!("Part 2: Final count of valid passwords in new system: {}", new_count); 
    Ok(())
}

struct Password {
    range: Vec<usize>,
    letter: char,
    password: String,
}

// Part 1
fn old_validate_password(pass: &Password) -> bool {
    let min = pass.range[0];
    let max = pass.range[1];
    let password = &pass.password;
    let number_matches = password.matches(pass.letter)
        .collect::<Vec<&str>>().len();

    if number_matches >= min && number_matches <= max {
        true
    } else {
        false
    }
}

// Part 2
fn new_validate_password(pass: &Password) -> bool {
    let pos1 = pass.range[0] - 1;
    let pos2 = pass.range[1] - 1;
    let letter = pass.letter;
    let chars: Vec<char> = pass.password.chars().collect();

    if (chars[pos1-1] == letter && chars[pos2-1] != letter) ||
        (chars[pos1-1] != letter && chars[pos2-1] == letter) {
        true
    } else {
        false
    }
}

fn parse_lines(input: Vec<String>) -> Vec<Password> {
    let mut password_list: Vec<Password> = Vec::new();
    let mut split_strings: Vec<&str>;

    for x in 0..input.len() {
        split_strings = input[x].split(" ")
            .collect();
        if split_strings[0] != "" {
            password_list.push(
                Password { 
                range: split_strings[0]
                    .split("-")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
                letter: split_strings[1].trim().chars().collect::<Vec<char>>()[0], 
                password: split_strings[2].trim().to_string() 
                });
        }
    }
    password_list
}

fn parse_file(file_name: &str) -> Vec<String> {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.split("\n")
        .filter_map(|w| Some(w.to_string()))
        .collect::<Vec<String>>()

}

