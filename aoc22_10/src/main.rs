use aoc_utils::input;
use std::iter::Iterator;

const INPUT: &str = include_str!("../input.txt");

enum Instruction {
    Noop,
    Addx(i32),
}

fn get_register_values(input: &str) -> Vec<i32> {
    let instructions = input.trim().lines().map(|line| {
        if line.starts_with("noop") {
            Instruction::Noop
        } else {
            let (_, num) = line
                .split_once(' ')
                .expect("addx lines follow pattern 'addx num'");
            Instruction::Addx(num.parse().expect("num should be a signed integer"))
        }
    });
    let mut register_values: Vec<i32> = vec![1];
    for instruction in instructions {
        let reg = *register_values
            .last()
            .expect("register_values has at least one element");
        register_values.push(reg);
        match instruction {
            Instruction::Noop => (),
            Instruction::Addx(num) => register_values.push(reg + num),
        }
    }
    register_values
}

fn part_one(input: &str) -> i32 {
    let register_values = get_register_values(input);
    (20..=220)
        .step_by(40)
        .fold(0i32, |a, e| a + e as i32 * register_values[e - 1])
}

fn part_two(input: &str) -> String {
    let register_values = get_register_values(input);
    (0..240)
        .map(|i| {
            let rv = register_values[i];
            if (rv - (i % 40) as i32).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect()
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

    const SAMPLE : &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 13140);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 13520);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....");
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), "###...##..###..#..#.###..####..##..###..#..#.#..#.#..#.#..#.#..#.#....#..#.#..#.#..#.#....#..#.####.###..###..#..#.###..###..#.##.###..#..#.#..#.#....####.#..#.#....#..#.#....#..#.#..#.#....#..#.#..#.#.....###.#....#..#.###..####.#..#.###..");
    }
}
