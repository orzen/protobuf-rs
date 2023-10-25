use std::fmt::Display;

use log::debug;

use crate::{error::ParserError, token::Token, token_stream::TokenStream};

#[derive(Clone, Debug, PartialEq)]
pub enum ImportScope {
    Weak,
    Public,
}

impl TryFrom<Token> for ImportScope {
    type Error = ParserError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Weak => Ok(ImportScope::Weak),
            Token::Public => Ok(ImportScope::Public),
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

        tokens.next_is(Token::Semicolon, "import line ending(';')")?;
        let value = tokens.next_is_constant("import value")?;

        // Check for scope
        let mut scope = None;
        if !tokens.peek_is(Token::Import) {
            scope = match tokens.next_contains(&[Token::Public, Token::Weak], "import scope") {
                Ok(v) => Some(ImportScope::try_from(v)?),
                Err(_) => None,
            };
        }

        tokens.next_is(Token::Import, "import identifier")?;

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
