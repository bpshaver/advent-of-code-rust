use aoc_utils::input;

const INPUT: &str = include_str!("../input.txt");

fn read_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|s| {
            s.parse()
                .expect("Each comma-separated value should be an integer!")
        })
        .collect()
}

fn series_sum(n: u32) -> u32 {
    (n + 1) * n / 2
}

fn linear_distance(i: u32, nums: &Vec<u32>) -> u32 {
    nums.iter().map(|j| j.abs_diff(i)).sum()
}

fn series_sum_distance(i: u32, nums: &Vec<u32>) -> u32 {
    nums.iter().map(|j| series_sum(j.abs_diff(i))).sum()
}

fn find_min_cost(nums: Vec<u32>, cost_func: impl Fn(u32, &Vec<u32>) -> u32) -> u32 {
    let min = nums
        .iter()
        .min()
        .expect("Nums should not be an empty vector!");
    let max = nums
        .iter()
        .max()
        .expect("Nums should not be an empty vector!");
    let indices: Vec<u32> = (*min..=*max).collect();
    indices
        .iter()
        .map(|i| cost_func(*i, &nums))
        .min()
        .expect("There is at least one number in nums!")
}

fn part_one(input: &str) -> u32 {
    find_min_cost(read_input(input), linear_distance)
}

fn part_two(input: &str) -> u32 {
    find_min_cost(read_input(input), series_sum_distance)
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT);
    println!("Solution to part_one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part_one: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn series_sum_basic() {
        assert_eq!(series_sum(1), 1);
        assert_eq!(series_sum(2), 3);
        assert_eq!(series_sum(3), 6);
        assert_eq!(series_sum(4), 10);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 37);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 349812);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 168);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 99763899);
    }
}
