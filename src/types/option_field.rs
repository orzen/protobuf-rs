use log::warn;
use std::collections::VecDeque;

use crate::*;
use crate::position::Position;
use crate::token::{Sym, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct OptionField {
    inner: Opt,
}

impl OptionField {
    pub fn new(name: String, value: String) -> Self {
        let inner = Opt::new(name, value);
        Self { inner }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        sym_back!("option field", tokens, pos, Sym::Semi);

        Self { inner:  Opt::from_token(tokens)? }
    }

    pub fn set_custom(&mut self, value: bool) {
        self.inner.set_custom(value)
    }

    fn to_string(&self, n: u8) -> String {
        format!("option {};\n", self.inner.to_string(n));
    }
}
