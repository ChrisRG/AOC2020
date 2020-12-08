//
// Advent of Code 2020: Day 7
//
//
#![feature(str_split_once)]

use std::error::Error;
use petgraph::graphmap::*;
use petgraph::visit::*;

static TARGET: &str = "shiny gold";

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut bag_graph = DiGraphMap::<&str, u32>::new();
    for line in input.lines() {
        build_graph(&mut bag_graph, line)
    }

    println!("Total number of bags which can contain `{}`: {}", 
            TARGET, total_containers(&bag_graph));

    println!("Total number of contents of `{}`: {}",
             TARGET, total_contents(&bag_graph));

    Ok(())
}

fn build_graph<'a>(bag_graph: &mut DiGraphMap<&'a str, u32>,
                 line: &'a str) {
    let (color, items) = line.split_once(" bags contain ").unwrap();

    let main_color = bag_graph.add_node(color);

    if !items.starts_with("no") {
        for rule in items.split(", ") {
            let (weight, target_color) = rule.split_once(" ").unwrap();
            let (target_color, _) = target_color.split_once(" bag").unwrap();

            let weight = weight.parse::<u32>().unwrap();

            bag_graph.add_edge(main_color, target_color, weight);

        }
    }
}

// Part 1
fn total_containers(bag_graph: &DiGraphMap<&str, u32>) -> i32 {
    let mut bfs = Bfs::new(&bag_graph, TARGET);
    let mut breadth_count = 0;

    while let Some(_) = bfs.next(&bag_graph) {
        breadth_count += 1;
    }

    breadth_count-1
}

// Part 2 -- Cheated for this next part... based on an example I found:
//  https://github.com/MarkDDR/advent_of_code_2020/blob/master/src/day7.rs
// But was getting tired of trying to figure out the the petgraph API... 
// NB: the build_graph() function sets up for part 1
// only if the edges are built from target_color to main_color 
// and for part 2 only if the edges are built from main_color to target
// Need to refactor, probably not using a directional graph 
fn total_contents(bag_graph: &DiGraphMap<&str, u32>) -> u32 {
    let mut content_count = 0;

    let mut nodes_to_visit: Vec<_> = bag_graph
        .edges(TARGET)
        .map(|e| (e.target(), *e.weight()))
        .collect();

    while !nodes_to_visit.is_empty() {
        let mut next_nodes_to_visit = vec![];

        for (node, mult) in nodes_to_visit {
            next_nodes_to_visit.extend(bag_graph
                .edges(node)
                .map(|e| (e.target(), mult * *e.weight()))
                );

            content_count += mult;
        }
        nodes_to_visit = next_nodes_to_visit;
    }   
    content_count
}

