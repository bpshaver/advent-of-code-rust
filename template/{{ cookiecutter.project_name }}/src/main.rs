use aoc_utils::input;

const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> usize {
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
    use crate::*;

    #[test]
    fn part_one_sample() {
        todo!()
    }

    #[test]
    fn part_two_sample() {
        todo!()
    }
}
