//
// Advent of Code 2020: Day 7
//
#![feature(str_split_once)]

use std::error::Error;
use petgraph::graphmap::*;
use petgraph::visit::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut bag_graph = DiGraphMap::<&str, u32>::new();

    for line in input.lines() {
        graph_bag(&mut bag_graph, line)
    }

    let target_color = "shiny gold"; 
    let mut dfs = Bfs::new(&bag_graph, target_color);
    let mut final_count = 0;

    while let Some(visited) = dfs.next(&bag_graph) {
        final_count += 1;
    }

    println!("Total number of bags which can contain {}: {}", target_color, final_count-1);
    Ok(())
}

fn graph_bag<'a>(bag_graph: &mut DiGraphMap<&'a str, u32>,
                 line: &'a str) {
    let (color, items) = line.split_once(" bags contain ").unwrap();

    let main_color = bag_graph.add_node(color);

    if !items.starts_with("no") {
        for rule in items.split(", ") {
            let (weight, target_color) = rule.split_once(" ").unwrap();
            let (target_color, _) = target_color.split_once(" bag").unwrap();

            let weight = weight.parse::<u32>().unwrap();

            bag_graph.add_edge(target_color, main_color, weight);

        }
    }
}


