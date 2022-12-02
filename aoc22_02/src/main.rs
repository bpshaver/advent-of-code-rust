use aoc_utils::input;
use std::fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
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
    type Err = EnumParseError;
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

impl FromStr for Outcome {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().take(2).count() > 1 {
            return Err(Self::Err::TooManyCharacters);
        }
        match s.chars().next() {
            None => return Err(Self::Err::EmptyString),
            Some('X') => return Ok(Self::Lose),
            Some('Y') => return Ok(Self::Draw),
            Some('Z') => return Ok(Self::Win),
            Some(_) => return Err(Self::Err::InvalidCharacter),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
enum EnumParseError {
    InvalidCharacter,
    TooManyCharacters,
    EmptyString,
}

fn parse_game_line<T>(input: &str) -> Result<(Hand, T), EnumParseError>
where
    T: FromStr<Err = EnumParseError>,
    <T as FromStr>::Err: fmt::Debug,
{
    let tpl = match input.split_once(' ') {
        None => return Err(EnumParseError::EmptyString),
        Some(tpl) => tpl,
    };
    let h1 = Hand::from_str(tpl.0)?;
    match T::from_str(tpl.1) {
        Err(e) => return Err(e),
        Ok(variant) => return Ok((h1, variant)),
    }
}

fn parse_input<T>(input: &str) -> Result<Vec<(Hand, T)>, EnumParseError>
where
    T: FromStr<Err = EnumParseError>,
    <T as FromStr>::Err: fmt::Debug,
{
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

fn rig_game(game: &(Hand, Outcome)) -> (u8, u8) {
    use crate::Hand::*;
    use crate::Outcome::*;
    let mut p1_score = game.0.value();
    let mut p2_score = 0;
    match &game.1 {
        Lose => {
            p1_score += 6;
            match game.0 {
                Rock => p2_score += Scissors.value(),
                Paper => p2_score += Rock.value(),
                Scissors => p2_score += Paper.value(),
            }
        }
        Draw => {
            p1_score += 3;
            p2_score += 3;
            // We play what they play
            p2_score += game.0.value();
        }
        Win => {
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
    use crate::Outcome::*;
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
            parse_input::<Hand>(SAMPLE).unwrap(),
            vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
        );
        assert_eq!(
            parse_input::<Outcome>(SAMPLE).unwrap(),
            vec![(Rock, Draw), (Paper, Lose), (Scissors, Win)]
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
