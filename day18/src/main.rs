//
// Advent of Code 2020: Day 18
//
use crate::Node::{Val, Op};

fn main() {
    let data = parse_data("input_test.txt");

    let part1_sum = part1(&data);

    println!("{:?}", part1_sum);
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Node {
    Val(i64),
    Op(char),
}

// Part 1
fn part1(expressions: &[Vec<Node>]) -> i64 {
    expressions
        .iter()
        .map(|line| parse(line))
        .sum::<i64>()
}

// stack: 5+(8*3
// 
fn parse(expression: &[Node]) -> i64 {
    let mut stack: Vec<Node> = Vec::new();
    let mut block: Vec<Node> = Vec::new();
    
    for char in expression.into_iter() {
        if *char != Op(')') {
            stack.push(*char);
        } else if *char == Op(')') {
            while stack[stack.len() - 1] != Op('(') {
                block.push(stack.pop().unwrap());
            }
            stack.pop().unwrap();
            let result = simplify(&block);
            stack.push(result);

        }
    }
    // TODO:
    // Add characters to stack until )
    // Reverse add (stack.pop()) to ordered list until (
    // Evaluate ordered list
    // While length > 1; pop left, pop operand, pop right, evaluate, append
    // expression should be mutable
    1
}

fn simplify(input: &[Node]) -> Node {
    let mut expression: Vec<Node> = input.to_vec();
    let mut total = Node::Val(0);

    while expression.len() > 0 {
        let left = expression.pop().unwrap();
        let op = expression.pop().unwrap();
        let right = expression.pop().unwrap();
        total = evaluate(left, right, op);
        expression.push(total);
    }
    total
}

fn evaluate(left: Node, right: Node, op: Node) -> Node {
    // TODO
    match op {
        Op('+') => left + right,
        Op('*') => left * right,
        _ => unreachable!(),
    }
}

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
        assert_eq!(8, evaluate(5, 3, '+'));
        assert_eq!(25, evaluate(5, 5, '*'));
    }

    #[test]
    fn test_simplify() {
        let data = parse_data("input_test.txt");
        let mut iter = data.iter();

        assert_eq!(26, simplify(iter.next().unwrap()));
        assert_eq!(437, simplify(iter.next().unwrap()));
        assert_eq!(12240, simplify(iter.next().unwrap()));
        assert_eq!(13632, simplify(iter.next().unwrap()));
    }
    
    #[test]
    fn test_part1() {
        let mut data = parse_data("input_test.txt");

        assert_eq!(part1(&mut data), 26_335);
    }
}

