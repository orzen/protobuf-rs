use super::field::Field;
use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub fields: Vec<Field>,
    pub opts: Vec<Opt>,
}

impl Enum {
    pub fn new(name: String) -> Self {
        let fields: Vec<Field> = Vec::new();
        let opts: Vec<Opt> = Vec::new();

        Enum {name, fields, opts}
    }

    pub fn push_field(&mut self, f: Field) {
        self.fields.push(f)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o)
    }
}
