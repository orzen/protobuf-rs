use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::comment::{BlockComment, LineComment};
use crate::types::enum_field::EnumField;
use crate::types::option_field::OptionField;

#[derive(Clone, Debug, PartialEq)]
pub enum EnumMember {
    BlockComment(BlockComment),
    Field(EnumField),
    LineComment(LineComment),
    Option(OptionField),
}

impl From<BlockComment> for EnumMember {
    fn from(value: BlockComment) -> Self {
        Self::BlockComment(value)
    }
}

impl From<EnumField> for EnumMember {
    fn from(value: EnumField) -> Self {
        Self::Field(value)
    }
}

impl From<LineComment> for EnumMember {
    fn from(value: LineComment) -> Self {
        Self::LineComment(value)
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

        tokens.next_eq(Type::Enum, "enum identifier")?;
        let name = tokens.ident_as_string("enum name")?;
        tokens.next_eq(Type::LBrace, "enum opening brace('{')")?;

        let mut enm = Enum::new(name);

        // Handle enum body
        while !tokens.is_empty() {
            let peek_token = match tokens.peek() {
                Some(v) => v,
                None => {
                    return Err(ParserError::Syntax(
                        "enum member".to_string(),
                        "nothing".to_string(),
                    ));
                }
            };

            let member = match peek_token.typ() {
                Type::Option => {
                    let line = tokens.select_until(Type::Semicolon);
                    EnumMember::from(OptionField::try_from(line)?)
                }
                Type::Slash => {
                    if tokens.is_line_comment() {
                        let line = tokens.select_line_comment()?;
                        EnumMember::from(LineComment::from(line))
                    } else if tokens.is_block_comment() {
                        let block = tokens.select_block_comment()?;
                        EnumMember::from(BlockComment::from(block))
                    } else {
                        return Err(ParserError::Syntax(
                            "enum comment".to_string(),
                            format!("enum tokens: {tokens}"),
                        ));
                    }
                }
                Type::RBrace => {
                    break;
                }
                _enum_field => {
                    let line = tokens.select_until(Type::Semicolon);
                    EnumMember::from(EnumField::try_from(line)?)
                }
            };

            tokens.next_eq(Type::RBrace, "enum closing brace('}')")?;

            enm.push(member);
        }


        return Ok(enm);
    }
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);
        indent(f)?;

        writeln!(f, "enum {} {{", self.name)?;
        for member in &self.members {
            writeln!(f, "{:indent$}", member, indent = i + 1)?;
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
