use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::token::{Keyword, Token};
use crate::types::{
    enumerate::Enum,
    field::Field,
    map::Map,
    oneof::Oneof,
    opt::Opt,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Message {
    pub name: String,
    pub enums: Vec<Enum>,
    pub fields: Vec<Field>,
    pub maps: Vec<Map>,
    pub messages: Vec<Message>,
    pub oneofs: Vec<Oneof>,
    pub opts: Vec<Opt>,
}

impl Message {
    pub fn new(name: String) -> Self {
        Self { name, ..Default::default() }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let name = tokens.pop_front()?.ident()?;
        let mut msg = Message::new(name);

        while !tokens.is_empty() {
            // Check for keyword token
            let token = tokens.pop_front()?;

            // First token decides if the line is an option or a field
            match token.keyword() {
                Some(Keyword::Enum) => {
                    let block = Token::as_block(&mut tokens);
                    msg.push_enum(Enum::from_token(block, pos)?);
                }
                Some(Keyword::Map) => {
                    let line = Token::as_line(&mut tokens);
                    msg.push_map(Map::from_token(line, pos)?);
                }
                Some(Keyword::Message) => {
                    let block = Token::as_block(&mut tokens);
                    msg.push_message(Message::from_token(block, pos)?);
                }
                Some(Keyword::Oneof) => {
                    let block = Token::as_block(&mut tokens);
                    msg.push_oneof(Oneof::from_token(block, pos)?);
                }
                Some(Keyword::Opt) => {
                    let line = Token::as_line(&mut tokens);
                    msg.push_opt(Opt::from_token(line, pos)?);
                }
                Some(other) => {
                    warn!("expected message entry, got: {:?}", other);
                }
                None => {
                    // Field does not have a keyword therefore the tolken used for this match
                    // is part of the field syntax so we have to put it back into the token
                    // sequence before sending it to the field parser.
                    tokens.push_front(token);
                    let line = Token::as_line(&mut tokens);
                    let field = Field::from_token(line, pos)?;
                    msg.push_field(field)
                }
            }
        }

        Some(msg)
    }

    pub fn push_enum(&mut self, e: Enum) {
        self.enums.push(e)
    }

    pub fn push_field(&mut self, f: Field) {
        self.fields.push(f)
    }

    pub fn push_map(&mut self, m: Map) {
        self.maps.push(m)
    }

    pub fn push_message(&mut self, m: Message) {
        self.messages.push(m)
    }

    pub fn push_oneof(&mut self, o: Oneof) {
        self.oneofs.push(o)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o)
    }
}
