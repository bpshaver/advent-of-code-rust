use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::cmp::min;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

#[allow(dead_code)] // src and dst not actually read
#[derive(Debug)]
struct Map<'a> {
    src: &'a str,
    dst: &'a str,
    mappings: Vec<(u64, u64, u64)>,
}

impl<'a> Map<'a> {
    fn map(&self, num: u64) -> u64 {
        for mapping in &self.mappings {
            if num >= mapping.1 && num < (mapping.1 + mapping.2) {
                return (mapping.0 + num) - mapping.1;
            }
        }
        num
    }
}

trait ChainMap<'a> {
    fn map(&self, num: u64) -> u64;
}

impl<'a> ChainMap<'a> for Vec<Map<'a>> {
    fn map(&self, num: u64) -> u64 {
        let mut num = num;
        for map in self {
            num = map.map(num);
        }
        num
    }
}

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(char(' '), number))(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, output) = separated_pair(
        terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:")),
        newline,
        separated_list1(
            newline,
            tuple((
                terminated(number, char(' ')),
                terminated(number, char(' ')),
                number,
            )),
        ),
    )(input)?;
    IResult::Ok((
        input,
        Map {
            src: output.0 .0,
            dst: output.0 .1,
            mappings: output.1,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    terminated(
        separated_pair(
            parse_seeds,
            tag("\n\n"),
            separated_list1(tag("\n\n"), parse_map),
        ),
        tag("\n"),
    )(input)
}

pub fn part_one(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    seeds.iter().map(|seed| maps.map(*seed)).min().unwrap()
}

fn part_two(input: &str) -> u64 {
    let mut res = u64::MAX;
    let (_, (seeds, maps)) = parse_input(input).unwrap();
    for pair in seeds.chunks(2) {
        for num in pair[0]..(pair[0] + pair[1]) {
            res = min(res, maps.map(num));
        }
    }
    res
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

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_seeds() {
        let (input, seeds) = parse_seeds("seeds: 1 2 3").unwrap();
        assert_eq!(seeds, vec![1, 2, 3]);
        assert_eq!(input, "");
    }

    #[test]
    fn test_parse_map() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let (input, map) = parse_map(input).unwrap();
        assert_eq!(input, "");
        assert_eq!((map.src, map.dst), ("seed", "soil"));
        assert_eq!(map.mappings, vec![(50, 98, 2), (52, 50, 48)])
    }

    #[test]
    fn test_parse_input() {
        let (input, (seeds, maps)) = parse_input(SAMPLE).unwrap();
        assert_eq!(input, "");
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(maps.last().unwrap().dst, "location");
        assert_eq!(
            maps.last().unwrap().mappings.first().unwrap(),
            &(60, 56, 37)
        )
    }

    #[test]
    fn test_map_associated_functions() {
        let map = Map {
            src: "seed",
            dst: "soil",
            mappings: vec![(50, 98, 2), (52, 50, 48)],
        };
        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(1), 1);
        assert_eq!(map.map(48), 48);
        assert_eq!(map.map(49), 49);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(51), 53);
        assert_eq!(map.map(96), 98);
        assert_eq!(map.map(97), 99);
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 35)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 46)
    }
}
