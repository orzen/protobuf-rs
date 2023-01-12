#[derive(Clone, Debug, PartialEq)]
pub struct Opt {
    pub name: String,
    pub value: String,
    pub custom: bool,
}

impl Opt {
    pub fn new(name: String, value: String) -> Self {
        Opt { name, value, custom: false }
    }
}

impl Opt {
    pub fn is_custom(&mut self) {
        self.custom = true;
    }
}
