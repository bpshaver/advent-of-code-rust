use aoc_utils::input;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let (mut rows, mut columns) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let mut row = Vec::new();
        for (col_idx, c) in line.chars().enumerate() {
            let height: i8 = c.to_digit(10).expect("All characters in input are digits") as i8;
            row.push(height);
            if col_idx >= columns.len() {
                columns.push(Vec::new());
            }
            columns
                .get_mut(col_idx)
                .expect("Column exists because we just created it")
                .push(height);
        }
        rows.push(row)
    }
    (rows, columns)
}

fn get_visible_trees(trees: &Vec<i8>) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, tree) in trees.iter().enumerate() {
        let mut tallest_left = &-1;
        let mut tallest_right = &-1;
        for (j, other) in trees.iter().enumerate() {
            if j < i {
                if other > tallest_left {
                    tallest_left = other
                }
            }
            if j > i {
                if other > tallest_right {
                    tallest_right = other
                }
            }
        }
        if (tree > tallest_left) || (tree > tallest_right) {
            res.push(i);
        }
    }
    res
}

fn get_visible_trees_with_views(trees: &Vec<i8>) -> Vec<(usize, usize, usize)> {
    let mut res = Vec::new();
    for (i, tree) in trees.iter().enumerate() {
        let mut view_left = i;
        let mut view_right = trees.len() - i - 1;
        for (j, other) in trees.iter().enumerate() {
            if j < i {
                if other >= tree {
                    view_left = i - j
                }
            }
            if j > i {
                view_right = j - i;
                if other >= tree {
                    break;
                }
            }
        }
        res.push((i, view_left, view_right));
    }
    res
}

fn part_one(input: &str) -> usize {
    let mut visible = HashSet::new();
    let (rows, columns) = parse_input(input);
    for (row_idx, row) in rows.iter().enumerate() {
        for col_idx in get_visible_trees(row) {
            visible.insert((row_idx, col_idx));
        }
    }
    for (col_idx, col) in columns.iter().enumerate() {
        for row_idx in get_visible_trees(col) {
            visible.insert((row_idx, col_idx));
        }
    }
    visible.len()
}

fn part_two(input: &str) -> usize {
    let mut scores = HashMap::new();
    let (rows, columns) = parse_input(input);
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, view_left, view_right) in get_visible_trees_with_views(row) {
            scores.insert((row_idx, col_idx), view_left * view_right);
        }
    }
    for (col_idx, col) in columns.iter().enumerate() {
        for (row_idx, view_left, view_right) in get_visible_trees_with_views(col) {
            if let Some(score) = scores.get_mut(&(row_idx, col_idx)) {
                *score *= view_left * view_right;
            }
        }
    }
    scores
        .into_values()
        .max()
        .expect("At least one value in scores HashMap")
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

    const SAMPLE: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn get_visible_trees_basic() {
        assert_eq!(get_visible_trees(&vec![3, 0, 3, 7, 3]), vec![0, 3, 4]);
        assert_eq!(get_visible_trees(&vec![2, 5, 5, 1, 2]), vec![0, 1, 2, 4]);
        assert_eq!(get_visible_trees(&vec![6, 5, 3, 3, 2]), vec![0, 1, 3, 4]);
        assert_eq!(get_visible_trees(&vec![3, 3, 5, 4, 9]), vec![0, 2, 4]);
        assert_eq!(get_visible_trees(&vec![3, 5, 3, 9, 0]), vec![0, 1, 3, 4]);
    }

    #[test]
    fn get_visible_trees_with_views_basic() {
        assert_eq!(
            get_visible_trees_with_views(&vec![2, 5, 5, 1, 2]),
            vec![(0, 0, 1), (1, 1, 1), (2, 1, 2), (3, 1, 1), (4, 2, 0)]
        );
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 21);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 1807);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 8);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 480000);
    }
}
