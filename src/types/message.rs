use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::enumerate::Enum;
use crate::types::field::Field;
use crate::types::map::Map;
use crate::types::oneof::Oneof;
use crate::types::option_field::OptionField;

use super::comment::{BlockComment, LineComment};

#[derive(Clone, Debug, PartialEq)]
pub enum MessageMember {
    Enum(Enum),
    Field(Field),
    Map(Map),
    Message(Message),
    Oneof(Oneof),
    Option(OptionField),
    LineComment(LineComment),
    BlockComment(BlockComment),
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

impl From<BlockComment> for MessageMember {
    fn from(value: BlockComment) -> Self {
        MessageMember::BlockComment(value)
    }
}

impl From<LineComment> for MessageMember {
    fn from(value: LineComment) -> Self {
        MessageMember::LineComment(value)
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
        Self {
            name,
            ..Default::default()
        }
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

        tokens.next_eq(Type::Message, "message identifier")?;
        let name = tokens.ident_as_string("message name")?;

        let mut res = Message::new(name);

        while !tokens.is_empty() {
            let peek_token = match tokens.peek() {
                Some(v) => v,
                None => {
                    return Err(ParserError::Syntax(
                        "message member".to_string(),
                        "nothing".to_string(),
                    ))
                }
            };

            let member = match peek_token.typ() {
                Type::Enum => {
                    let block = tokens.select_block(Type::LBrace, Type::RBrace);
                    MessageMember::from(Enum::try_from(block)?)
                }
                Type::Map => {
                    let line = tokens.select_until(Type::Semicolon);
                    MessageMember::from(Enum::try_from(line)?)
                }
                Type::Message => {
                    let block = tokens.select_block(Type::LBrace, Type::RBrace);
                    MessageMember::from(Message::try_from(block)?)
                }
                Type::Oneof => {
                    let block = tokens.select_block(Type::LBrace, Type::RBrace);
                    MessageMember::from(Oneof::try_from(block)?)
                }
                Type::Option => {
                    let line = tokens.select_until(Type::Semicolon);
                    MessageMember::from(OptionField::try_from(line)?)
                }
                Type::RBrace => {
                    break;
                }
                Type::Slash => {
                    if tokens.is_line_comment() {
                        let line = tokens.select_line_comment()?;
                        MessageMember::from(LineComment::from(line))
                    } else if tokens.is_block_comment() {
                        let block = tokens.select_block_comment()?;
                        MessageMember::from(BlockComment::from(block))
                    } else {
                        return Err(ParserError::Syntax(
                            "message comment".to_string(),
                            format!("message tokens: {tokens}"),
                        ));
                    }
                }
                _field => {
                    let line = tokens.select_until(Type::Semicolon);
                    MessageMember::from(Field::try_from(line)?)
                }
            };

            tokens.next_eq(Type::RBrace, "message closing brace('}')")?;

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

        indent(f)?;
        writeln!(f, "}}")
    }
}
