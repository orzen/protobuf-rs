use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::field::Field;
use crate::types::option_field::OptionField;
use crate::types::comment::{BlockComment, LineComment};

#[derive(Clone, Debug, PartialEq)]
pub enum OneofMember {
    BlockComment(BlockComment),
    Field(Field),
    LineComment(LineComment),
    Option(OptionField),
}

impl From<BlockComment> for OneofMember {
    fn from(value: BlockComment) -> Self {
        Self::BlockComment(value)
    }
}

impl From<Field> for OneofMember {
    fn from(value: Field) -> Self {
        Self::Field(value)
    }
}

impl From<LineComment> for OneofMember {
    fn from(value: LineComment) -> Self {
        Self::LineComment(value)
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

        tokens.next_eq(Type::Oneof, "oneof identifier")?;
        let name = tokens.ident_as_string("oneof name")?;
        tokens.next_eq(Type::LBrace, "oneof opening brace('{')")?;

        let mut res = Self::new(name);

        while !tokens.is_empty() {
            let peek_token = match tokens.peek() {
                Some(v) => v,
                None => return Err(ParserError::Syntax("oneof member".to_string(), "unexpected oneof ending".to_string())),
            };

            let member = match peek_token.typ() {
                Type::Option => {
                    let option_tokens = tokens.select_until(Type::Semicolon);
                    OneofMember::from(OptionField::try_from(option_tokens)?)
                }
                Type::Slash => {
                    if tokens.is_line_comment() {
                        let line = tokens.select_line_comment()?;
                        OneofMember::from(LineComment::from(line))
                    } else if tokens.is_block_comment() {
                        let block = tokens.select_block_comment()?;
                        OneofMember::from(BlockComment::from(block))
                    } else {
                        return Err(ParserError::Syntax(
                            "oneof comment".to_string(),
                            format!("oneof tokens: {tokens}"),
                        ));
                    }
                }
                Type::RBrace => break,
                _field => {
                    let field_tokens = tokens.select_until(Type::Semicolon);
                    OneofMember::from(Field::try_from(field_tokens)?)

                }
            };

            res.push(member);
        }

        tokens.next_eq(Type::RBrace, "oneof closing brace('}')")?;

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
