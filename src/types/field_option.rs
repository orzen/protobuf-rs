use log::warn;
use std::collections::VecDeque;

use crate::*;
use crate::position::Position;
use crate::token::{Sym, Token};
use crate::types::opt::Opt;


// Options following a field e.g. "int32 foo = 0 [packed=true];"
//                                               ^^^^^^^^^^^^^

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldOption {
    inner: Vec<Opt>,
}

impl FieldOption {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let mut fld_opt = Self::new();

        // Check for wrapping hardbrackets
        if !tokens.pop_front()?.sym()?.eq(&Sym::Hard0) || !tokens.pop_back()?.sym()?.eq(&Sym::Hard1)
        {
            warn!("field option miss wrapping brackets, {}", pos.near());
            return None;
        }

        while !tokens.is_empty() {
            let mut opt_tokens: VecDeque<Token> = VecDeque::new();
            while let Some(tok) = tokens.pop_front() {
                match tok {
                    Token::Symbol(Sym::Comma) => break,
                    other => opt_tokens.push_back(other),
                }
            }
            // Take token index 0,1,2 from the front which should correspond with: "key = value".
            // This should contain `key = value ,`
            let entry: Vec<Token> = tokens.drain(..2).collect();
            if entry.is_empty() {
                warn!("field option expected pair, got none {}", pos.near())
            }

            let key = entry[0].full_ident()?;
            let value = entry[2].constant()?;

            if !entry[1].sym()?.eq(&Sym::Equal) {
                warn!(
                    "field option expected '=' between key and value, got '{:?}' {}",
                    entry[1], pos.near()
                );
                return None;
            }

            fld_opt.push(Opt::new(key, value));

            // Break and skip the comma check when field option is out of pairs
            if tokens.is_empty() {
                break;
            }

            sym!("field option", tokens, pos, Sym::Comma);
        }

        Some(fld_opt)
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

    pub fn to_string(&self, n: u8) {
        let i = (0..n).map(|_| "\t").collect::<String>();
        for 
    }
}
