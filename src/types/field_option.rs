// Options following a field e.g. "int32 foo = 0 [packed=true];"
//                                               ^^^^^^^^^^^^^
use log::debug;
use std::ops::Deref;
use std::fmt::Display;

use crate::error::ParserError;
use crate::token::Token;
use crate::token_stream::TokenStream;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldOption {
    inner: Vec<(String, String)>,
}

impl FieldOption {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, opt: (String, String)) {
        self.inner.push(opt);
    }
}

impl Deref for FieldOption {
    type Target = Vec<(String, String)>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TryFrom<TokenStream> for FieldOption {
    type Error = ParserError;

    // Parsing forwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("field option({:?})", tokens);

        let mut opt = FieldOption::new();

        tokens.next_is(Token::LBrack, "field option opening bracket('[')")?;

        while !tokens.is_empty() {
            let name = tokens.next_is_fullident("field option name")?;
            tokens.next_is(Token::Assign, "field option assignment('=')")?;
            let value = tokens.next_is_constant("field option value")?;

            opt.push((name, value));

            if tokens.peek_is(Token::RBrack) {
                break;
            }

            tokens.next_is(Token::Comma, "field option delimiter(',')")?;
        }

        tokens.next_is(Token::RBrace, "field option closing bracket(']')")?;

        Ok(opt)
    }
}

impl Display for FieldOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: String = self
            .inner
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join(",");

        write!(f, "[{values}]")
    }
}
