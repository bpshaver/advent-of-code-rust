#![allow(unused, dead_code)]

use crate::types::{Line, Point};
use aoc_utils::input;
use std::collections::HashMap;

mod types;

const INPUT: &str = include_str!("../input.txt");

fn count_points_visited_more_than_once(input: &str, ignore_diagonal: bool) -> u32 {
    let mut count_visited_more_than_once: u32 = 0;
    let mut visit_counts: HashMap<Point, u32> = HashMap::new();
    let lines: Vec<Line> = input::get_lines_of_type(input);
    for line in lines {
        // Only consider horizontal or vertical lines
        if !ignore_diagonal | ((line.src.x == line.dst.x) | (line.src.y == line.dst.y)) {
            for point in line.get_points() {
                *visit_counts.entry(point).or_insert(0) += 1;
            }
        }
    }

    for (key, value) in visit_counts.iter() {
        if *value > 1 {
            count_visited_more_than_once += 1
        }
    }
    count_visited_more_than_once
}
fn part_one(input: &str) -> u32 {
    count_points_visited_more_than_once(input, true)
}
fn part_two(input: &str) -> u32 {
    count_points_visited_more_than_once(input, false)
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 5)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 12)
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 5084)
    }
}
