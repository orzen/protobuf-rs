use std::fmt::Display;
use std::ops::Deref;
use log::debug;

use crate::error::ParserError;
use crate::token::Type;
use crate::token_stream::TokenStream;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RpcOption {
    inner: Vec<(String, String)>,
}

impl RpcOption {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, opt: (String, String)) {
        self.inner.push(opt);
    }
}

impl Deref for RpcOption {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<TokenStream> for RpcOption {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("rpc option: {:?}", tokens);

        let mut opt = RpcOption::new();

        tokens.next_eq(Type::LBrace, "rpc option opening brace('{')")?;

        while !tokens.is_empty() {
            tokens.next_eq(Type::Option, "rpc option identifier")?;
            // TODO handle custom option
            let name = tokens.fullident_as_string("rpc option name")?;
            tokens.next_eq(Type::Assign, "rpc option assignment('=')")?;
            let value = tokens.constant_as_string("rpc option value")?;

            opt.push((name, value));

            if tokens.peek_eq(Type::RBrace) {
                break;
            }

            tokens.next_eq(Type::Comma, "rpc option delimiter(';')")?;
        }

        tokens.next_eq(Type::RBrace, "rpc option closing brace('{')")?;

        Ok(opt)
    }
}

impl Display for RpcOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: String = self
            .inner
            .iter()
            .map(|(k, v)| format!("option {k} = {v};"))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{{{values}}}")
    }
}
