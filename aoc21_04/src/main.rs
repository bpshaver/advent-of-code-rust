use aoc_utils::input;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq)]
struct BingoBoard {
    elements: [i32; 25],
    marked_idxes: HashSet<usize>,
}

impl BingoBoard {
    fn new(elements: [i32; 25]) -> BingoBoard {
        BingoBoard {
            elements,
            marked_idxes: HashSet::new(),
        }
    }
    fn mark(&mut self, num: i32) {
        for (i, element) in self.elements.iter().enumerate() {
            if num == *element {
                self.marked_idxes.insert(i);
            }
        }
    }

    fn get_all_unmarked_nums(&self) -> Vec<i32> {
        let mut unmarked_nums: Vec<i32> = Vec::new();
        for (i, num) in self.elements.iter().enumerate() {
            if !self.marked_idxes.contains(&i) {
                unmarked_nums.push(*num)
            }
        }
        unmarked_nums
    }

    fn is_winning(&self) -> bool {
        for line in [
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
            vec![0, 5, 10, 15, 20],
            vec![1, 6, 11, 16, 21],
            vec![2, 7, 12, 17, 22],
            vec![3, 8, 13, 18, 23],
            vec![4, 9, 14, 19, 24],
        ]
        .iter()
        {
            let winning = line.iter().rfold(true, |b, elem| {
                if b == false {
                    false
                } else if self.marked_idxes.contains(&elem) {
                    true
                } else {
                    false
                }
            });
            if winning == true {
                return true;
            }
        }
        false
    }
}

fn parse_bingo_input(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let mut sections = input.split("\n\n");
    let mut boards: Vec<BingoBoard> = vec![];
    let nums = Some(
        sections
            .next()
            .expect("There should be at least one line in input!")
            .split(',')
            .map(|s| s.parse().expect("Lines should contain only digits"))
            .collect(),
    );
    for section in sections {
        let mut elements: Vec<i32> = vec![];
        for element in section.split_whitespace() {
            elements.push(
                element
                    .parse()
                    .expect("Each bingo section should contain only whitespace!"),
            )
        }
        boards.push(BingoBoard::new(
            elements
                .try_into()
                .expect("Expected 25 elements for BingoBoard!"),
        ))
    }

    (
        nums.expect("There should be at least one line in input!"),
        boards,
    )
}

fn part_one(input: &str) -> u32 {
    let (nums, mut boards) = parse_bingo_input(input);
    for num in nums {
        for board in boards.iter_mut() {
            board.mark(num as i32);
            if board.is_winning() {
                return board
                    .get_all_unmarked_nums()
                    .iter()
                    .fold(0, |c, &e| c + e as u32)
                    * num;
            }
        }
    }
    0
}

fn part_two(input: &str) -> u32 {
    let mut res: Option<u32> = None;
    let (nums, mut boards) = parse_bingo_input(input);
    for num in nums {
        for board in boards.iter_mut() {
            if !board.is_winning() {
                board.mark(num as i32);
                if board.is_winning() {
                    res = Some(
                        board
                            .get_all_unmarked_nums()
                            .iter()
                            .fold(0, |c, &e| c + e as u32)
                            * num,
                    );
                };
            }
        }
        if boards.iter().all(|b| b.is_winning()) {
            return res.expect("There should be at least one winning board!");
        }
    }
    res.expect("There should be at least one winning board!")
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
    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";

    #[test]
    fn bingo_board_basic() {
        let mut bb = BingoBoard::new([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        bb.mark(1);
        assert_eq!(bb.marked_idxes, {
            let mut t: HashSet<usize> = HashSet::new();
            t.insert(0);
            t
        });
        assert!(!bb.is_winning());
        for i in 2..=5 {
            bb.mark(i)
        }
        assert!(bb.is_winning());
    }
    #[test]
    fn parse_bingo_input_sample() {
        assert_eq!(
            parse_bingo_input(SAMPLE).0,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(parse_bingo_input(SAMPLE).1.len(), 3)
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 4512)
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 1924)
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 5685)
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 21070)
    }
}
