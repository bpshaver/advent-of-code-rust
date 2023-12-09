use aoc_utils::input;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Card {
    char: char,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let selfc = self.char;
        let other = other.char;

        match (selfc.to_digit(10), other.to_digit(10)) {
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(s), Some(o)) => s.cmp(&o),
            (None, None) => {
                if selfc == other {
                    Ordering::Equal
                } else if selfc == 'A' {
                    Ordering::Greater
                } else if other == 'A' {
                    Ordering::Less
                } else if selfc == 'K' {
                    Ordering::Greater
                } else if other == 'K' {
                    Ordering::Less
                } else if selfc == 'Q' {
                    Ordering::Greater
                } else if other == 'Q' {
                    Ordering::Less
                } else if selfc == 'J' {
                    Ordering::Greater
                } else if other == 'J' {
                    Ordering::Less
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

trait ToCard {
    fn to_card(&self) -> Card;
}

impl ToCard for char {
    fn to_card(&self) -> Card {
        Card { char: *self }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType<'a> {
    HighCard(&'a Vec<Card>),
    OnePair(&'a Vec<Card>),
    TwoPair(&'a Vec<Card>),
    ThreeKind(&'a Vec<Card>),
    FullHouse(&'a Vec<Card>),
    FourKind(&'a Vec<Card>),
    FiveKind(&'a Vec<Card>),
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut map = HashMap::new();
        for card in self.cards.iter() {
            *map.entry(card).or_insert_with(|| 0) += 1;
        }
        let mut counts: Vec<u8> = map.values().map(|n| *n).collect();
        counts.sort();
        use HandType::*;
        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HighCard(&self.cards),
            [1, 1, 1, 2] => OnePair(&self.cards),
            [1, 2, 2] => TwoPair(&self.cards),
            [1, 1, 3] => ThreeKind(&self.cards),
            [2, 3] => FullHouse(&self.cards),
            [1, 4] => FourKind(&self.cards),
            [5] => FiveKind(&self.cards),
            _ => {
                dbg!(self);
                dbg!(counts);
                panic!("Unexected hand!")
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_type().cmp(&other.get_type())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or_else(|| ParseHandError)?;
        let cards = cards.chars().map(|char| char.to_card()).collect();
        let bid = bid.parse().or_else(|_| Err(ParseHandError))?;
        return Ok(Hand { cards, bid });
    }
}

#[allow(unused_variables)]
fn part_one(input: &str) -> u32 {
    let mut hands: Vec<Hand> = input::get_lines_of_type(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

fn part_two(input: &str) -> usize {
    #![allow(unused, dead_code)]
    todo!()
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

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_card_orderings() {
        let (a, k, q, j, t, nine, two) = (
            'A'.to_card(),
            'K'.to_card(),
            'Q'.to_card(),
            'J'.to_card(),
            'T'.to_card(),
            '9'.to_card(),
            '2'.to_card(),
        );
        assert!(a > k);
        assert!(a > t);
        assert!(a > nine);
        assert!(nine < k);
        assert!(two < k);
        assert!(two < nine);
        assert!(two < t);
        assert!(a > k);
        assert!(k < a);
        assert!(a > q);
        assert!(k > q);
        assert!(j > t);
    }

    #[test]
    fn test_hand_type_ordering() {
        use HandType::*;
        assert!(
            FiveKind(&vec![
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' }
            ]) > FourKind(&vec![
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'K' }
            ])
        );
    }

    #[test]
    fn test_hand() {
        let hand = Hand {
            cards: vec![
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
                Card { char: 'A' },
            ],
            bid: 300,
        };

        assert_eq!(hand, "AAAAA 300".parse::<Hand>().unwrap());

        assert_eq!(hand.get_type(), HandType::FiveKind(&hand.cards))
    }

    #[test]
    fn test_hand_ordering() {
        let a: Hand = "KTJJT 220".parse().unwrap();
        let b: Hand = "QQQJA 483".parse().unwrap();

        assert!(b > a);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 6440)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_one(SAMPLE), 5905)
    }
}
