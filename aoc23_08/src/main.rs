use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use num::integer::lcm;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn node_parser(input: &str) -> IResult<&str, Node> {
    let (input, (name, (left, right))) = separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alpha1, tag(", "), alpha1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, Node { name, left, right }))
}

#[derive(Debug, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn compute_path_length(
    node: &Node,
    instructions: &str,
    nodes: &HashMap<&str, Node>,
    condition: &str,
) -> Result<u64, PathLengthError> {
    let mut count = 0;
    let mut node = node;
    for instruction in instructions.chars().cycle() {
        if node.name.ends_with(condition) {
            return Ok(count);
        }
        if count == u64::MAX {
            return Err(PathLengthError);
        }
        match instruction {
            'L' => {
                node = nodes.get(node.left).unwrap();
            }
            'R' => {
                node = nodes.get(node.right).unwrap();
            }
            _ => panic!("Unexpected instruction char!"),
        }
        count += 1;
    }
    Ok(count)
}

fn part_one(input: &str) -> u64 {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map = HashMap::new();
    for node in nodes.lines() {
        let (_, node) = node_parser(node).unwrap();
        node_map.insert(node.name, node);
    }
    let node = node_map.get("AAA").unwrap();
    compute_path_length(node, instructions, &node_map, "ZZZ").unwrap()
}

fn part_two(input: &str) -> u64 {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let mut node_map = HashMap::new();
    for node in nodes.lines() {
        let (_, node) = node_parser(node).unwrap();
        node_map.insert(node.name, node);
    }
    let lengths: Vec<u64> = node_map
        .values()
        .into_iter()
        .filter(|node| node.name.ends_with('A'))
        .map(|node| compute_path_length(node, instructions, &node_map, "Z").unwrap())
        .collect();
    lengths.iter().fold(1, |a, e| lcm(a, *e as u64))
}

#[derive(Debug)]
struct PathLengthError;

#[allow(dead_code, unused_mut, unused_variables)]
fn part_two_old(input: &str) -> u32 {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();
    let mut current_nodes: Vec<&str> = nodes
        .lines()
        .map(|line| node_parser(line).unwrap().1.name)
        .filter(|name| name.ends_with('A'))
        .collect();
    let node_map: HashMap<&str, Node> = HashMap::from_iter(
        nodes
            .lines()
            .map(|line| node_parser(line).unwrap().1)
            .map(|node| (node.name, node)),
    );
    let mut count = 0;
    for instruction in instructions.chars().cycle() {
        if current_nodes
            .iter()
            .all(|node_name| node_name.ends_with('Z'))
        {
            break;
        }
        let mut next_nodes: Vec<&str> = Vec::new();
        for node_name in current_nodes.iter() {
            let node = node_map.get(node_name).unwrap();
            match instruction {
                'L' => next_nodes.push(node.left),
                'R' => next_nodes.push(node.right),
                _ => panic!("Unexpected instruction char!"),
            }
        }
        current_nodes = next_nodes;
        count += 1;
    }
    count
}

fn main() {
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = include_str!("../sample.txt");
    const SAMPLE2: &str = include_str!("../sample2.txt");

    #[test]
    fn test_node_parser() {
        let input = "AAA = (BBB, CCC)";
        let (input, output) = node_parser(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(
            output,
            Node {
                name: "AAA",
                left: "BBB",
                right: "CCC"
            }
        )
    }

    #[test]
    fn test_lcm_vec() {
        let vec = vec![1, 3, 4, 2];
        let res = vec.iter().fold(1, |a, e| lcm(a, *e));

        assert_eq!(res, 12);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 6)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE2), 6)
    }
}
