use std::fmt::Display;
use std::ops::Deref;
use log::debug;

use crate::error::ParserError;
use crate::token::Token;
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

        tokens.next_is(Token::LBrace, "rpc option opening brace('{')")?;

        while !tokens.is_empty() {
            tokens.next_is(Token::Option, "rpc option identifier")?;
            // TODO handle custom option
            let name = tokens.next_is_fullident("rpc option name")?;
            tokens.next_is(Token::Assign, "rpc option assignment('=')")?;
            let value = tokens.next_is_constant("rpc option value")?;

            opt.push((name, value));

            if tokens.peek_is(Token::RBrace) {
                break;
            }

            tokens.next_is(Token::Comma, "rpc option delimiter(';')")?;
        }

        tokens.next_is(Token::RBrace, "rpc option closing brace('{')")?;

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
