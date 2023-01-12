use super::enumerate::Enum;
use super::field::Field;
use super::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub name: String,
    pub enums: Vec<Enum>,
    pub fields: Vec<Field>,
    pub messages: Vec<Message>,
    pub opts: Vec<Opt>,
}

impl Message {
    pub fn new(name: String) -> Self {
        let enums: Vec<Enum> = Vec::new();
        let fields: Vec<Field> = Vec::new();
        let messages: Vec<Message> = Vec::new();
        let opts: Vec<Opt> = Vec::new();

        Message {
            name,
            enums,
            fields,
            messages,
            opts,
        }
    }

    pub fn push_enum(&mut self, e: Enum) {
        self.enums.push(e)
    }

    pub fn push_field(&mut self, f: Field) {
        self.fields.push(f)
    }

    pub fn push_message(&mut self, m: Message) {
        self.messages.push(m)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o)
    }
}
