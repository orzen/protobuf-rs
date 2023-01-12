use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::token::{Sym, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Syntax {
    pub value: String,
}

impl Syntax {
    pub fn new(value: String) -> Self {
        Syntax { value }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let value = tokens.pop_front()?.ident()?;

        if !value.eq("proto2") && !value.eq("proto3") {
            warn!("syntax expected 'proto2' or 'proto3', got '{:?}' {}", value, pos.near());
            return None;
        }

        match tokens.pop_front()?.sym()? {
            Sym::Semi => (),
            other => {
                warn!("syntax expected ';', got '{:?}' {}", other, pos.near());
                return None;
            }
        }

        Some(Self::new(value))
    }
}

impl Default for Syntax {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl ToString for Syntax {
    fn to_string(&self) -> String {
        format!("syntax = \"{}\"\n", self.value)
    }
}
