use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::{keyword, sym, sym_back};
use crate::token::{Keyword, Sym, Token};
use crate::types::{opt::Opt, rpc_option::RpcOption};

#[derive(Clone, Debug, PartialEq)]
pub struct Rpc {
    pub name: String,
    pub arg: String,
    pub ret: String,
    pub opts: RpcOption,
    pub stream_arg: bool,
    pub stream_ret: bool,
}

impl Rpc {
    pub fn new(name: String, arg: String, ret: String, stream_arg: bool, stream_ret: bool) -> Self {
        Rpc {
            name,
            arg,
            ret,
            opts: RpcOption::new(),
            stream_arg,
            stream_ret,
        }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let mut stream_arg = false;
        let mut stream_ret = false;
        let arg: String;
        let ret: String;

        let name = tokens.pop_front()?.ident()?;

        // Request definition

        sym!("rpc", tokens, pos, Sym::Par0);

        match tokens.pop_front()?.keyword() {
            Some(Keyword::Stream) => {
                stream_arg = true;
                arg = tokens.pop_front()?.full_ident()?;
            }
            Some(other) => {
                warn!(
                    "rpc expected 'stream' or request, got '{:?}' {}",
                    other,
                    pos.near()
                );
                return None;
            }
            None => {
                arg = tokens.pop_front()?.full_ident()?;
            }
        }

        sym!("rpc", tokens, pos, Sym::Par1);

        // Response definition

        keyword!("rpc", tokens, pos, Keyword::Returns);

        sym!("rpc", tokens, pos, Sym::Par0);

        match tokens.pop_front()?.keyword() {
            Some(Keyword::Stream) => {
                stream_ret = true;
                ret = tokens.pop_front()?.full_ident()?;
            }
            Some(other) => {
                warn!(
                    "rpc expected 'stream' or request, got '{:?}' {}",
                    other,
                    pos.near()
                );
                return None;
            }
            None => {
                ret = tokens.pop_front()?.full_ident()?;
            }
        }

        sym!("rpc", tokens, pos, Sym::Par1);

        // Ending, option or empty statement

        sym_back!("rpc", tokens, pos, Sym::Semi);

        let mut opts = RpcOption::new();
        if !tokens.is_empty() {
            opts = RpcOption::from_token(tokens, pos)?;
        }

        let mut r = Rpc::new(name, arg, ret, stream_arg, stream_ret);
        r.set_opts(opts);

        Some(r)
    }

    pub fn push_opt(&mut self, o: Opt) {
        self.opts.push(o);
    }

    pub fn set_opts(&mut self, opts: RpcOption) {
        self.opts = opts;
    }
}
