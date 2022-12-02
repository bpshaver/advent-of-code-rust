use aoc_utils::input;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
#[non_exhaustive]
enum HandParseError {
    InvalidCharacter,
    TooManyCharacters,
    EmptyString,
}
impl Hand {
    fn value(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Hand {
    type Err = HandParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().take(2).count() > 1 {
            return Err(Self::Err::TooManyCharacters);
        }
        match s.chars().next() {
            None => return Err(Self::Err::EmptyString),
            Some('X') | Some('A') => return Ok(Self::Rock),
            Some('Y') | Some('B') => return Ok(Self::Paper),
            Some('Z') | Some('C') => return Ok(Self::Scissors),
            Some(_) => return Err(Self::Err::InvalidCharacter),
        };
    }
}

fn parse_game_line(input: &str) -> Result<(Hand, Hand), HandParseError> {
    let tpl = match input.split_once(' ') {
        None => return Err(HandParseError::EmptyString),
        Some(tpl) => tpl,
    };
    let h1 = Hand::from_str(tpl.0)?;
    let h2 = Hand::from_str(tpl.1)?;
    Ok((h1, h2))
}

fn parse_input(input: &str) -> Result<Vec<(Hand, Hand)>, HandParseError> {
    input.lines().map(parse_game_line).collect()
}

fn score_game(game: &(Hand, Hand)) -> (u8, u8) {
    use crate::Hand::*;
    let mut p1_score = game.0.value();
    let mut p2_score = game.1.value();
    if game == &(Rock, Paper) {
        p2_score += 6
    } else if game == &(Paper, Scissors) {
        p2_score += 6
    } else if game == &(Scissors, Rock) {
        p2_score += 6
    } else if game.0 == game.1 {
        p1_score += 3;
        p2_score += 3;
    } else {
        p1_score += 6
    }

    (p1_score, p2_score)
}

fn rig_game(game: &(Hand, Hand)) -> (u8, u8) {
    // Oops, we misunderstood what the second letter in the input was.
    // Ah well, let's use the same enum for part two, but here the
    // second element in the tuple means (lose, draw, win) not (rock,
    // paper, scissors).
    use crate::Hand::*;
    let mut p1_score = game.0.value();
    let mut p2_score = 0;
    match game.1 {
        Rock => {
            // We want to lose
            p1_score += 6;
            match game.0 {
                Rock => p2_score += Scissors.value(),
                Paper => p2_score += Rock.value(),
                Scissors => p2_score += Paper.value(),
            }
        }
        Paper => {
            // We want to draw
            p1_score += 3;
            p2_score += 3;
            // We play what they play
            p2_score += game.0.value();
        }
        Scissors => {
            // We want to win
            p2_score += 6;
            match game.0 {
                Rock => p2_score += Paper.value(),
                Paper => p2_score += Scissors.value(),
                Scissors => p2_score += Rock.value(),
            }
        }
    }
    (p1_score, p2_score)
}

fn part_one(input: &str) -> u32 {
    parse_input(input)
        .expect("Should be at least one line of input")
        .iter()
        .map(|game| score_game(game).1 as u32)
        .sum()
}

fn part_two(input: &str) -> u32 {
    parse_input(input)
        .expect("Should be at least one line of input")
        .iter()
        .map(|game| rig_game(game).1 as u32)
        .sum()
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
    use crate::Hand::*;
    use crate::*;

    const SAMPLE: &str = "A Y\nB X\nC Z";

    #[test]
    fn hand_from_str() {
        assert_eq!(Hand::from_str("X").unwrap(), Rock);
        assert_eq!(Hand::from_str("Y").unwrap(), Paper);
        assert_eq!(Hand::from_str("Z").unwrap(), Scissors);
        assert_eq!(Hand::from_str("A").unwrap(), Rock);
        assert_eq!(Hand::from_str("B").unwrap(), Paper);
        assert_eq!(Hand::from_str("C").unwrap(), Scissors);
    }

    #[test]
    fn parse_input_sample() {
        assert_eq!(
            parse_input(SAMPLE).unwrap(),
            vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        )
    }
    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 15)
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 9241)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 12)
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 14610)
    }
}
