#[derive(Clone, Debug, PartialEq)]
pub struct Syntax {
    pub value: String,
}

impl Syntax {
    pub fn new(value: String) -> Self {
        Syntax { value }
    }
}
