use aoc_utils::input;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

fn chebyshev_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    if dx > dy {
        dx
    } else {
        dy
    }
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let delta_x = head.0 - tail.0;
    let delta_y = head.1 - tail.1;
    let mut move_x = 0;
    let mut move_y = 0;
    if chebyshev_distance(head, tail) > 1 {
        if delta_x > 0 {
            move_x = 1;
        }
        if delta_x < 0 {
            move_x = -1;
        }
        if delta_y > 0 {
            move_y = 1;
        }
        if delta_y < 0 {
            move_y = -1;
        }
    }
    ((move_x, move_y), (tail.0 + move_x, tail.1 + move_y))
}

fn simulate_rope_motion_with_n_knots(input: &str, n: usize) -> usize {
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); n];
    visited_locations.insert((0, 0));
    for line in input.lines() {
        let (dir, mag) = line
            .split_once(' ')
            .expect("Each line should have pattern 'dir mag'");
        let mag: i32 = mag
            .parse()
            .expect("Magnitude should be an unsigned integer");
        match dir {
            "R" => knots[0].0 += mag,
            "L" => knots[0].0 -= mag,
            "U" => knots[0].1 += mag,
            "D" => knots[0].1 -= mag,
            s => panic!("Unexpected direction {}", s),
        };
        for _ in 0..mag {
            for i in 0..(n - 1) {
                let (_, new_tail) = move_tail(knots[i], knots[i + 1]);
                knots[i + 1] = new_tail;
            }
            visited_locations.insert(knots[n - 1]);
        }
    }
    visited_locations.len()
}

fn part_one(input: &str) -> usize {
    simulate_rope_motion_with_n_knots(input, 2)
}

fn part_two(input: &str) -> usize {
    simulate_rope_motion_with_n_knots(input, 10)
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

    const SAMPLE1: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
    const SAMPLE2: &str = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    #[test]
    fn move_tail_basic() {
        // No movement of tail
        assert_eq!(move_tail((0, 0), (0, 0)), ((0, 0), (0, 0)));
        assert_eq!(move_tail((5, 5), (5, 5)), ((0, 0), (5, 5)));
        assert_eq!(move_tail((1, 1), (0, 0)), ((0, 0), (0, 0)));
        assert_eq!(move_tail((6, 6), (5, 5)), ((0, 0), (5, 5)));
        assert_eq!(move_tail((6, 5), (5, 5)), ((0, 0), (5, 5)));
        assert_eq!(move_tail((0, 1), (0, 0)), ((0, 0), (0, 0)));
        assert_eq!(move_tail((1, 0), (0, 0)), ((0, 0), (0, 0)));
        assert_eq!(move_tail((0, -1), (0, 0)), ((0, 0), (0, 0)));
        assert_eq!(move_tail((-1, 0), (0, 0)), ((0, 0), (0, 0)));
        // Head moves vertically
        assert_eq!(move_tail((0, 2), (0, 0)), ((0, 1), (0, 1)));
        assert_eq!(move_tail((0, -2), (0, 0)), ((0, -1), (0, -1)));
        assert_eq!(move_tail((5, 7), (5, 5)), ((0, 1), (5, 6)));
        assert_eq!(move_tail((5, 3), (5, 5)), ((0, -1), (5, 4)));
        // Head moves horizontally
        assert_eq!(move_tail((-3, -5), (-5, -5)), ((1, 0), (-4, -5)));
        assert_eq!(move_tail((-7, -5), (-5, -5)), ((-1, 0), (-6, -5)));
        // Head moves diagonally
        assert_eq!(move_tail((12, 12), (10, 10)), ((1, 1), (11, 11)));
        assert_eq!(move_tail((12, 11), (10, 10)), ((1, 1), (11, 11)));
        assert_eq!(move_tail((8, 9), (10, 10)), ((-1, -1), (9, 9)));
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE1), 13);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 6087);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE2), 36);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 2493);
    }
}
