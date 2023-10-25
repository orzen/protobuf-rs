use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::{indent, level};
use crate::token::Token;
use crate::token_stream::TokenStream;
use crate::types::option_field::OptionField;
use crate::types::rpc::Rpc;

#[derive(Clone, Debug, PartialEq)]
pub enum ServiceMember {
    Option(OptionField),
    RPC(Rpc),
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

        tokens.next_is(Token::Service, "service identifier")?;
        let name = tokens.next_is_fullident("service name")?;
        tokens.next_is(Token::LBrace, "serice opening brace('{')")?;

        let mut service = Service::new(name.to_string());

        while !tokens.is_empty() {
            let member = match tokens.last() {
                Some(Token::Option) => {
                    let option_tokens = tokens.line(Token::Semicolon);
                    ServiceMember::from(OptionField::try_from(option_tokens)?)
                }
                Some(Token::RPC) => {
                    let rpc_tokens = tokens.line(Token::Semicolon);
                    ServiceMember::from(Rpc::try_from(rpc_tokens)?)
                }
                Some(invalid) => {
                    return Err(ParserError::Syntax(
                        "service member".to_string(),
                        format!("{:?}", invalid),
                    ))
                }
                None => {
                    return Err(ParserError::Syntax(
                        "service option or rpc".to_string(),
                        "nothing".to_string(),
                    ))
                }
            };

            service.push(member);
        }

        tokens.next_is(Token::RBrace, "service closing brace('}')")?;

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
