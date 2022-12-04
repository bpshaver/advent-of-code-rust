use aoc_utils::input;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

// Leaving unimplemented... we'll just panic
// But the type is required for the FromStr trait
#[derive(Debug)]
enum RangeParseError {}

struct SimpleRange {
    min: u8,
    max: u8,
}

impl SimpleRange {
    fn contains(&self, other: &SimpleRange) -> bool {
        ((other.min >= self.min) & (other.min < self.max))
            || ((other.max <= self.max) & (other.max >= self.min))
    }

    fn fully_contains(&self, other: &SimpleRange) -> bool {
        (other.min >= self.min) & (other.max <= self.max)
    }
}

impl FromStr for SimpleRange {
    type Err = RangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(',');
        let mut range_str = split
            .next()
            .expect("Line contains two comma separated values")
            .split('-');
        let min: u8 = range_str
            .next()
            .expect("Each range is hyphen-separated")
            .parse()
            .expect("Min is a number");
        let max: u8 = range_str
            .next()
            .expect("Each range is hyphen-separated")
            .parse()
            .expect("Max is a number");

        Ok(Self { min, max })
    }
}

fn parse_line(line: &str) -> (SimpleRange, SimpleRange) {
    let mut split = line.split(',');
    let r1 = split
        .next()
        .expect("Line contains two comma-separated values")
        .parse()
        .expect("Line contains two valid ranges");
    let r2 = split
        .next()
        .expect("Line contains two comma-separated values")
        .parse()
        .expect("Line contains two valid ranges");
    (r1, r2)
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = parse_line(line);
            a.fully_contains(&b) || b.fully_contains(&a)
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = parse_line(line);
            a.contains(&b) || b.contains(&a)
        })
        .count()
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
    fn simple_range_basic() {
        let r1 = SimpleRange { min: 0, max: 10 };
        let r2 = SimpleRange { min: 0, max: 7 };
        let r3 = SimpleRange { min: 3, max: 10 };
        let r4 = SimpleRange { min: 3, max: 7 };
        let r5 = SimpleRange { min: 6, max: 12 };
        let r6 = SimpleRange { min: 11, max: 15 };

        assert!(r1.contains(&r2));
        assert!(r1.contains(&r3));
        assert!(r1.contains(&r4));
        assert!(r1.contains(&r5));
        assert!(!r1.contains(&r6));
    }

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
