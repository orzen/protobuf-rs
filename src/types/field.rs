use std::fmt::Display;

use log::debug;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::field_option::FieldOption;

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub typ: String,
    pub index: i32,
    pub options: Option<FieldOption>,
    pub repeated: bool,
}

impl Field {
    pub fn new(name: String, typ: String, index: i32, repeated: bool) -> Self {
        Field {
            name,
            typ,
            index,
            options: None,
            repeated,
        }
    }

    pub fn set_options(&mut self, options: Option<FieldOption>) {
        self.options = options;
    }
}

impl TryFrom<TokenStream> for Field {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("field({:?})", tokens);

        tokens.next_eq(Type::Semicolon, "field line ending(';')")?;

        let options = match tokens.peek_eq(Type::RBrack) {
            true => {
                let option_tokens = tokens.select_block(Type::RBrack, Type::LBrack);
                Some(FieldOption::try_from(option_tokens)?)
            }
            false => None,
        };

        let index = tokens.intlit_as_i32("field index")?;
        tokens.next_eq(Type::Assign, "field assignment('=')")?;
        let name = tokens.ident_as_string("field name")?;
        let typ = tokens.fullident_as_string("field value")?;
        let repeated = tokens.peek_eq(Type::Repeated);

        let mut res = Self::new(name, typ, index, repeated);
        res.set_options(options);

        return Ok(res);
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        if self.repeated {
            write!(f, "repeated ")?;
        }

        match &self.options {
            Some(v) => writeln!(f, "{} {} = {} {};", self.typ, self.name, self.index, v),
            None => writeln!(f, "{} {} = {};",  self.typ, self.name, self.index),
        }
    }
}
