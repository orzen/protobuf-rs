use std::collections::VecDeque;

use crate::token::{Keyword, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub value: String,
    pub scope: Option<Keyword>,
}

impl Import {
    pub fn new(value: String, scope: Option<Keyword>) -> Self {
        Import { value, scope }
    }

    // The 'import' keyword has been consumed and this function will handle the remaining tokens.
    pub fn from_token(mut tokens: VecDeque<Token>) -> Option<Self> {
        let first = tokens.pop_front()?;

        let scope = match first.keyword()? {
            Keyword::Public => Some(Keyword::Public),
            Keyword::Weak => Some(Keyword::Weak),
            _ => None,
        };

        let value = match scope {
            Some(_) => tokens.pop_front()?.constant()?,
            None => first.constant()?,
        };

        Some(Import::new(value, scope))
    }
}
