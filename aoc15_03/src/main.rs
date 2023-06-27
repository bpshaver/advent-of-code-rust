use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn increment_hashmap_counter<T>(map: &mut HashMap<T, isize>, key: T)
where
    T: std::cmp::Eq + std::cmp::PartialEq + std::hash::Hash,
{
    *map.entry(key).or_insert(0) += 1;
}

fn part_one(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), isize> = HashMap::new();
    let mut loc: (isize, isize) = (0, 0);
    increment_hashmap_counter(&mut map, loc);
    for char in input.chars() {
        match char {
            '^' => loc.1 += 1,
            'v' => loc.1 -= 1,
            '<' => loc.0 -= 1,
            '>' => loc.0 += 1,
            _ => panic!("Unexpected character in input!"),
        };
        increment_hashmap_counter(&mut map, loc);
    }
    map.len()
}

fn part_two(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), isize> = HashMap::new();
    let mut santa_loc: (isize, isize) = (0, 0);
    let mut robos_loc: (isize, isize) = (0, 0);
    increment_hashmap_counter(&mut map, santa_loc);
    increment_hashmap_counter(&mut map, robos_loc);
    for (i, char) in input.chars().enumerate() {
        let loc = {
            if i % 2 == 0 {
                &mut santa_loc
            } else {
                &mut robos_loc
            }
        };
        match char {
            '^' => loc.1 += 1,
            'v' => loc.1 -= 1,
            '<' => loc.0 -= 1,
            '>' => loc.0 += 1,
            _ => panic!("Unexpected character in input!"),
        };
        increment_hashmap_counter(&mut map, *loc);
    }
    map.len()
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
    fn input_is_correct_length() {
        assert_eq!(INPUT.len(), 8192)
    }

    #[test]
    fn increment_hashmap_counter_basic() {
        let mut map = HashMap::new();
        map.insert((0, 0), 0);
        increment_hashmap_counter(&mut map, (0, 0));
        increment_hashmap_counter(&mut map, (1, 1));
        increment_hashmap_counter(&mut map, (1, 1));
        assert_eq!(*map.get(&(0, 0)).unwrap(), 1);
        assert_eq!(*map.get(&(1, 1)).unwrap(), 2)
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(">"), 2);
        assert_eq!(part_one("^>v<"), 4);
        assert_eq!(part_one("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part_one_actual() {
        assert_eq!(part_one(INPUT), 2081);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two("^v"), 3);
        assert_eq!(part_two("^>v<"), 3);
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }

    #[test]
    fn part_two_actual() {
        assert_eq!(part_two(INPUT), 2341);
    }
}
