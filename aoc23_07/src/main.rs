use aoc_utils::input;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Hash)]
struct Card<const JOKERS: bool> {
    char: char,
}

impl PartialOrd for Card<false> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Card<true> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Not yet supported:
// https://github.com/rust-lang/project-const-generics/issues/26
// impl<const JOKERS: bool> PartialOrd for Card<JOKERS> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

fn no_joker_ordering(a: char, b: char) -> Ordering {
    match (a.to_digit(10), b.to_digit(10)) {
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(s), Some(o)) => s.cmp(&o),
        (None, None) => {
            if a == b {
                Ordering::Equal
            } else if a == 'A' {
                Ordering::Greater
            } else if b == 'A' {
                Ordering::Less
            } else if a == 'K' {
                Ordering::Greater
            } else if b == 'K' {
                Ordering::Less
            } else if a == 'Q' {
                Ordering::Greater
            } else if b == 'Q' {
                Ordering::Less
            } else if a == 'J' {
                Ordering::Greater
            } else if b == 'J' {
                Ordering::Less
            } else {
                Ordering::Less
            }
        }
    }
}

impl Ord for Card<false> {
    fn cmp(&self, other: &Self) -> Ordering {
        let selfc = self.char;
        let other = other.char;

        no_joker_ordering(selfc, other)
    }
}

impl Ord for Card<true> {
    fn cmp(&self, other: &Self) -> Ordering {
        let selfc = self.char;
        let other = other.char;

        if selfc == 'J' && other != 'J' {
            return Ordering::Less;
        }
        if other == 'J' && selfc != 'J' {
            return Ordering::Greater;
        }

        no_joker_ordering(selfc, other)
    }
}

trait ToCard<const JOKERS: bool> {
    fn to_card(&self) -> Card<JOKERS>;
}

impl ToCard<false> for char {
    fn to_card(&self) -> Card<false> {
        Card { char: *self }
    }
}
impl ToCard<true> for char {
    fn to_card(&self) -> Card<true> {
        Card { char: *self }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType<'a> {
    HighCard(&'a Vec<Card<false>>),
    OnePair(&'a Vec<Card<false>>),
    TwoPair(&'a Vec<Card<false>>),
    ThreeKind(&'a Vec<Card<false>>),
    FullHouse(&'a Vec<Card<false>>),
    FourKind(&'a Vec<Card<false>>),
    FiveKind(&'a Vec<Card<false>>),
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum JokersHandType<'a> {
    HighCard(&'a Vec<Card<true>>),
    OnePair(&'a Vec<Card<true>>),
    TwoPair(&'a Vec<Card<true>>),
    ThreeKind(&'a Vec<Card<true>>),
    FullHouse(&'a Vec<Card<true>>),
    FourKind(&'a Vec<Card<true>>),
    FiveKind(&'a Vec<Card<true>>),
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<const JOKERS: bool> {
    cards: Vec<Card<JOKERS>>,
    bid: u32,
}

impl Hand<false> {
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
                panic!("Unexected hand!")
            }
        }
    }
}

impl Hand<true> {
    fn get_type(&self) -> JokersHandType {
        let mut map = HashMap::new();
        let mut joker_count: u8 = 0;
        for card in self.cards.iter() {
            if card.char != 'J' {
                *map.entry(card).or_insert_with(|| 0) += 1;
            } else {
                joker_count += 1;
            }
        }
        let mut counts: Vec<u8> = map.values().map(|n| *n).collect();
        counts.sort();
        use JokersHandType::*;
        match (joker_count, counts.as_slice()) {
            (0, [1, 1, 1, 1, 1]) => HighCard(&self.cards),
            (0, [1, 1, 1, 2]) => OnePair(&self.cards),
            (1, [1, 1, 1, 1]) => OnePair(&self.cards),
            (0, [1, 2, 2]) => TwoPair(&self.cards),
            (1, [1, 1, 2]) => ThreeKind(&self.cards),
            (2, [1, 2]) => FourKind(&self.cards),
            (2, [1, 1, 1]) => ThreeKind(&self.cards),
            (0, [1, 1, 3]) => ThreeKind(&self.cards),
            (3, [1, 1]) => FourKind(&self.cards),
            (1, [2, 2]) => FullHouse(&self.cards),
            (0, [2, 3]) => FullHouse(&self.cards),
            (1, [1, 3]) => FourKind(&self.cards),
            (0, [1, 4]) => FourKind(&self.cards),
            (0, [5]) => FiveKind(&self.cards),
            (1, [4]) => FiveKind(&self.cards),
            (2, [3]) => FiveKind(&self.cards),
            (3, [2]) => FiveKind(&self.cards),
            (4, [1]) => FiveKind(&self.cards),
            (5, []) => FiveKind(&self.cards),
            _ => {
                dbg!(self);
                dbg!(counts);
                panic!("Unexected hand!")
            }
        }
    }
}

impl PartialOrd for Hand<false> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd for Hand<true> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<true> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_type().cmp(&other.get_type())
    }
}
impl Ord for Hand<false> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_type().cmp(&other.get_type())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand<false> {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or_else(|| ParseHandError)?;
        let cards = cards.chars().map(|char| char.to_card()).collect();
        let bid = bid.parse().or_else(|_| Err(ParseHandError))?;
        return Ok(Hand { cards, bid });
    }
}
impl FromStr for Hand<true> {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or_else(|| ParseHandError)?;
        let cards = cards.chars().map(|char| char.to_card()).collect();
        let bid = bid.parse().or_else(|_| Err(ParseHandError))?;
        return Ok(Hand { cards, bid });
    }
}

fn part_one(input: &str) -> u32 {
    let mut hands: Vec<Hand<false>> = input::get_lines_of_type(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let mut hands: Vec<Hand<true>> = input::get_lines_of_type(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
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
    use crate::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_card_orderings() {
        let (a, k, q, j, t, nine, two) = (
            ToCard::<false>::to_card(&'A'),
            ToCard::<false>::to_card(&'K'),
            ToCard::<false>::to_card(&'Q'),
            ToCard::<false>::to_card(&'J'),
            ToCard::<false>::to_card(&'T'),
            ToCard::<false>::to_card(&'9'),
            ToCard::<false>::to_card(&'2'),
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

        assert_eq!(hand, "AAAAA 300".parse::<Hand<false>>().unwrap());

        assert_eq!(hand.get_type(), HandType::FiveKind(&hand.cards))
    }

    #[test]
    fn test_hand_ordering() {
        let a: Hand<false> = "KTJJT 220".parse().unwrap();
        let b: Hand<false> = "QQQJA 483".parse().unwrap();

        assert!(b > a);
    }

    #[test]
    fn test_hand_ordering_with_jokers() {
        let a: Hand<true> = "KTJJT 220".parse().unwrap();
        let b: Hand<true> = "QQQJA 483".parse().unwrap();

        assert!(b < a);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 6440)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 5905)
    }
}
