#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub value: String,
}

impl Package {
    pub fn new(value: String) -> Self {
        Package { value }
    }
}
