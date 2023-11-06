use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::option_field::OptionField;
use crate::types::rpc::Rpc;
use crate::types::comment::{BlockComment, LineComment};

#[derive(Clone, Debug, PartialEq)]
pub enum ServiceMember {
    BlockComment(BlockComment),
    LineComment(LineComment),
    Option(OptionField),
    RPC(Rpc),
}

impl From<BlockComment> for ServiceMember {
    fn from(value: BlockComment) -> Self {
        ServiceMember::BlockComment(value)
    }
}

impl From<LineComment> for ServiceMember {
    fn from(value: LineComment) -> Self {
        ServiceMember::LineComment(value)
    }
}

impl From<OptionField> for ServiceMember {
    fn from(value: OptionField) -> Self {
        ServiceMember::Option(value)
    }
}

impl From<Rpc> for ServiceMember {
    fn from(value: Rpc) -> Self {
        ServiceMember::RPC(value)
    }
}

impl Display for ServiceMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    pub name: String,
    pub members: Vec<ServiceMember>,
}

impl Service {
    pub fn new(name: String) -> Self {
        Service {
            name,
            members: vec![],
        }
    }

    pub fn push(&mut self, member: ServiceMember) {
        self.members.push(member);
    }
}

impl TryFrom<TokenStream> for Service {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("service({:?})", &tokens);

        tokens.next_eq(Type::Service, "service identifier")?;
        let name = tokens.fullident_as_string("service name")?;
        tokens.next_eq(Type::LBrace, "serice opening brace('{')")?;

        let mut service = Service::new(name.to_string());

        while !tokens.is_empty() {
            let peek_token = match tokens.peek() {
                Some(v) => v,
                None => {
                    return Err(ParserError::Syntax(
                        "service option or rpc".to_string(),
                        "nothing".to_string(),
                    ))
                }
            };

            let member = match peek_token.typ() {
                Type::Option => {
                    let option_tokens = tokens.select_until(Type::Semicolon);
                    ServiceMember::from(OptionField::try_from(option_tokens)?)
                }
                Type::RPC => {
                    let rpc_tokens = tokens.select_until(Type::Semicolon);
                    ServiceMember::from(Rpc::try_from(rpc_tokens)?)
                }
                Type::Slash => {
                    if tokens.is_line_comment() {
                        let line = tokens.select_line_comment()?;
                        ServiceMember::from(LineComment::from(line))
                    } else if tokens.is_block_comment() {
                        let block = tokens.select_block_comment()?;
                        ServiceMember::from(BlockComment::from(block))
                    } else {
                        return Err(ParserError::Syntax(
                            "service comment".to_string(),
                            format!("service tokens: {tokens}"),
                        ));
                    }
                }
                invalid => {
                    return Err(ParserError::Syntax(
                        "service member".to_string(),
                        format!("{invalid}"),
                    ))
                }
            };

            service.push(member);
        }

        tokens.next_eq(Type::RBrace, "service closing brace('}')")?;

        Ok(service)
    }
}

impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);
        indent(f)?;
        writeln!(f, "service {} {{", self.name)?;
        for member in &self.members {
            writeln!(f, "{:indent$}", member, indent = i + 1)?;
        }
        writeln!(f, "}}")
    }
}
