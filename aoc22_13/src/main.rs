#![allow(unused, dead_code)]
use aoc_utils::input;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum ListIteme {
    Integer(u8),
    List(Vec<ListItem>),
}

fn line_to_substrings(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\[.*\]|\d)"#).unwrap();
    }
    RE.find_iter(line).map(|m| m.as_str()).collect()
}

fn parse_line(line: &str) -> Vec<ListItem> {
    let mut res = vec![ListItem::Raw(line)];
    res
}

#[derive(Debug, PartialEq)]
enum NItem {
    Integer(u8),
    List(Vec<NItem>),
}

const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> usize {
    #![allow(unused, dead_code)]
    todo!()
}

fn part_two(input: &str) -> usize {
    #![allow(unused, dead_code)]
    todo!()
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    // let part_two_solution = part_two(INPUT);
    // println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    #![allow(unused, dead_code)]
    use crate::*;

    #[test]
    fn line_to_substrings_basic() {
        assert_eq!(
            line_to_substrings("1,1,3,1,1"),
            vec!["1", "1", "3", "1", "1"]
        );
        assert_eq!(
            line_to_substrings("1,[2,[3,[4,[5,6,7]]]],8,9"),
            vec!["1", "[2,[3,[4,[5,6,7]]]]", "8", "9"]
        );
    }

    #[test]
    fn parse_line_test() {
        //  assert_eq!(parse_line("1,[2,[3,[4,[5,6,7]]]],8,9"), vec![]);
        assert_eq!(
            parse_line("1,2, 3"),
            vec![
                ListItem::Integer(1),
                ListItem::Integer(2),
                ListItem::Integer(3)
            ]
        );
        assert_eq!(
            parse_line("1,2, 3, [4,5]"),
            vec![
                ListItem::Integer(1),
                ListItem::Integer(2),
                ListItem::Integer(3),
                ListItem::List(vec![ListItem::Integer(4), ListItem::Integer(5)])
            ]
        );
    }

    // #[test]
    fn part_one_sample() {
        todo!()
    }

    // #[test]
    fn part_two_sample() {
        todo!()
    }
}
