use std::{fmt::Display, ops::{Add, AddAssign, MulAssign}};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(line: i32, colum: i32) -> Self {
        Self(line, colum)
    }
}

impl Add<&i32> for Position {
    type Output = Self;

    fn add(self, other: &i32) -> Self {
        Position(self.0, self.1 + other)
    }
}

impl AddAssign<&i32> for Position {
    fn add_assign(&mut self, rhs: &i32) {
        self.1 += rhs;
    }
}

impl AddAssign<i32> for Position {
    fn add_assign(&mut self, rhs: i32) {
        self.1 += rhs;
    }
}

impl MulAssign<i32> for Position {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 += 1;
        self.1 = rhs;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(L{},C{})", self.0, self.1)
    }
}
