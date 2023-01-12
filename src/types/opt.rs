use log::warn;
use std::collections::VecDeque;

use crate::*;
use crate::position::Position;
use crate::token::{Sym, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct Opt {
    pub name: String,
    pub value: String,
    pub custom: bool,
}

impl Opt {
    pub fn new(name: String, value: String) -> Self {
        Opt {
            name,
            value,
            custom: false,
        }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let mut name = String::new();
        let value: String;

        // Check if option is a custom option. Custom option names are wrapped in parentheses.
        let is_custom = match tokens.pop_front()? {
            Token::Symbol(Sym::Par0) => true,
            Token::FullIdent(n) => {
                name = n;
                false
            }
            other => {
                warn!(
                    "option expected '(' or name, got '{:?}' {}",
                    other,
                    pos.near()
                );
                return None;
            }
        };

        // Handling the rest of the elements depending on if it's a custom option or not.
        if is_custom {
            name = tokens.pop_front()?.full_ident()?;

            sym!("option", tokens, pos, Sym::Par1);

            value = tokens.pop_front()?.constant()?;
        } else {
            // The name was already added when checked for custom option.
            value = tokens.pop_front()?.constant()?;
        }

        let mut opt = Opt::new(name, value);
        opt.set_custom(is_custom);

        Some(opt)
    }

    pub fn set_custom(&mut self, value: bool) {
        self.custom = value;
    }

    pub fn to_string(&self, n: u8) -> String {
        if self.custom {
            return fmt_indent!(n, "({}) = {}", self.name, self.value)
        }

        format!("{} = {}", self.name, self.value)
    }
}
