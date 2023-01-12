use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::sym;
use crate::token::{
    Sym,
    Token,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub value: String,
}

impl Package {
    pub fn new(value: String) -> Self {
        Package { value }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let value = tokens.pop_front()?.full_ident()?;

        sym!("package", tokens, pos, Sym::Semi);

        Some(Self::new(value))
    }
}

impl Default for Package {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl ToString for Package {
    fn to_string(&self) -> String {
        format!("package \"{}\"\n", self.value)
    }
}
