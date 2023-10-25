use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Token;
use crate::token_stream::TokenStream;
use crate::types::enum_field::EnumField;
use crate::types::option_field::OptionField;

#[derive(Clone, Debug, PartialEq)]
pub enum EnumMember {
    Field(EnumField),
    Option(OptionField),
}

impl From<EnumField> for EnumMember {
    fn from(value: EnumField) -> Self {
        Self::Field(value)
    }
}

impl From<OptionField> for EnumMember {
    fn from(value: OptionField) -> Self {
        Self::Option(value)
    }
}

impl Display for EnumMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub name: String,
    pub members: Vec<EnumMember>,
}

impl Enum {
    pub fn new(name: String) -> Self {
        Enum {
            name,
            members: vec![],
        }
    }

    pub fn push(&mut self, member: EnumMember) {
        self.members.push(member);
    }
}

impl TryFrom<TokenStream> for Enum {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("enum({:?})", tokens);

        tokens.next_is(Token::Enum, "enum identifier")?;
        let name = tokens.next_is_ident("enum name")?;
        tokens.next_is(Token::LBrace, "enum opening brace('{')")?;

        let mut enm = Enum::new(name);

        // Handle enum body
        while !tokens.is_empty() {
            let member = match tokens.peek_is(Token::Option) {
                true => {
                    let line = tokens.line(Token::Semicolon);
                    EnumMember::from(OptionField::try_from(line)?)
                }
                false => {
                    let line = tokens.line(Token::Semicolon);
                    EnumMember::from(EnumField::try_from(line)?)
                }
            };

            enm.push(member);
        }

        tokens.next_is(Token::RBrace, "enum closing brace('}')")?;

        return Ok(enm);
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);
        indent(f)?;

        writeln!(f, "enum {} {{", self.name)?;
        for member in &self.members {
            writeln!(f, "{:indent$}", member, indent = i+1)?;
        }
        writeln!(f, "}}")
    }
}

//#[cfg(test)]
//mod tests {
//    use crate::load_file;
//
//    #[test]
//    fn to_string() {
//        let p = load_file("example.proto");
//        assert_eq!(p, None, "empty proto");
//    }
//}
