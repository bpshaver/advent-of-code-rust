use itertools::Itertools;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> usize {
    find_marker(input, 4)
}

fn part_two(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, size: usize) -> usize {
    let mut deque = VecDeque::with_capacity(4);
    for (i, c) in input.chars().enumerate() {
        if deque.len() == size {
            deque.pop_front();
        }
        deque.push_back(c);
        if deque.iter().unique().count() == size {
            return i + 1;
        }
    }
    0
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
    fn part_one_sample() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 1953);
    }
    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 2301);
    }
}
