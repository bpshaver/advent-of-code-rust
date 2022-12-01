use aoc_utils::input;

const INPUT: &str = include_str!("../input.txt");

fn part_one(input: &str) -> usize {
    let nums: Vec<u32> = input::get_lines_of_type(input);
    nums.windows(2)
        .filter(|window| window[1] > window[0])
        .count()
}

fn part_two(input: &str) -> usize {
    let nums: Vec<u32> = input::get_lines_of_type(input)
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    nums.windows(2)
        .filter(|window| window[1] > window[0])
        .count()
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

    #[test]
    fn part_one_sample() {
        let sample = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        assert_eq!(part_one(sample), 7);
    }
    #[test]
    fn part_two_sample() {
        let sample = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
        assert_eq!(part_two(sample), 5);
    }
}
