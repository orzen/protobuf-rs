use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Token;
use crate::token_stream::TokenStream;

// OptionField e.g. `option foo = true;`

#[derive(Clone, Debug, PartialEq)]
pub struct OptionField {
    name: String,
    value: String,
    custom: bool,
}

impl OptionField {
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            custom: false,
        }
    }

    pub fn set_custom(&mut self, value: bool) {
        self.custom = value;
    }
}

impl TryFrom<TokenStream> for OptionField {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("option field({:?})", &tokens);

        tokens.next_is(Token::Semicolon, "option line ending(';')")?;
        let value = tokens.next_is_constant("option value")?;
        tokens.next_is(Token::Assign, "option assignment('=')")?;

        // Determine if it's a custom option which means that the name is declared within
        // parentheses. We peek to check if the next token is the closing parenthesis since we are
        // popping the vector backwards.
        let mut is_custom = false;
        if tokens.last() == Some(&Token::RParen) {
            is_custom = true;
        }

        let name = match is_custom {
            true => {
                // Check left parenthasis
                tokens.next_is(Token::RParen, "option closing parenthesis('(')")?;
                let n = tokens.next_is_ident("option name")?;
                tokens.next_is(Token::LParen, "option opening parenthesis(')')")?;
                n
            }
            false => tokens.next_is_ident("option name")?,
        };

        tokens.next_is(Token::Option, "option identifier")?;

        let mut res = Self::new(name, value);
        res.set_custom(is_custom);

        Ok(res)
    }
}

impl Display for OptionField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        writeln!(f, "option {} = {};", self.name, self.value)
    }
}
