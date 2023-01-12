use log::warn;
use std::collections::VecDeque;

use crate::*;
use crate::position::Position;
use crate::token::{Sym, Token};
use crate::types::{
    opt::Opt,
    field_option::FieldOption,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    name: String,
    key: String,
    value: String,
    index: i32,
    opts: FieldOption,
}

impl Map {
    pub fn new(name: String, key: String, value: String, index: i32) -> Self {
        Map {
            name,
            key,
            value,
            index,
            opts: FieldOption::new(),
        }
    }

    // The keyword 'map' has already been consumed. This function handle:
    // <K,V> = idx [o=x];
    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        sym!("map", tokens, pos, Sym::Lt);

        let key = tokens.pop_front()?.ident()?;

        sym!("map", tokens, pos, Sym::Comma);

        let val = tokens.pop_front()?.full_ident()?;

        sym!("map", tokens, pos, Sym::Gt);

        let name = tokens.pop_front()?.ident()?;

        sym!("map", tokens, pos, Sym::Equal);

        let idx = tokens.pop_front()?.int()?;

        sym_back!("map", tokens, pos, Sym::Semi);

        let mut map = Map::new(name, key, val, idx as i32);

        if tokens.len() > 1 {
            map.set_opts(FieldOption::from_token(tokens, pos)?.inner());
        }

        Some(map)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o);
    }

    pub fn set_opts(&mut self, i: Vec<Opt>) {
        self.opts.set_inner(i);
    }
}
