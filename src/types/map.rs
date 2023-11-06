use log::debug;

use crate::error::ParserError;
use crate::token::Type;
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

        tokens.next_eq(Type::Semicolon, "map line ending(';')")?;

        // Check for field options
        let mut options = None;
        if tokens.peek_eq(Type::RBrack) {
            let option_tokens = tokens.select_block(Type::RBrack, Type::LBrack);
            options = Some(FieldOption::try_from(option_tokens)?);
        }

        let index = tokens.intlit_as_i32("map index")?;
        tokens.next_eq(Type::Assign, "map assigment('=')")?;
        let name = tokens.ident_as_string("map name")?;
        tokens.next_eq(Type::GT, "map closing('>')")?;
        let value = tokens.fullident_as_string("map value type")?;
        tokens.next_eq(Type::Comma, "map key-value delimiter(',')")?;
        let key = tokens.ident_as_string("map key type")?;
        tokens.next_eq(Type::LT, "map opening('<')")?;
        tokens.next_eq(Type::Map, "map identifier")?;

        let mut map = Self::new(name, key, value, index);
        map.set_options(options);

        return Ok(map);
    }
}
