use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::token::{Keyword, Token};
use crate::types::enum_field::EnumField;
use crate::types::opt::Opt;

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub fields: Vec<EnumField>,
    pub opts: Vec<Opt>,
}

impl Enum {
    pub fn new(name: String) -> Self {
        let fields: Vec<EnumField> = Vec::new();
        let opts: Vec<Opt> = Vec::new();

        Enum { name, fields, opts }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Enum> {
        let name = tokens.pop_front()?.ident()?;
        let mut enumerate = Enum::new(name);

        if tokens.is_empty() {
            warn!("enum with empty block {}", pos.near());
        }

        while !tokens.is_empty() {
            let mut line = Token::as_line(&mut tokens);

            // Will return if token is empty
            let token = match line.pop_front() {
                Some(t) => t,
                None => return Some(enumerate),
            };

            // First token decides if the line is an option or a field
            match token.keyword() {
                // Option
                Some(Keyword::Opt) => enumerate.push_opt(Opt::from_token(line, pos)?),
                // Invalid
                Some(other) => {
                    warn!(
                        "enum expected option or field, got {:?} {}",
                        other,
                        pos.near()
                    );
                }
                // Field
                None => {
                    // Put back the token used for peeking
                    line.push_front(token);
                    let field = match EnumField::from_token(line, pos) {
                        Some(f) => f,
                        None => return Some(enumerate),
                    };
                    enumerate.push_field(field)
                }
            }
        }

        Some(enumerate)
    }

    pub fn push_field(&mut self, f: EnumField) {
        self.fields.push(f)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o)
    }

    pub fn to_string(&self, n: u8) -> String {
        let s = String::new();


        for opt in self.opts {
            s.push_str(opt.to_string(n).as_str());
        }

        for fld in self.fields {
            s.push_str(fld.to_string(n).as_str());
        }

        s
    }
}
