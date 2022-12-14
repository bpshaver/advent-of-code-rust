use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Source,
    Rock,
    Sand,
    Void,
}

struct Cave {
    tiles: HashMap<(usize, usize), Tile>,
    max_y: usize,
    min_x: usize,
    max_x: usize,
    source: (usize, usize),
}

fn update_min(min: usize, other: &usize) -> usize {
    if *other < min {
        *other
    } else {
        min
    }
}

fn update_max(max: usize, other: &usize) -> usize {
    if *other > max {
        *other
    } else {
        max
    }
}

fn parse_input(input: &str, part_two: bool) -> Cave {
    let mut tiles = HashMap::new();
    let mut max_y = 0;
    let mut min_x = usize::MAX;
    let mut max_x = 0;

    for line in input.lines() {
        let mut split = line.split(" -> ");
        let mut left = split.next().unwrap();
        let mut opt_right = split.next();
        while opt_right.is_some() {
            let right = opt_right.unwrap();
            let (xl, yl) = left.split_once(',').unwrap();
            let (xr, yr) = right.split_once(',').unwrap();
            let mut xl = xl.parse().unwrap();
            let mut yl = yl.parse().unwrap();
            let mut xr = xr.parse().unwrap();
            let mut yr = yr.parse().unwrap();
            max_y = update_max(max_y, &yl);
            max_y = update_max(max_y, &yr);
            min_x = update_min(min_x, &xl);
            min_x = update_min(min_x, &xr);
            max_x = update_max(max_x, &xl);
            max_x = update_max(max_x, &xr);

            if xl > xr {
                (xl, xr) = (xr, xl)
            }
            if yl > yr {
                (yl, yr) = (yr, yl)
            }
            for x in xl..=xr {
                for y in yl..=yr {
                    tiles.insert((x, y), Tile::Rock);
                }
            }

            left = right;
            opt_right = split.next();
        }
    }

    let source = (500, 0);
    tiles.insert((500, 0), Tile::Source);

    if part_two {
        max_y = max_y + 2;
        for x in (min_x - 1)..(max_x + 1) {
            tiles.insert((x, max_y), Tile::Rock);
        }
    } else {
        for x in (min_x - 1)..=(max_x + 1) {
            if !tiles.contains_key(&(x, max_y)) {
                tiles.insert((x, max_y), Tile::Void);
            }
        }
    }

    Cave {
        tiles,
        max_y,
        min_x,
        max_x,
        source,
    }
}

impl Cave {
    fn get_new_loc(&self, new_loc: (usize, usize)) -> Option<(usize, usize)> {
        match self.tiles.get(&new_loc) {
            None => Some(new_loc),
            Some(Tile::Source) => panic!(),
            Some(Tile::Rock) => None,
            Some(Tile::Sand) => None,
            Some(Tile::Void) => Some(new_loc),
        }
    }
    fn down(&self, loc: &(usize, usize)) -> Option<(usize, usize)> {
        self.get_new_loc((loc.0, loc.1 + 1))
    }

    fn down_left(&self, loc: &(usize, usize)) -> Option<(usize, usize)> {
        self.get_new_loc((loc.0 - 1, loc.1 + 1))
    }

    fn down_right(&self, loc: &(usize, usize)) -> Option<(usize, usize)> {
        self.get_new_loc((loc.0 + 1, loc.1 + 1))
    }

    fn extend_left(&mut self, num: usize) {
        for x in (self.min_x - num)..self.min_x {
            self.tiles.insert((x, self.max_y), Tile::Rock);
        }
        self.min_x -= num;
    }
    fn extend_right(&mut self, num: usize) {
        for x in self.max_x..=(self.max_x + num) {
            self.tiles.insert((x, self.max_y), Tile::Rock);
        }
        self.max_x += num;
    }

    fn drop_sand(&mut self, part_two: bool) -> Option<(usize, usize)> {
        let mut sand = self.source;
        loop {
            if part_two {
                if sand.0 - self.min_x <= 2 {
                    self.extend_left(2)
                } else if self.max_x - sand.0 <= 2 {
                    self.extend_right(2)
                }
            }
            match (
                self.down(&sand),
                self.down_left(&sand),
                self.down_right(&sand),
            ) {
                (Some(loc), _, _) => sand = loc,
                (None, Some(loc), _) => sand = loc,
                (None, None, Some(loc)) => sand = loc,
                (None, None, None) => break,
            }
            match self.tiles.get(&sand) {
                Some(Tile::Void) => {
                    if part_two {
                        panic!()
                    } else {
                        return None;
                    }
                }
                Some(Tile::Source) => break,

                _ => (),
            }
        }
        self.tiles.insert(sand, Tile::Sand);
        Some(sand)
    }

    fn as_string(&self) -> String {
        let mut s = String::new();
        for y in 0..=(self.max_y) {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                match self.tiles.get(&(x, y)) {
                    None => s.push('.'),
                    Some(Tile::Source) => s.push('+'),
                    Some(Tile::Sand) => s.push('o'),
                    Some(Tile::Void) => s.push('v'),
                    Some(Tile::Rock) => s.push('#'),
                }
            }
            s.push('\n');
        }
        s
    }
}

fn run(input: &str, part_two: bool) -> usize {
    let mut cave = parse_input(input, part_two);
    let mut c = 0;
    loop {
        match cave.drop_sand(part_two) {
            None => {
                println!("{}", cave.as_string());
                break c;
            }
            Some(_) => c += 1,
        }
        if let Some(Tile::Sand) = cave.tiles.get(&cave.source) {
            break c;
        }
    }
}

fn main() {
    let part_one_solution = run(INPUT, false);
    println!("Solution to part one: {}", part_one_solution);
    let part_two_solution = run(INPUT, true);
    println!("Solution to part two: {}", part_two_solution);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parse_input_sample() {
        let cave = parse_input(SAMPLE, false);
        assert_eq!(cave.max_y, 9);
        assert_eq!(cave.min_x, 494);
        assert_eq!(cave.max_x, 503);
        assert_eq!(*cave.tiles.get(&(493, 9)).unwrap(), Tile::Void);
        assert_eq!(*cave.tiles.get(&(504, 9)).unwrap(), Tile::Void);
    }

    #[test]
    fn part_one_sample() {
        assert_eq!(run(SAMPLE, false), 24);
    }

    #[test]
    fn part_one_solution() {
        assert_eq!(run(INPUT, false), 862);
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(run(SAMPLE, true), 93);
    }

    #[test]
    fn part_two_solution() {
        assert_eq!(run(INPUT, true), 28744);
    }
}
