use aoc_utils::input;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn get_priority_of_line(line: &str) -> u32 {
    let n = line.chars().count() / 2;
    let (first, second): (Vec<_>, Vec<_>) = line.chars().enumerate().partition(|(i, _)| i < &n);
    let first: HashSet<&char> = first.iter().map(|(_, c)| c).collect();
    let second: HashSet<&char> = second.iter().map(|(_, c)| c).collect();
    let shared = first
        .intersection(&second)
        .next()
        .expect("There is one character in the intersection of the first half and the second half");
    get_priority(**shared)
}
fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}
fn part_one(input: &str) -> u32 {
    input.lines().map(get_priority_of_line).sum()
}

fn part_two(input: &str) -> u32 {
    let mut res = 0;
    let mut c = 0;
    let mut counts: HashMap<char, usize> = HashMap::new();
    for line in input.lines() {
        c += 1;
        for c in HashSet::<char>::from_iter(line.chars()) {
            *counts.entry(c).or_insert(0) += 1;
        }
        if c == 3 {
            c = 0;
            for (c, count) in counts {
                if count == 3 {
                    res += get_priority(c);
                }
            }
            counts = HashMap::new();
        }
    }
    res
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

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn get_priority_of_line_basic() {
        assert_eq!(get_priority_of_line("abcdae"), 1);
        assert_eq!(get_priority_of_line("AbcdAe"), 27)
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 157);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 7997);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 70);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 2545);
    }
}
