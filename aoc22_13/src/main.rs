use serde_json;
use serde_json::Value;
use serde_json::Value::*;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");

fn parse_line(line: &str) -> Value {
    serde_json::from_str(line)
        .expect("Each line in the input represents a valid (nested) JSON array")
}

fn in_order(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Number(l), Number(r)) => {
            let l = l.as_u64().unwrap();
            let r = r.as_u64().unwrap();
            if l < r {
                Some(true)
            } else if l > r {
                Some(false)
            } else {
                None
            }
        }
        (Number(l), Array(_)) => in_order(&Value::from(vec![l.as_u64().unwrap()]), right),
        (Array(_), Number(r)) => in_order(left, &Value::from(vec![r.as_u64().unwrap()])),
        (Array(l), Array(r)) => {
            for i in 0.. {
                let l = l.get(i);
                let r = r.get(i);
                match (l, r) {
                    (None, None) => return None,
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (Some(l), Some(r)) => match in_order(l, r) {
                        None => (),
                        Some(bool_) => return Some(bool_),
                    },
                }
            }
            None
        }
        _ => panic!("Not expecting any JSON `Value`s other than `Number` and `Array`!"),
    }
}

fn part_one(input: &str) -> usize {
    input.split("\n\n").enumerate().fold(0, |a, (i, pair)| {
        let (left, right) = pair
            .split_once('\n')
            .expect("Input sample is two lines followed by a blank line");
        let left = parse_line(left);
        let right = parse_line(right);
        match in_order(&left, &right) {
            None => panic!(),
            Some(true) => a + i + 1,
            Some(false) => a,
        }
    })
}

fn part_two(input: &str) -> usize {
    let mut values: Vec<Value> = input
        .lines()
        .filter(|line| line != &"")
        .map(|line| {
            line.parse()
                .expect("Each line is a valid nested JSON array")
        })
        .collect();
    values.push("[[2]]".parse().unwrap());
    values.push("[[6]]".parse().unwrap());
    values.sort_by(|l, r| match in_order(l, r) {
        None => panic!(),
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    });
    values
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            (*v == &"[[2]]".parse::<Value>().unwrap()) || (*v == &"[[6]]".parse::<Value>().unwrap())
        })
        .map(|(i, _)| i + 1)
        .product()
}

fn main() {
    let part_one_solution = part_one(INPUT);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    #![allow(unused, dead_code)]
    use crate::*;

    const SAMPLE: &str = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const SAMPLE2: &str = "[]\n[[]]\n[[[]]]\n[1,1,3,1,1]\n[1,1,5,1,1]\n[[1],[2,3,4]]\n[1,[2,[3,[4,[5,6,0]]]],8,9]\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[[1],4]\n[[2]]\n[3]\n[[4,4],4,4]\n[[4,4],4,4,4]\n[[6]]\n[7,7,7]\n[7,7,7,7]\n[[8,7,6]]\n[9]";

    fn strings_in_order(l: &str, r: &str) -> bool {
        in_order(&l.parse().unwrap(), &r.parse().unwrap()).unwrap()
    }

    #[test]
    fn in_order_basic() {
        assert!(strings_in_order("7", "9"));
        assert!(strings_in_order("[4]", "[4,5]"));
        assert!(strings_in_order("[4, 5, 6]", "[5]"));
        assert!(strings_in_order("4", "[5]"));
        assert!(strings_in_order("[5,6,0, 1]", "[5,6,7, 1]"));
        assert!(!strings_in_order("[5,6,7,1]", "[5,6,0,1]"));
    }

    #[test]
    fn in_order_complex() {
        assert!(!strings_in_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE), 13);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT), 5292);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE), 140);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT), 23868);
    }
}
