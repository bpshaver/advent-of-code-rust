use aoc_utils::input;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Debug, PartialEq)]
enum DirectionParseError {
    SplitError,
    ParseVariantError,
    ParseDistanceError,
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once(' ');
        match split {
            Some(tpl) => {
                let distance = tpl.1.parse::<usize>();
                match distance {
                    Ok(distance) => match tpl.0 {
                        "forward" => Ok(Direction::Forward(distance)),
                        "up" => Ok(Direction::Up(distance)),
                        "down" => Ok(Direction::Down(distance)),
                        _ => Err(Self::Err::ParseVariantError),
                    },
                    Err(_) => Err(Self::Err::ParseDistanceError),
                }
            }
            None => Err(Self::Err::SplitError),
        }
    }
}

fn part_one(input: &str) -> usize {
    let directions: Vec<Direction> = input::get_lines_of_type(input);
    let mut hposition: usize = 0;
    let mut depth: usize = 0;

    for direction in directions {
        match direction {
            Direction::Forward(distance) => hposition += distance,
            Direction::Down(distance) => depth += distance,
            Direction::Up(distance) => depth -= distance,
        }
    }
    hposition * depth
}

fn part_two(input: &str) -> usize {
    let directions: Vec<Direction> = input::get_lines_of_type(input);
    let mut hposition: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for direction in directions {
        match direction {
            Direction::Forward(distance) => {
                hposition += distance;
                depth += aim * distance;
            }
            Direction::Down(distance) => aim += distance,
            Direction::Up(distance) => aim -= distance,
        }
    }
    hposition * depth
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part_one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part_one: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn direction_enum() {
        let d = Direction::Forward(5);
        match d {
            Direction::Forward(num) => assert_eq!(num, 5),
            _ => assert!(false),
        }
    }

    #[test]
    fn direction_enum_from_string() {
        assert_eq!(Direction::from_str("forward 5"), Ok(Direction::Forward(5)))
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(
            part_one("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            150
        )
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(
            part_two("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"),
            900
        )
    }
}
