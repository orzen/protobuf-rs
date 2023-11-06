use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Type;
use crate::token_stream::TokenStream;

// OptionField e.g. `option foo = true;`

#[derive(Clone, Debug, PartialEq)]
pub struct OptionField {
    name: String,
    value: String,
}

impl OptionField {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
        }
    }
}

impl TryFrom<TokenStream> for OptionField {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("option field({:?})", &tokens);

        tokens.next_eq(Type::Semicolon, "option line ending(';')")?;
        let value = tokens.constant_as_string("option value")?;
        tokens.next_eq(Type::Assign, "option assignment('=')")?;

        let name = tokens.optname_as_string("option name")?;

        tokens.next_eq(Type::Option, "option identifier")?;

        let res = Self::new(name, value);

        Ok(res)
    }
}

impl Display for OptionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        writeln!(f, "option {} = {};", self.name, self.value)
    }
}
