use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    num: i64,
    pub items: VecDeque<i64>,
    pub operation: Op,
    pub divisor: i64,
    pub true_dst: usize,
    pub false_dst: usize,
    pub num_inspections: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add(i64),
    Sub(i64),
    Mul(i64),
    Div(i64),
    Square,
}

impl Op {
    pub fn apply(&self, old: i64) -> i64 {
        match self {
            Op::Add(num) => old + num,
            Op::Sub(num) => old - num,
            Op::Mul(num) => old * num,
            Op::Div(num) => old / num,
            Op::Square => old * old,
        }
    }
}

pub fn get_monkey(text: &str) -> Option<Monkey> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"Monkey ([0-9]+)\W+Starting items: (.+)\W+Operation: new = old (.) (old|\d+)\W+Test: divisible by (\d+)\W+If true: throw to monkey (\d+)\W+If false: throw to monkey (\d+)"#
        )
        .unwrap();
    }
    let captures = RE.captures(text)?;
    let num = captures
        .get(1)
        .map_or_else(|| return None, |m| Some(m.as_str()))?
        .parse()
        .expect("Monkey number should be an integer");
    let items = captures
        .get(2)
        .map_or_else(|| return None, |m| Some(m.as_str()))?;
    let items = items
        .split(", ")
        .map(|item| item.parse().expect("Each item should be a number"))
        .collect();
    let op = captures
        .get(3)
        .map_or_else(|| return None, |m| Some(m.as_str()))?;
    let op_num = captures
        .get(4)
        .map_or_else(|| return None, |m| Some(m.as_str()))?;
    let operation = {
        if op_num == "old" {
            Op::Square
        } else {
            let op_num = op_num
                .parse()
                .expect("If not 'old,' `op_num` should be an integer");
            match op {
                "+" => Op::Add(op_num),
                "-" => Op::Sub(op_num),
                "*" => Op::Mul(op_num),
                "/" => Op::Div(op_num),
                _ => panic!("Invalid operation {op} with op_num {op_num}"),
            }
        }
    };

    let divisor = captures
        .get(5)
        .map_or_else(|| return None, |m| Some(m.as_str()))?
        .parse()
        .expect("Divisor should be integer!");
    let true_dst = captures
        .get(6)
        .map_or_else(|| return None, |m| Some(m.as_str()))?
        .parse()
        .expect("True monkey dst should be integer!");
    let false_dst = captures
        .get(7)
        .map_or_else(|| return None, |m| Some(m.as_str()))?
        .parse()
        .expect("False monkey dst should be integer!");
    Some(Monkey {
        num,
        items,
        operation,
        divisor,
        true_dst,
        false_dst,
        num_inspections: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn operation_basic() {
        assert_eq!(Op::Add(5).apply(2), 7);
        assert_eq!(Op::Sub(2).apply(5), 3);
        assert_eq!(Op::Mul(3).apply(4), 12);
        assert_eq!(Op::Div(3).apply(9), 3);
    }

    #[test]
    fn get_monkey_basic() {
        assert_eq!(
            get_monkey(
                "Monkey 0:
  Starting items: 63, 57
  Operation: new = old * 11
  Test: divisible by 7
    If true: throw to monkey 6
    If false: throw to monkey 2"
            ),
            Some(Monkey {
                num: 0,
                items: VecDeque::from(vec![63, 57]),
                operation: Op::Mul(11),
                divisor: 7,
                true_dst: 6,
                false_dst: 2,
                num_inspections: 0
            })
        )
    }

    #[test]
    fn get_monkey_sample() {
        for line in SAMPLE.split("\n\n") {
            dbg!(&line);
            get_monkey(line).unwrap();
        }
    }
}
