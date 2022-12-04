use aoc_utils::input;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn overlaps(line: &str) -> bool {
    let mut split = line.trim().split(',');
    let mut range_one = split
        .next()
        .expect("Line contains two comma separated values")
        .split('-');
    let mut range_two = split
        .next()
        .expect("Line contains two comma separated values")
        .split('-');
    let one_min: u8 = range_one
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Min is a number");
    let one_max: u8 = range_one
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Max is a number");
    let two_min: u8 = range_two
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Min is a number");
    let two_max: u8 = range_two
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Max is a number");

    ((two_min <= one_min) & (two_max >= one_max)) || ((one_min <= two_min) & (one_max >= two_max))
}

fn overlaps_at_all(line: &str) -> bool {
    let mut split = line.trim().split(',');
    let mut range_one = split
        .next()
        .expect("Line contains two comma separated values")
        .split('-');
    let mut range_two = split
        .next()
        .expect("Line contains two comma separated values")
        .split('-');
    let one_min: u8 = range_one
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Min is a number");
    let one_max: u8 = range_one
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Max is a number");
    let two_min: u8 = range_two
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Min is a number");
    let two_max: u8 = range_two
        .next()
        .expect("Each range is hyphen-separated")
        .parse()
        .expect("Max is a number");

    HashSet::<u8>::from_iter(one_min..=one_max)
        .intersection(&HashSet::from_iter(two_min..=two_max))
        .count()
        > 0
}

fn part_one(input: &str) -> usize {
    input.lines().filter(|line| overlaps(line)).count()
}

fn part_two(input: &str) -> usize {
    input.lines().filter(|line| overlaps_at_all(line)).count()
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

    const SAMPLE: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 2);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 431);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 4);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 823);
    }
}
