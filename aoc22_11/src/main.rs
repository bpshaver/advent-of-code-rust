use crate::monkey::*;
use num_integer::lcm;

mod monkey;

const INPUT: &str = include_str!("../input.txt");

fn simulate_round(monkeys: &mut Vec<Monkey>, part_one: bool, lcm: i64) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        let mut outgoing: Vec<(usize, i64)> = Vec::new();
        while monkey.items.len() > 0 {
            monkey.num_inspections += 1;
            let mut item = monkey
                .items
                .pop_front()
                .expect("`items` will alwayshave at least one item");
            item = monkey.operation.apply(item);
            if part_one {
                item /= 3
            } else {
                item = item % lcm
            };
            if item % monkey.divisor == 0 {
                outgoing.push((monkey.true_dst, item));
            } else {
                outgoing.push((monkey.false_dst, item));
            }
        }
        for (idx, item) in outgoing {
            monkeys[idx].items.push_back(item);
        }
    }
}

fn part_one(input: &str) -> u32 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|line| get_monkey(line).expect("Each line can be parsed as a `Monkey` instance`"))
        .collect();
    let lcm = monkeys
        .iter()
        .fold(1, |accum, monkey| lcm(accum, monkey.divisor));
    for _ in 0..20 {
        simulate_round(&mut monkeys, true, lcm);
    }
    monkeys.sort_unstable_by_key(|m| m.num_inspections);
    monkeys[monkeys.len() - 1].num_inspections * monkeys[monkeys.len() - 2].num_inspections
}

fn part_two(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|line| get_monkey(line).expect("Each line can be parsed as a `Monkey` instance`"))
        .collect();
    let lcm = monkeys
        .iter()
        .fold(1, |accum, monkey| lcm(accum, monkey.divisor));
    for _ in 0..10000 {
        simulate_round(&mut monkeys, false, lcm);
    }
    monkeys.sort_unstable_by_key(|m| m.num_inspections);
    (monkeys[monkeys.len() - 1].num_inspections as u64)
        * (monkeys[monkeys.len() - 2].num_inspections as u64)
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
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 10605);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 107822);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 2713310158);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 27267163742);
    }
}
