//
// Advent of Code 2020: Day 18
//
use crate::Node::{Val, Op};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let data = parse_data("input.txt");

    let part1_sum = part1(&data);

    println!("Part 1, final sum: {}", part1_sum);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Node {
    Val(i64),
    Op(char),
}

// Part 1
fn part1(expressions: &[Vec<Node>]) -> i64 {
    let mut total: i64 = 0;
    for exp in expressions {
        let result = parse(exp);
        match result {
            Val(v) => total += v,
            _ => unreachable!(),
        };
    }
    total
}

fn parse(expression: &[Node]) -> Node {
    let mut stack: Vec<Node> = Vec::new();
    let mut block: Vec<Node> = Vec::new();
    
    for char in expression {
        if *char != Op(')') {
            stack.push(*char);
        } else if *char == Op(')') {
            block.clear();
            while stack[stack.len() - 1] != Op('(') {
                block.push(stack.pop().unwrap());
            }
            stack.pop().unwrap();
            let result = simplify(&block);
            stack.push(result);

        }
    }
    stack.reverse();
    simplify(&stack)
}

fn simplify(input: &[Node]) -> Node {
    let mut expression: Vec<Node> = input.to_vec();

    while expression.len() > 1 {
        let left = expression.pop().unwrap();
        let op = expression.pop().unwrap();
        let right = expression.pop().unwrap();
        let result = evaluate(left, right, op);
        expression.push(result);
    }
    expression[0]
}

fn evaluate(num1: Node, num2: Node, op: Node) -> Node {
    match (num1, num2, op) {
        (Val(left), Val(right), Op('+')) => Node::Val(left + right),
        (Val(left), Val(right), Op('*')) => Node::Val(left * right),
        _ => unreachable!(),
    }
}

// Data wrangling
fn to_node(ch: &char) -> Node {
    if ch.is_digit(10) {
        Val(ch.to_digit(10).unwrap() as i64)
    } else {
        Op(*ch)
    }
}

fn parse_data(filename: &str) -> Vec<Vec<Node>> {
    let input = std::fs::read_to_string(filename).unwrap();

    let data = input.lines()
        .map(|line| {
             line
                 .chars()
                 .filter(|&c| c != ' ')
                 .map(|c| to_node(&c))
                 .collect::<Vec<_>>()})
        .collect::<Vec<_>>();

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(Val(8), evaluate(Val(5), Val(3), Op('+')));
        assert_eq!(Val(25), evaluate(Val(5), Val(5), Op('*')));
    }
    
    #[test]
    fn test_part1() {
        let data = parse_data("input_test.txt");

        assert_eq!(part1(&data), 26_335);
    }
}

