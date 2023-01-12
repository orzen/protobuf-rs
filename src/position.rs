#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(line: i32, colum: i32) -> Self {
        Self(line, colum)
    }

    pub fn near(self) -> String {
        format!("near {}:{}", self.0, self.1)
    }
}
