use std::collections::VecDeque;

use crate::types::{
    field::Field,
    opt::Opt,
};
use crate::token::{Keyword, Token};
use crate::position::Position;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Oneof {
    pub name: String,
    pub fields: Vec<Field>,
    pub opts: Vec<Opt>,
}

impl Oneof {
    pub fn new(name: String) -> Self {
        Oneof { name, fields: vec![], opts: vec![] }
    }

    // The 'oneof' keyword has already been consumed from tokens.
    // This function will handle the name and the block fields.
    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let name = tokens.pop_front()?.ident()?;
        let mut oneof = Self::new(name);

        while !tokens.is_empty() {
            let first = tokens.pop_front()?;
            let mut line = Token::as_line(&mut tokens);

            match first.keyword() {
                Some(Keyword::Opt) => oneof.push_opt(Opt::from_token(line, pos)?),
                _ => {
                    line.push_front(first);
                    oneof.fields.push(Field::from_token(line, pos)?);
                }
            }
        }

        Some(oneof)
    }

    pub fn push_field(&mut self, f: Field) {
        self.fields.push(f);
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o);
    }
}
