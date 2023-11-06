use std::fmt::Display;
use log::debug;

use crate::{error::ParserError, token::{Token, Type}, token_stream::TokenStream};

#[derive(Clone, Debug, PartialEq)]
pub enum ImportScope {
    Weak,
    Public,
}

impl TryFrom<Token> for ImportScope {
    type Error = ParserError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token.typ() {
            Type::Weak => Ok(ImportScope::Weak),
            Type::Public => Ok(ImportScope::Public),
            invalid => Err(ParserError::Syntax(
                "import scope(weak, public)".to_string(),
                format!("{:?}", invalid),
            )),
        }
    }
}

impl Display for ImportScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = match self {
            Self::Public => "public",
            Self::Weak => "weak",
        };
        write!(f, "{scope}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub value: String,
    pub scope: Option<ImportScope>,
}

impl Import {
    pub fn new(value: String) -> Self {
        Import { value, scope: None }
    }

    pub fn set_scope(&mut self, scope: Option<ImportScope>) {
        self.scope = scope;
    }
}

impl TryFrom<TokenStream> for Import {
    type Error = ParserError;

    // Parsed backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("import({:?})", &tokens);

        tokens.next_eq(Type::Semicolon, "import line ending(';')")?;
        let value = tokens.constant_as_string("import value")?;

        // Check for scope
        let mut scope = None;
        if !tokens.peek_eq(Type::Import) {
            scope = match tokens.next_contains(&[Type::Public, Type::Weak], "import scope") {
                Ok(v) => Some(ImportScope::try_from(v)?),
                Err(_) => None,
            };
        }

        tokens.next_eq(Type::Import, "import identifier")?;

        let mut res = Self::new(value);
        res.set_scope(scope);

        return Ok(res);
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.scope {
            Some(v) => write!(f, "import {v} \"{}\";", self.value),
            None => write!(f, "import \"{}\";", self.value),
        }
    }
}
