use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Token;
use crate::token_stream::TokenStream;
use crate::types::field_option::FieldOption;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnumField {
    pub name: String,
    pub index: i32,
    pub options: Option<FieldOption>,
}

impl EnumField {
    pub fn new(name: String, index: i32) -> Self {
        EnumField {
            name,
            index,
            options: None,
        }
    }

    pub fn set_options(&mut self, options: Option<FieldOption>) {
        self.options = options;
    }
}

impl TryFrom<TokenStream> for EnumField {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("enum field({:?})", tokens);

        tokens.next_is(Token::Semicolon, "enum field line ending(';')")?;

        // Handle field options
        let options = match tokens.peek_is(Token::RBrack) {
            true => {
                let option_tokens = tokens.block(Token::RBrack, Token::LBrace);
                Some(FieldOption::try_from(option_tokens)?)
            }
            false => None,
        };

        // Note index could be a negative integer according to spec.
        // https://protobuf.dev/reference/protobuf/proto3-spec/#enum_definition
        let index = tokens.next_is_intlit("enum field index")?;
        tokens.next_is(Token::Assign, "enum field assignment('=')")?;
        let name = tokens.next_is_ident("enum field name")?;

        let mut res = Self::new(name, index);
        res.set_options(options);

        return Ok(res);
    }
}

impl Display for EnumField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        match &self.options {
            Some(v) => writeln!(f, "{} = {} {v};", self.name, self.index),
            None => writeln!(f, "{} = {};", self.name, self.index),
        }
    }
}
