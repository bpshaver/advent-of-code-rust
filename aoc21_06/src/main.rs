use aoc_utils::input;

const INPUT: &str = include_str!("../input.txt");

fn spawn_fish(input: &str, num_days: u64) -> u64 {
    let mut fishes = LanternSchool::new();
    for count in input.trim().split(',') {
        let count: u8 = count
            .parse()
            .expect("Input should be comma-separated integers!");
        fishes.enroll(count);
    }
    for _ in 0..num_days {
        fishes.increment()
    }
    fishes.count()
}

struct LanternSchool {
    counts: [u64; 9],
}

impl LanternSchool {
    fn new() -> Self {
        Self { counts: [0; 9] }
    }

    fn increment(&mut self) {
        let old_counts = self.counts.clone();
        let mut new_counts = [0; 9];
        new_counts[8] = old_counts[0];
        new_counts[7] = old_counts[8];
        new_counts[6] = old_counts[7] + old_counts[0];
        new_counts[5] = old_counts[6];
        new_counts[4] = old_counts[5];
        new_counts[3] = old_counts[4];
        new_counts[2] = old_counts[3];
        new_counts[1] = old_counts[2];
        new_counts[0] = old_counts[1];
        self.counts = new_counts;
    }

    fn count(&self) -> u64 {
        self.counts.iter().sum()
    }

    fn enroll(&mut self, fish: u8) {
        if fish > 8 {
            panic!("Fish can't have a count higher than 8!")
        };
        self.counts[fish as usize] += 1
    }
}

fn part_one(input: &str) -> u64 {
    spawn_fish(input, 80)
}

fn part_two(input: &str) -> u64 {
    spawn_fish(input, 256)
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
    fn lanternshool_increment() {
        let mut school = LanternSchool {
            counts: [3, 1, 1, 2, 1, 0, 0, 0, 0],
        };
        school.increment();
        assert_eq!(school.counts, [1, 1, 2, 1, 0, 0, 3, 0, 3])
    }

    #[test]
    fn spawn_fish_sample() {
        assert_eq!(spawn_fish("3,4,3,1,2", 18), 26);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one("3,4,3,1,2"), 5934);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 359999);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two("3,4,3,1,2"), 26984457539);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 1631647919273);
    }
}
