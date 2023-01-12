use log::warn;
use std::collections::VecDeque;

use crate::position::Position;
use crate::token::{Keyword, Token};
use crate::types::{
    rpc::Rpc,
    opt::Opt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Service {
    pub name: String,
    pub opts: Vec<Opt>,
    pub rpcs: Vec<Rpc>,
}

impl Service {
    pub fn new(name: String) -> Self {
        let opts: Vec<Opt> = Vec::new();
        let rpcs: Vec<Rpc> = Vec::new();

        Service { name, opts, rpcs }
    }

    pub fn from_token(mut tokens: VecDeque<Token>, pos: Position) -> Option<Self> {
        let name = tokens.pop_front()?.ident()?;
        let mut service = Service::new(name);

        while !tokens.is_empty() {
            let kw = tokens.pop_front()?.keyword();
            let line = Token::as_line(&mut tokens);

            match kw {
                Some(Keyword::Opt) => match Opt::from_token(line, pos) {
                    Some(opt) => service.push_opt(opt),
                    None => warn!("service expected option, got none {}", pos.near()),
                },
                Some(Keyword::Rpc) => match Rpc::from_token(line, pos) {
                    Some(rpc) => service.push_rpc(rpc),
                    None => warn!("service expected rpc, got none {}", pos.near()),
                },
                Some(other) => {
                    warn!("expected option or rpc for service, got {:?} {}", other, pos.near())
                }
                None => {
                    warn!("expected keyword for service, got none {}", pos.near());
                }
            }
        }

        Some(service)
    }

    pub fn push_opt(&mut self, opt: Opt) {
        self.opts.push(opt)
    }

    pub fn push_rpc(&mut self, rpc: Rpc) {
        self.rpcs.push(rpc)
    }
}
