use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Token;
use crate::token_stream::TokenStream;
use crate::types::option_field::OptionField;
use crate::types::oneof::Oneof;
use crate::types::map::Map;
use crate::types::field::Field;
use crate::types::enumerate::Enum;

#[derive(Clone, Debug, PartialEq)]
pub enum MessageMember {
    Enum(Enum),
    Field(Field),
    Map(Map),
    Message(Message),
    Oneof(Oneof),
    Option(OptionField),
}

impl From<Enum> for MessageMember {
    fn from(value: Enum) -> Self {
        MessageMember::Enum(value)
    }
}

impl From<Field> for MessageMember {
    fn from(value: Field) -> Self {
        MessageMember::Field(value)
    }
}

impl From<Map> for MessageMember {
    fn from(value: Map) -> Self {
        MessageMember::Map(value)
    }
}

impl From<Message> for MessageMember {
    fn from(value: Message) -> Self {
        MessageMember::Message(value)
    }
}

impl From<Oneof> for MessageMember {
    fn from(value: Oneof) -> Self {
        MessageMember::Oneof(value)
    }
}

impl From<OptionField> for MessageMember {
    fn from(value: OptionField) -> Self {
        MessageMember::Option(value)
    }
}

impl Display for MessageMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


#[derive(Clone, Debug, Default, PartialEq)]
pub struct Message {
    pub name: String,
    pub members: Vec<MessageMember>,
}

impl Message {
    pub fn new(name: String) -> Self {
        Self { name, ..Default::default() }
    }

    pub fn push(&mut self, value: MessageMember) {
        self.members.push(value)
    }
}

impl TryFrom<TokenStream> for Message {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("message({:?})", tokens);

        tokens.next_is(Token::Message, "message identifier")?;
        let name = tokens.next_is_ident("message name")?;

        let mut res = Message::new(name);

        while !tokens.is_empty() {
            let member = match tokens.last() {
                Some(Token::Enum) => {
                    let block = tokens.block(Token::LBrace, Token::RBrace);
                    MessageMember::from(Enum::try_from(block)?)
                }
                Some(Token::Map) => {
                    let line = tokens.line(Token::Semicolon);
                    MessageMember::from(Enum::try_from(line)?)
                }
                Some(Token::Message) => {
                    let block = tokens.block(Token::LBrace, Token::RBrace);
                    MessageMember::from(Message::try_from(block)?)
                }
                Some(Token::Oneof) => {
                    let block = tokens.block(Token::LBrace, Token::RBrace);
                    MessageMember::from(Oneof::try_from(block)?)
                }
                Some(Token::Option) => {
                    let line = tokens.line(Token::Semicolon);
                    MessageMember::from(OptionField::try_from(line)?)
                }
                Some(_field) => {
                    let line = tokens.line(Token::Semicolon);
                    MessageMember::from(Field::try_from(line)?)
                }
                None => unreachable!(),
            };

            res.push(member);
        }

        return Ok(res);
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);
        indent(f)?;

        writeln!(f, "message {} {{", self.name)?;
        for member in &self.members {
            writeln!(f, "{:indent$}", member, indent = i + 1)?;
        }
        writeln!(f, "}}")
    }
}
