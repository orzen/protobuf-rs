use std::collections::VecDeque;

use crate::buffer::Buffer;
use crate::lexer::Lexer;
use crate::token::{
    Keyword,
    Sym,
    Token,
};
use crate::types::{
    enumerate::Enum,
    import::Import,
    message::Message,
    opt::Opt,
    package::Package,
    proto::Proto,
    service::Service,
    syntax::Syntax,
};

pub struct Parser<T> {
    lexer: Lexer<T>,
}

// Public functions
impl<T: std::io::Read> Parser<T> {
    pub fn new(inner: Buffer<T>) -> Self {
        let lexer = Lexer::new(inner);
        Self { lexer }
    }

    // End of line
    fn until_eol(lexer: &mut Lexer<T>) -> VecDeque<Token>
    where
        T: std::io::Read,
    {
        let mut tokens: VecDeque<Token> = VecDeque::new();

        while let Some(token) = lexer.next() {
            match token {
                Token::Symbol(Sym::Semi) => {
                    tokens.push_back(token);
                    break;
                }
                _ => tokens.push_back(token),
            }
        }

        tokens
    }

    // End of block
    fn until_eob(lexer: &mut Lexer<T>) -> VecDeque<Token>
    where
        T: std::io::Read,
    {
        let mut nested = 0;
        let mut tokens: VecDeque<Token> = VecDeque::new();

        while let Some(token) = lexer.next() {
            nested = Token::is_nested(&nested, &token);

            tokens.push_back(token);

            if nested == 0 {
                break;
            }
        }

        tokens
    }

    fn async_proto(lexer: &mut Lexer<T>) -> Option<Proto>
    where
        T: std::io::Read,
    {
        let mut proto = Proto::new();

        while let Some(token) = lexer.next() {
            let start = lexer.pos();

            match token.keyword()? {
                Keyword::Enum => {
                    let block = Self::until_eob(lexer);

                    proto.push_enum(Enum::from_token(block, start)?);
                }
                Keyword::Import => {
                    let line = Self::until_eol(lexer);

                    proto.push_import(Import::from_token(line)?);
                }
                Keyword::Message => {
                    let block = Self::until_eob(lexer);

                    proto.push_message(Message::from_token(block, start)?)
                }
                Keyword::Opt => {
                    let line = Self::until_eol(lexer);

                    proto.push_opt(Opt::from_token(line, start)?);
                }
                Keyword::Package => {
                    let line = Self::until_eol(lexer);

                    proto.set_package(Package::from_token(line, start)?);
                }
                Keyword::Service => {
                    let block = Self::until_eob(lexer);

                    proto.push_service(Service::from_token(block, start)?)
                }
                Keyword::Syntax => {
                    let line = Self::until_eol(lexer);
                    let s = Syntax::from_token(line, start)?;

                    proto.set_syntax(s);
                }
                other => {
                    unreachable!("unhandled keyword {:?}", other);
                }
            }
        }

        Some(proto)
    }

    pub fn as_proto(&mut self) -> Option<Proto> {
        Self::async_proto(&mut self.lexer)
    }
}

//#[cfg(test)]
//mod tests {
//    use std::fs::File;
//    use std::io::BufReader;
//
//    use super::*;
//
//    #[test]
//    fn new_from_file() {
//    }
//}
