use aoc_utils::maze::{HashMapMaze, NavigableMaze};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Path(Vec<(usize, usize)>);

impl Path {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, loc: (usize, usize)) {
        self.0.push(loc)
    }

    fn last(&self) -> Option<(usize, usize)> {
        if self.len() == 0 {
            None
        } else {
            Some(self.0[self.len() - 1])
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.len().cmp(&other.len())
    }
}

fn parse_input(
    input: &str,
) -> (
    HashMapMaze<(usize, usize), u8>,
    (usize, usize),
    (usize, usize),
) {
    let mut maze = HashMapMaze::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (j, row) in input.lines().rev().enumerate() {
        for (i, c) in row.chars().enumerate() {
            let v = match c {
                'S' => {
                    start = (i, j);
                    0
                }
                'E' => {
                    end = (i, j);
                    25
                }
                _ => (c as u32 - 97) as u8,
            };
            maze.add_loc((i, j), v)
        }
    }
    (maze, start, end)
}

fn part_one(input: &str) -> usize {
    let mut visited_locs = HashSet::new();
    let mut pq: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    let (maze, start, end) = parse_input(input);
    pq.push(Reverse(Path(vec![start])));
    loop {
        let path = pq.pop().expect("Priority queue is not empty").0;
        let loc = path
            .last()
            .expect("All paths in priority queue are non-zero length");
        if visited_locs.contains(&loc) {
            continue;
        }
        visited_locs.insert(loc);
        if loc == end {
            return path.len() - 1;
        }
        let height = maze
            .get_value_at_loc(&loc)
            .expect("Location in path is always valid location in the maze");
        for new_loc_result in [
            maze.loc_above(loc),
            maze.loc_below(loc),
            maze.loc_left(loc),
            maze.loc_right(loc),
        ] {
            match new_loc_result {
                Err(_) => (),
                Ok(new_loc) => {
                    let new_height = maze
                        .get_value_at_loc(&new_loc)
                        .expect("new_loc is valid location");
                    if !path.0.contains(&new_loc) & ((*new_height as i8) - (*height as i8) <= 1) {
                        let mut new_path = path.clone();
                        new_path.push(new_loc);
                        pq.push(Reverse(new_path));
                    }
                }
            };
        }
    }
}

fn part_two(input: &str) -> usize {
    let mut visited_locs = HashSet::new();
    let mut pq: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    let (maze, _, end) = parse_input(input);
    pq.push(Reverse(Path(vec![end])));
    loop {
        let path = pq.pop().expect("Priority queue is not empty").0;
        let loc = path
            .last()
            .expect("All paths in priority queue are non-zero length");
        if visited_locs.contains(&loc) {
            continue;
        }
        visited_locs.insert(loc);
        let height = maze
            .get_value_at_loc(&loc)
            .expect("Location in path is always valid location in the maze");
        if height == &0 {
            return path.len() - 1;
        }
        for new_loc_result in [
            maze.loc_above(loc),
            maze.loc_below(loc),
            maze.loc_left(loc),
            maze.loc_right(loc),
        ] {
            match new_loc_result {
                Err(_) => (),
                Ok(new_loc) => {
                    let new_height = maze
                        .get_value_at_loc(&new_loc)
                        .expect("new_loc is valid location");
                    if !path.0.contains(&new_loc) & ((*height as i8) - (*new_height as i8) <= 1) {
                        let mut new_path = path.clone();
                        new_path.push(new_loc);
                        pq.push(Reverse(new_path));
                    }
                }
            };
        }
    }
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

    const SAMPLE: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn parse_input_sample() {
        let (maze, start, end) = parse_input(SAMPLE);
        assert_eq!(maze.get_value_at_loc(&(0, 4)).unwrap(), &0);
        assert_eq!(start, (0, 4));
        assert_eq!(end, (5, 2));
    }

    #[test]
    fn path_ordering_in_binary_heap() {
        let mut pq = BinaryHeap::new();
        let p1 = Path(vec![(0, 0), (0, 1)]);
        let p2 = Path(vec![(0, 0), (0, 1), (1, 1)]);
        pq.push(Reverse(p1));
        pq.push(Reverse(p2));
        assert_eq!(pq.peek().unwrap().0.len(), 2);
        assert_eq!(pq.pop().unwrap().0, Path(vec![(0, 0), (0, 1)]));
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 31);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 468);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 29);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 459);
    }
}
