use aoc_utils::input;

mod parse;

const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> String {
    let (stack_lines, instructions) = input
        .split_once("\n\n")
        .expect("Input has at least one empty line.");
    let mut stacks = parse::parse_stacks(stack_lines);

    for (n, from, to) in parse::parse_instructions(instructions) {
        for _ in 0..n {
            let c = stacks
                .get_mut(from - 1)
                .expect("From is valid stack")
                .pop()
                .expect("From stack should have at least one element");
            stacks.get_mut(to - 1).expect("To is valid stack").push(c);
        }
    }

    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .flatten()
        .collect()
}

fn part_two(input: &str) -> String {
    let (stack_lines, instructions) = input
        .split_once("\n\n")
        .expect("Input has at least one empty line.");
    let mut stacks = parse::parse_stacks(stack_lines);

    for (n, from, to) in parse::parse_instructions(instructions) {
        let from_stack: &mut Vec<char> = stacks.get_mut(from - 1).expect("From is valid stack");
        let from_n = from_stack.len();
        let mut substack: Vec<char> = from_stack.drain(from_n - n..).collect();

        stacks
            .get_mut(to - 1)
            .expect("To is valid stack")
            .append(&mut substack);
    }

    stacks
        .iter_mut()
        .map(|stack| stack.pop())
        .flatten()
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

    const SAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), "CMZ");
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), "TLFGBZHCN")
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), "MCD");
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), "QRQFHFWCL")
    }
}
