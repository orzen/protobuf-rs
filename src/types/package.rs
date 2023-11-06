use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Type;
use crate::token_stream::TokenStream;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Package {
    pub value: String,
}

impl Package {
    pub fn new(value: String) -> Self {
        Package { value }
    }
}

impl TryFrom<TokenStream> for Package {
    type Error = ParserError;

    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("package({:?})", &tokens);

        tokens.next_eq(Type::Semicolon, "package line ending(';')")?;
        let value = tokens.fullident_as_string("package value")?;
        tokens.next_eq(Type::Package, "package identifier")?;

        Ok(Self::new(value))
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        writeln!(f, "package {};", self.value)
    }
}
