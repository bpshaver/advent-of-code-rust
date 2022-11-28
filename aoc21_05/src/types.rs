use std::cmp::max;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Neither error should not be instantiated, since
// I'm just panicking in the from_str functions
// instead of returning Err.
// I just like the FromStr trait.
#[derive(Debug)]
pub enum PointParseError {}
#[derive(Debug)]
pub enum LineParseError {}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(',');
        let x: i32 = split
            .next()
            .expect("String should have one element befofe one comma!")
            .trim()
            .parse()
            .expect("First element of string should be an integer!");
        let y: i32 = split
            .next()
            .expect("String should have one element after one comma!")
            .trim()
            .parse()
            .expect("Second element of string should be an integer!");
        match split.next() {
            None => (),
            Some(_) => panic!("Unexpected third element in string!"),
        }
        Ok(Self { x, y })
    }
}

pub struct Line {
    pub src: Point,
    pub dst: Point,
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("->");
        let src = Point::from_str(
            split
                .next()
                .expect("Line should have one element before '->'!"),
        )
        .expect("Point should parse from string or panic!");
        let dst = Point::from_str(
            split
                .next()
                .expect("Line should have one element after '->'!"),
        )
        .expect("Point should parse from string or panic!");
        match split.next() {
            None => (),
            Some(_) => panic!("Unexpected third element in string!"),
        }
        Ok(Self { src, dst })
    }
}

impl Line {
    pub fn get_points(self) -> Vec<Point> {
        let mut points = vec![self.src.clone()];
        let mut x_inc = 0;
        if self.dst.x > self.src.x {
            x_inc = 1;
        } else if self.dst.x < self.src.x {
            x_inc = -1
        };
        let mut y_inc = 0;
        if self.dst.y > self.src.y {
            y_inc = 1;
        } else if self.dst.y < self.src.y {
            y_inc = -1
        }
        let max_distance = max(
            self.src.x.abs_diff(self.dst.x),
            self.src.y.abs_diff(self.dst.y),
        );

        let mut x = self.src.x + x_inc;
        let mut y = self.src.y + y_inc;
        for i in 0..max_distance - 1 {
            points.push(Point { x, y });
            x += x_inc;
            y += y_inc;
        }
        points.push(self.dst.clone());
        points
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn point_basic() {
        let a = Point::from_str("5,6").unwrap();
        let b = Point::from_str("5,6").unwrap();
        let c = Point::from_str("5,4").unwrap();
        let d = Point::from_str("4,7").unwrap();

        assert_eq!(a, b);
        assert!(a > c);
        assert!(a > d);
    }

    #[test]
    #[should_panic]
    fn point_from_str_should_panic() {
        let mut _point = Point::from_str("5").unwrap();
        _point = Point::from_str("5,a").unwrap();
        _point = Point::from_str("5,6,7").unwrap();
    }

    #[test]
    fn line_basic() {
        let _line = Line::from_str("5,6 -> 7,8").unwrap();
    }

    #[test]
    #[should_panic]
    fn line_from_str_should_panic() {
        let mut _line = Line::from_str("").unwrap();
        _line = Line::from_str("5,6 ->").unwrap();
        _line = Line::from_str("5,6 -> a,b").unwrap();
        _line = Line::from_str("5,6 -> 7,8 -> foo").unwrap();
    }

    #[test]
    fn line_get_points() {
        let line = Line::from_str("1,2 -> 1,5").unwrap();
        let points = line.get_points();
        assert_eq!(
            points,
            vec![
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
                Point { x: 1, y: 4 },
                Point { x: 1, y: 5 }
            ]
        )
    }
}
