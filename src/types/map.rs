use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    name: String,
    key: String,
    value: String,
    position: i32,
    opts: Vec<Opt>,
}

impl Map {
    pub fn new(name: String, key: String, value: String, position: i32, opts: Vec<Opt>) -> Self {
        Map {
            name,
            key,
            value,
            position,
            opts,
        }
    }
}
