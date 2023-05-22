use aoc_utils::input;

use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

pub struct Present(usize, usize, usize);

impl Present {
    pub fn wrapping_paper(&self) -> usize {
        let mut dims = vec![self.0, self.1, self.2];
        dims.sort();

        2 * self.0 * self.1 + 2 * self.0 * self.2 + 2 * self.1 * self.2 + dims[0] * dims[1]
    }

    pub fn ribbon(&self) -> usize {
        let mut dims = vec![self.0, self.1, self.2];
        dims.sort();

        2 * dims[0] + 2 * dims[1] + self.0 * self.1 * self.2
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsePresentError {
    SplitError,
    IntegerParseError,
}

fn parse_to_int(s: &str) -> Result<usize, ParsePresentError> {
    match s.parse() {
        Ok(int) => Ok(int),
        Err(_) => Err(ParsePresentError::IntegerParseError),
    }
}

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lwh: Vec<&str> = s.split('x').collect();
        if lwh.len() != 3 {
            return Err(ParsePresentError::SplitError);
        };
        Ok(Present(
            parse_to_int(lwh[0])?,
            parse_to_int(lwh[1])?,
            parse_to_int(lwh[2])?,
        ))
    }
}

fn part_one(input: &str) -> usize {
    input::get_lines_of_type(input)
        .iter()
        .map(|present: &Present| present.wrapping_paper())
        .sum()
}

fn part_two(input: &str) -> usize {
    input::get_lines_of_type(input)
        .iter()
        .map(|present: &Present| present.ribbon())
        .sum()
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

    #[test]
    fn parse_present_from_str() {
        let p: Present = "4x5x8".parse().unwrap();
        assert_eq!(p.0, 4);
        assert_eq!(p.1, 5);
        assert_eq!(p.2, 8);

        let p: Present = "34x5x21".parse().unwrap();
        assert_eq!(p.0, 34);
        assert_eq!(p.1, 5);
        assert_eq!(p.2, 21);
    }

    #[test]
    fn present_wrapping_paper() {
        assert_eq!(Present(2, 3, 4).wrapping_paper(), 58);
        assert_eq!(Present(2, 4, 3).wrapping_paper(), 58);
        assert_eq!(Present(1, 1, 10).wrapping_paper(), 43);
        assert_eq!(Present(10, 1, 1).wrapping_paper(), 43);
    }

    #[test]
    fn present_ribbon() {
        assert_eq!(Present(2, 3, 4).ribbon(), 34);
        assert_eq!(Present(2, 4, 3).ribbon(), 34);
        assert_eq!(Present(1, 1, 10).ribbon(), 14);
        assert_eq!(Present(10, 1, 1).ribbon(), 14);
    }

    #[test]
    fn part_one_input() {
        assert_eq!(part_one(INPUT), 1586300);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(INPUT), 3737498);
    }
}
