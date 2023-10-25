use log::debug;

use crate::error::ParserError;
use crate::token::Token;
use crate::token_stream::TokenStream;
use crate::types::field_option::FieldOption;

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    name: String,
    key: String,
    value: String,
    index: i32,
    options: Option<FieldOption>,
}

impl Map {
    pub fn new(name: String, key: String, value: String, index: i32) -> Self {
        Map {
            name,
            key,
            value,
            index,
            options: None,
        }
    }

    pub fn set_options(&mut self, options: Option<FieldOption>) {
        self.options = options;
    }
}

impl TryFrom<TokenStream> for Map {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("map({:?})", tokens);

        tokens.next_is(Token::Semicolon, "map line ending(';')")?;

        // Check for field options
        let mut options = None;
        if tokens.peek_is(Token::RBrack) {
            let option_tokens = tokens.block(Token::RBrack, Token::LBrack);
            options = Some(FieldOption::try_from(option_tokens)?);
        }

        let index = tokens.next_is_intlit("map index")?;
        tokens.next_is(Token::Assign, "map assigment('=')")?;
        let name = tokens.next_is_ident("map name")?;
        tokens.next_is(Token::GT, "map closing('>')")?;
        let value = tokens.next_is_fullident("map value type")?;
        tokens.next_is(Token::Comma, "map key-value delimiter(',')")?;
        let key = tokens.next_is_ident("map key type")?;
        tokens.next_is(Token::LT, "map opening('<')")?;
        tokens.next_is(Token::Map, "map identifier")?;

        let mut map = Self::new(name, key, value, index);
        map.set_options(options);

        return Ok(map);
    }
}
