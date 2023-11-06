use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Type;
use crate::token_stream::TokenStream;
use crate::types::rpc_option::RpcOption;

#[derive(Clone, Debug, PartialEq)]
pub struct Rpc {
    pub name: String,
    pub arg: String,
    pub ret: String,
    pub options: Option<RpcOption>,
    pub stream_arg: bool,
    pub stream_ret: bool,
}

impl Rpc {
    pub fn new(name: String, arg: String, ret: String, stream_arg: bool, stream_ret: bool) -> Self {
        Rpc {
            name,
            arg,
            ret,
            options: None,
            stream_arg,
            stream_ret,
        }
    }

    pub fn set_options(&mut self, options: Option<RpcOption>) {
        self.options = options;
    }
}

impl TryFrom<TokenStream> for Rpc {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("rpc({:?})", tokens);

        tokens.next_eq(Type::Semicolon, "line ending(';')")?;

        // Check for RPC options
        let options = match tokens.peek_eq(Type::RBrace) {
            true => {
                let option_tokens = tokens.select_block(Type::RBrace, Type::LBrace);
                Some(RpcOption::try_from(option_tokens)?)
            }
            false => None,
        };

        // Handle return value
        tokens.next_eq(Type::RParen, "rpc closing return parenthesis(')')")?;
        let ret = tokens.fullident_as_string("rpc return type")?;

        // Check for streamed return
        let stream_ret = tokens.peek_eq(Type::Stream);
        if stream_ret {
            // Pop one since we used peek to determine the stream_ret
            tokens.pop();
        }

        tokens.next_eq(Type::LParen, "rpc opening return parenthesis('(')")?;
        tokens.next_eq(Type::Returns, "rpc returns")?;

        // Handle argument
        tokens.next_eq(Type::RParen, "rpc closing argument parenthesis(')')")?;
        let arg = tokens.fullident_as_string("rpc argument type")?;

        // Check for streamed argument
        let stream_arg = tokens.peek_eq(Type::Stream);
        if stream_arg {
            // Pop one since we used peek to determine the stream_arg
            tokens.pop();
        }

        tokens.next_eq(Type::LParen, "rpc opening argument parenthesis('(')")?;

        let name = tokens.ident_as_string("rpc name")?;

        tokens.next_eq(Type::RPC, "rpc identifier")?;

        let mut res = Rpc::new(name, arg, ret, stream_arg, stream_ret);
        res.set_options(options);

        return Ok(res);
    }
}

impl Display for Rpc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;

        write!(f, "rpc {} ", self.name)?;

        match self.stream_arg {
            true => write!(f, "(stream {}) ", self.arg)?,
            false => write!(f, "({}) ", self.arg)?,
        }

        match self.stream_ret {
            true => write!(f, "returns (stream {})", self.ret)?,
            false => write!(f, "returns ({})", self.ret)?,
        }

        match &self.options {
            Some(v) => write!(f, " {v};"),
            None => write!(f, ";"),
        }
    }
}
