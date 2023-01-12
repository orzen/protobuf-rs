use log::warn;
use std::collections::VecDeque;

use crate::*;
use crate::position::Position;
use crate::token::{Keyword, Sym, Token};
use crate::types::{
    opt::Opt,
    field_option::FieldOption
};

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub typ: String,
    pub idx: i32,
    pub opts: FieldOption,
    pub is_repeated: bool,
}

impl Field {
    pub fn new(name: String, typ: String, idx: i32, is_repeated: bool) -> Self {
        Field {
            name,
            typ,
            idx,
            opts: FieldOption::new(),
            is_repeated,
        }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let first = tokens.pop_front()?;

        let repeated = matches!(first.keyword(), Some(Keyword::Repeat));

        let typ = match repeated {
            true => tokens.pop_front()?.full_ident()?,
            false => first.full_ident()?,
        };

        let name = tokens.pop_front()?.ident()?;

        sym!("field", tokens, pos, Sym::Equal);
        sym_back!("field", tokens, pos, Sym::Semi);

        let idx = tokens.pop_front()?.int()?;

        let mut field = Self::new(name, typ, idx as i32, repeated);

        let mut opts = FieldOption::new();
        if !tokens.is_empty() {
            opts = FieldOption::from_token(tokens, pos)?;
        }

        field.set_opts(opts);

        Some(field)
    }

    pub fn push_opt(&mut self, opt: Opt) {
        self.opts.push(opt);
    }

    pub fn set_opts(&mut self, opts: FieldOption) {
        self.opts = opts;
    }
}
