use aoc_utils::input;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::ops::Add;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    x: i32,
    y: i32,
    bx: i32,
    by: i32,
}

impl Sensor {
    fn mdist(&self) -> u32 {
        return self.x.abs_diff(self.bx) + self.y.abs_diff(self.by);
    }

    fn outside_points(&self) -> Vec<(i32, i32)> {
        let mut res = Vec::new();
        let mdist = self.mdist() + 1;
        for y in (self.y - mdist as i32)..=(self.y + mdist as i32) {
            let hdist = (mdist - self.y.abs_diff(y)) as i32;
            if hdist == 0 {
                res.push((self.x, y));
            } else {
                res.push((self.x - hdist, y));
                res.push((self.x + hdist, y));
            }
        }
        res
    }

    fn contains(&self, point: &(i32, i32)) -> bool {
        (self.x.abs_diff(point.0) + self.y.abs_diff(point.1)) <= self.mdist()
    }
}

fn parse_line(line: &str) -> Sensor {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#
        )
        .unwrap();
    }
    let captures = RE.captures(line).expect("Line matches regex");

    let x = captures.get(1).unwrap().as_str().parse().unwrap();
    let y = captures.get(2).unwrap().as_str().parse().unwrap();
    let bx = captures.get(3).unwrap().as_str().parse().unwrap();
    let by = captures.get(4).unwrap().as_str().parse().unwrap();

    Sensor { x, y, bx, by }
}

fn find_no_beacon_range(sensor: &Sensor, row: i32) -> Option<(i32, i32)> {
    let hdist = {
        let mdist = sensor.mdist();
        let vdist = sensor.y.abs_diff(row);
        if vdist > mdist {
            return None;
        } else {
            mdist - vdist
        }
    };

    Some((sensor.x - hdist as i32, sensor.x + hdist as i32))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct IRange {
    left: i32,
    right: i32,
}

impl IRange {
    fn overlaps(&self, other: IRange) -> bool {
        let (l, r) = (self.left, self.right);
        let (ol, or) = (other.left, other.right);
        if (ol <= l) & (or >= r) {
            return true;
        } else if (ol >= l) & (ol <= r) {
            return true;
        } else if (or >= l) & (or <= r) {
            return true;
        } else {
            return false;
        }
    }

    fn contains(&self, num: i32) -> bool {
        (num >= self.left) & (num <= self.right)
    }
}

impl Add for IRange {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let (l, r) = (self.left, self.right);
        let (ol, or) = (other.left, other.right);
        if ol > r {
            self
        } else if or < l {
            self
        } else if (ol <= l) & (or >= r) {
            IRange {
                left: ol,
                right: or,
            }
        } else if (ol >= l) & (or <= r) {
            self
        } else if ol >= l {
            IRange { left: l, right: or }
        } else if or <= r {
            IRange { left: ol, right: r }
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
struct IRangeSet {
    ranges: Vec<IRange>,
}

impl IRangeSet {
    fn new() -> IRangeSet {
        IRangeSet { ranges: Vec::new() }
    }

    fn add(&mut self, range: IRange) {
        if self.ranges.len() == 0 {
            self.ranges.push(range);
        } else {
            let (overlapping_ranges, mut ranges): (Vec<IRange>, Vec<IRange>) =
                self.ranges.iter().partition(|r| r.overlaps(range));
            ranges.push(overlapping_ranges.iter().fold(range, |a, e| a + *e));
            self.ranges = ranges;
        }
    }

    fn contains(&self, num: i32) -> bool {
        for range in self.ranges.iter() {
            if range.contains(num) {
                return true;
            }
        }
        false
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for range in self.ranges.iter() {
            len += 1 + range.right - range.left
        }
        len as usize
    }
}

fn part_one(input: &str, row: i32) -> usize {
    let mut rs = IRangeSet::new();
    // Locations where beacons or sensors are:
    let mut locs = HashSet::new();
    for sensor in input.lines().map(|line| parse_line(line)) {
        if sensor.by == row {
            locs.insert(sensor.bx);
        };
        if sensor.y == row {
            locs.insert(sensor.x);
        };
        let range = find_no_beacon_range(&sensor, row);
        match range {
            None => (),
            Some(range) => rs.add(IRange {
                left: range.0,
                right: range.1,
            }),
        }
    }
    rs.len() - locs.iter().filter(|x| rs.contains(**x)).count()
}
fn part_two(input: &str, limit: i32) -> i64 {
    let sensors: Vec<Sensor> = input.lines().map(|line| parse_line(line)).collect();
    let mut outside_points = Vec::new();
    for sensor in &sensors {
        outside_points.extend(sensor.outside_points());
    }
    outside_points.dedup();
    for point in outside_points.iter() {
        if (point.0 >= 0) & (point.0 <= limit) & (point.1 >= 0) & (point.1 <= limit) {
            if !&sensors.iter().any(|sensor| sensor.contains(&point)) {
                return (4000000 * point.0 as i64) + (point.1 as i64);
            }
        }
    }
    unreachable!("There should always be at least one point outside all sensor ranges")
}

fn main() {
    println!("Head of INPUT:\n{:?}", input::head(INPUT));
    let part_one_solution = part_one(INPUT, 2000000);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = part_two(INPUT, 4000000);
    println!("Solution to part two: {:?}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn parse_line_basic() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            Sensor {
                x: 2,
                y: 18,
                bx: -2,
                by: 15
            }
        );
    }

    #[test]
    fn sensor_mdist() {
        let s = Sensor {
            x: 2,
            y: 18,
            bx: -2,
            by: 15,
        };
        assert_eq!(s.mdist(), 7)
    }

    #[test]
    fn find_no_beacon_range_basic() {
        let sensor = Sensor {
            x: 5,
            y: 9,
            bx: 10,
            by: 9,
        };
        assert_eq!(find_no_beacon_range(&sensor, 6).unwrap(), (3, 7));
        assert_eq!(find_no_beacon_range(&sensor, 5).unwrap(), (4, 6));
        assert_eq!(find_no_beacon_range(&sensor, 4).unwrap(), (5, 5));
        assert!(find_no_beacon_range(&sensor, 3).is_none());
    }

    #[test]
    fn irange_set_basic() {
        let mut ranges = IRangeSet::new();
        ranges.add(IRange { left: 5, right: 6 });
        ranges.add(IRange {
            left: 15,
            right: 16,
        });
        ranges.add(IRange { left: 3, right: 5 });
        assert_eq!(
            ranges.ranges,
            vec![
                IRange {
                    left: 15,
                    right: 16
                },
                IRange { left: 3, right: 6 },
            ]
        );
        ranges.add(IRange {
            left: 12,
            right: 17,
        });
        assert_eq!(
            ranges.ranges,
            vec![
                IRange { left: 3, right: 6 },
                IRange {
                    left: 12,
                    right: 17
                }
            ]
        );
        assert!(ranges.contains(17));
        assert!(ranges.contains(14));
        assert!(!ranges.contains(-3));
        assert!(!ranges.contains(7));
        assert!(!ranges.contains(32));
        assert_eq!(ranges.len(), 10);
    }

    #[test]
    fn sensor_find_outside_points_contains() {
        let sensor = Sensor {
            x: 0,
            y: 0,
            bx: 0,
            by: 2,
        };
        let mut op = sensor.outside_points();
        op.sort();
        assert_eq!(
            op,
            vec![
                (-3, 0),
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (0, -3),
                (0, 3),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
                (3, 0)
            ]
        );
        assert!(sensor.contains(&(0, 2)));
        assert!(sensor.contains(&(0, 0)));
        assert!(sensor.contains(&(0, 0)));
        assert!(sensor.contains(&(-1, -1)));
        assert!(!sensor.contains(&(0, 3)));
        assert!(!sensor.contains(&(-1, -3)));
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(part_one(SAMPLE, 10), 26);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(part_one(INPUT, 2000000), 4582667);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(part_two(SAMPLE, 20), 56000011);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(part_two(INPUT, 4000000), 10961118625406);
    }
}
