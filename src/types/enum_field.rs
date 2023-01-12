use crate::fmt_ident;
use crate::position::Position;
use crate::token::{Sym, Token};
use crate::types::field_option::FieldOption;
use crate::types::opt::Opt;
use std::collections::VecDeque;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnumField {
    pub name: String,
    pub index: i32,
    pub opts: FieldOption,
}

impl EnumField {
    pub fn new(name: String, index: i32) -> Self {
        EnumField { name, index, opts: FieldOption::new() }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let name = tokens.pop_front()?.ident()?;

        // Check that the next token is the equal sign that separates the field name and the index
        // also check that the ends with a semi colon.
        if !tokens.pop_front()?.sym()?.eq(&Sym::Equal) || !tokens.pop_back()?.sym()?.eq(&Sym::Semi)
        {
            return None;
        }

        let index = tokens.pop_front()?.int()?;

        // Note: Casting i64 to i32 here. This should be fine because index values should not reach
        // higher than the i32_MAX.
        let mut field = EnumField::new(name, index as i32);

        // There should not be any tokens remaining unless there's field options so lets try and
        // parse field options.
        if !tokens.is_empty() {
            field.set_opts(FieldOption::from_token(tokens, pos)?);
        }

        Some(field)
    }

    pub fn push_opt(&mut self, opt: Opt) {
        self.opts.push(opt);
    }

    pub fn set_opts(&mut self, opts: FieldOption) {
        self.opts = opts;
    }

    pub fn to_string(&self, n: u8) -> String {
        fmt_ident!(n, "{} = {}{};", self.name, self.index, self.opts)
    }
}
