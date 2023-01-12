use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::token::{Sym, Token};
use crate::types::opt::Opt;
use crate::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RpcOption {
    inner: Vec<Opt>,
}

impl RpcOption {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let mut rpc_opt = Self::new();

        // Check for wrapping hardbrackets
        if tokens.pop_front()?.sym()? != Sym::Curly0 || tokens.pop_back()?.sym()? != Sym::Curly1 {
            warn!("rpc option miss wrapping brackets, {}", pos.near());
            return None;
        }

        while !tokens.is_empty() {
            // Take token index 0,1,2 from the front which should correspond with: "key = value".
            // This should contain `key = value ,`
            let entry: Vec<Token> = tokens.drain(..2).collect();
            if entry.is_empty() {
                warn!("rpc option expected pair, got none {}", pos.near());
            }

            let key = entry[0].full_ident()?;
            let value = entry[2].constant()?;

            sym!("rpc option", tokens, pos, Sym::Equal);

            rpc_opt.push(Opt::new(key, value));

            // Break and skip the comma check when field option is out of pairs
            if tokens.is_empty() {
                break;
            }

            sym!("rpc option", tokens, pos, Sym::Comma);
        }

        Some(rpc_opt)
    }

    pub fn inner(&self) -> Vec<Opt> {
        self.inner.clone()
    }

    pub fn set_inner(&mut self, i: Vec<Opt>) {
        self.inner = i;
    }

    pub fn push(&mut self, o: Opt) {
        self.inner.push(o);
    }
}
