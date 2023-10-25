use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Token;
use crate::token_stream::TokenStream;

use crate::types::field::Field;
use crate::types::option_field::OptionField;

#[derive(Clone, Debug, PartialEq)]
pub enum OneofMember {
    Field(Field),
    Option(OptionField),
}

impl From<Field> for OneofMember {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl From<OptionField> for OneofMember {
    fn from(value: OptionField) -> Self {
        Self::Option(value)
    }
}

impl Display for OneofMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Oneof {
    pub name: String,
    pub members: Vec<OneofMember>,
}

impl Oneof {
    pub fn new(name: String) -> Self {
        Oneof {
            name,
            members: vec![],
        }
    }

    pub fn push(&mut self, member: OneofMember) {
        self.members.push(member);
    }
}

impl TryFrom<TokenStream> for Oneof {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("oneof({:?})", tokens);

        tokens.next_is(Token::Oneof, "oneof identifier")?;
        let name = tokens.next_is_ident("oneof name")?;
        tokens.next_is(Token::LBrace, "oneof opening brace('{')")?;

        let mut res = Self::new(name);

        while !tokens.is_empty() {
            if tokens.peek_is(Token::RBrace) {
                break;
            }

            let member = match tokens.peek_is(Token::Option) {
                true => {
                    let option_tokens = tokens.line(Token::Semicolon);
                    OneofMember::from(OptionField::try_from(option_tokens)?)
                }
                false => {
                    let field_tokens = tokens.line(Token::Semicolon);
                    OneofMember::from(Field::try_from(field_tokens)?)
                }
            };

            res.push(member);
        }

        tokens.next_is(Token::RBrace, "oneof closing brace('}')")?;

        return Ok(res);
    }
}

impl Display for Oneof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);
        indent(f)?;

        writeln!(f, "oneof {} {{", self.name)?;
        for member in &self.members {
            writeln!(f, "{:indent$}", member, indent = i+1)?;
        }
        writeln!(f, "}}")
    }
}
