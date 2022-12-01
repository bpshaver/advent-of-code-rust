use aoc_utils::input;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn get_sums_of_substrings<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
    input.trim().split("\n\n").map(|substring| {
        substring
            .split("\n")
            .map(|string| {
                string
                    .parse::<u32>()
                    .expect("Every item in input should be valid integer")
            })
            .sum()
    })
}

fn part_one(input: &str) -> u32 {
    get_sums_of_substrings(input)
        .max()
        .expect("Input should have at least one line of integers")
}

fn part_two(input: &str) -> u32 {
    Itertools::sorted(get_sums_of_substrings(input))
        .rev()
        .take(3)
        .sum::<u32>()
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part two: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 24000)
    }
    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 71506)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 45000)
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 209603)
    }
}
