use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign};

// Point

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point(usize, usize);

impl Point {
    pub fn new(line: usize, character: usize) -> Self {
        Self(line, character)
    }

    pub fn line(&self) -> usize {
        self.0
    }

    pub fn char(&self) -> usize {
        self.1
    }
}

impl Add<usize> for Point {
    type Output = Point;

    fn add(self, rhs: usize) -> Self::Output {
        Point(self.0, self.1 + rhs)
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point(self.0 + 1, rhs)
    }
}

impl AddAssign<usize> for Point {
    fn add_assign(&mut self, rhs: usize) {
        self.1 += rhs;
    }
}

impl MulAssign<usize> for Point {
    fn mul_assign(&mut self, rhs: usize) {
        self.0 += 1;
        self.1 = rhs;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(L{},C{})", self.0, self.1)
    }
}

// Range

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Range {
    begin: Point,
    end: Point,
}

impl Range {
    pub fn new(begin: Point) -> Self {
        Range {
            begin,
            end: begin.clone(),
        }
    }

    pub fn line(&self) -> usize {
        self.end.line()
    }

    pub fn char(&self) -> usize {
        self.end.char()
    }
}

impl From<Point> for Range {
    fn from(begin: Point) -> Self {
        Self::new(begin)
    }
}

impl Add<usize> for Range {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Range {
            begin: self.begin,
            end: self.end + rhs,
        }
    }
}

impl Mul<usize> for Range {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Range {
            begin: self.begin,
            end: self.end * rhs,
        }
    }
}

impl AddAssign<usize> for Range {
    fn add_assign(&mut self, rhs: usize) {
        self.end += rhs;
    }
}

impl MulAssign<usize> for Range {
    fn mul_assign(&mut self, rhs: usize) {
        self.end *= rhs;
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.begin, self.end)
    }
}

// Position

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Point(Point),
    Range(Range),
}

impl Position {
    pub fn range(point: Point) -> Self {
        Self::Range(Range::from(point))
    }

    pub fn line(&self) -> usize {
        match self {
            Self::Point(v) => v.line(),
            Self::Range(v) => v.line(),
        }
    }

    pub fn char(&self) -> usize {
        match self {
            Self::Point(v) => v.char(),
            Self::Range(v) => v.char(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from(Point::default())
    }
}

impl From<Point> for Position {
    fn from(value: Point) -> Self {
        Self::Point(value)
    }
}

impl From<Range> for Position {
    fn from(value: Range) -> Self {
        Self::Range(value)
    }
}

impl Add<usize> for Position {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        match self {
            Self::Point(v) => Self::Point(v + rhs),
            Self::Range(v) => Self::Range(v + rhs),
        }
    }
}

impl Mul<usize> for Position {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        match self {
            Self::Point(v) => Self::Point(v * rhs),
            Self::Range(v) => Self::Range(v * rhs),
        }
    }
}

impl AddAssign<usize> for Position {
    fn add_assign(&mut self, rhs: usize) {
        match self {
            Self::Point(v) => *self = Self::Point(*v + rhs),
            Self::Range(v) => *self = Self::Range(*v + rhs),
        }
    }
}

impl MulAssign<usize> for Position {
    fn mul_assign(&mut self, rhs: usize) {
        match self {
            Self::Point(v) => *self = Self::Point(*v * rhs),
            Self::Range(v) => *self = Self::Range(*v * rhs),
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.line().cmp(&other.line()))
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Point(v) => write!(f, "{}", v),
            Self::Range(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::position::Position;

    use super::{Point, Range};

    #[test]
    fn point() {
        let mut p = Point::default() + 1;

        // Add
        assert_eq!(0, p.line(), "line remains during add");
        assert_eq!(1, p.char(), "char increase during add");

        // Mul
        p = p * 2;
        assert_eq!(1, p.line(), "line increase during mul");
        assert_eq!(2, p.char(), "char set during mul");

        // AddAssign
        p += 1;
        assert_eq!(1, p.line(), "line remains during add assign");
        assert_eq!(3, p.char(), "char increase during add assign");

        // MulAssign
        p *= 2;
        assert_eq!(2, p.line(), "line increase during mul assign");
        assert_eq!(2, p.char(), "char set during mul assign");

        // Display
        assert_eq!("(L2,C2)", format!("{}", p))
    }

    #[test]
    fn range() {
        let mut r = Range::default() + 1;

        // Add
        assert_eq!(0, r.line(), "line remains during add");
        assert_eq!(1, r.char(), "char increase during add");

        // Mul
        r = r * 2;
        assert_eq!(1, r.line(), "line increase during mul");
        assert_eq!(2, r.char(), "char set during mul");

        // AddAssign
        r += 1;
        assert_eq!(1, r.line(), "line remains during add assign");
        assert_eq!(3, r.char(), "char increase during add assign");

        // MulAssign
        r *= 2;
        assert_eq!(2, r.line(), "line increase during mul assign");
        assert_eq!(2, r.char(), "char set during mul assign");

        //Display
        assert_eq!("(L0,C0) -> (L2,C2)", format!("{}", r))
    }

    #[test]
    fn point_position() {
        let mut p = Position::from(Point::default());

        // Add
        p = p + 1;
        assert_eq!(0, p.line(), "line remains during add");
        assert_eq!(1, p.char(), "char increase during add");

        // Mul
        p = p * 2;
        assert_eq!(1, p.line(), "line increase during mul");
        assert_eq!(2, p.char(), "char set during mul");

        // AddAssign
        p += 1;
        assert_eq!(1, p.line(), "line remains during add assign");
        assert_eq!(3, p.char(), "char increase during add assign");

        // MulAssign
        p *= 2;
        assert_eq!(2, p.line(), "line increase during mul assign");
        assert_eq!(2, p.char(), "char set during mul assign");

        //Display
        assert_eq!("(L2,C2)", format!("{}", p))
    }

    #[test]
    fn range_position() {
        let mut p = Position::from(Range::default());

        // Add
        p = p + 1;
        assert_eq!(0, p.line(), "line remains during add");
        assert_eq!(1, p.char(), "char increase during add");

        // Mul
        p = p * 2;
        assert_eq!(1, p.line(), "line increase during mul");
        assert_eq!(2, p.char(), "char set during mul");

        // AddAssign
        p += 1;
        assert_eq!(1, p.line(), "line remains during add assign");
        assert_eq!(3, p.char(), "char increase during add assign");

        // MulAssign
        p *= 2;
        assert_eq!(2, p.line(), "line increase during mul assign");
        assert_eq!(2, p.char(), "char set during mul assign");

        //Display
        assert_eq!("(L0,C0) -> (L2,C2)", format!("{}", p))
    }
}
