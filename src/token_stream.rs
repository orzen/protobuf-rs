use std::ops::Deref;

use crate::error::ParserError;
use crate::token::Token;

const SPEC_URL: &str = "https://protobuf.dev/reference/protobuf/proto3-spec/#identifiers";

#[derive(Debug, Clone, PartialEq)]
pub struct TokenStream {
    inner: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> TokenStream {
        TokenStream { inner: vec![] }
    }

    pub fn push(&mut self, token: Token) {
        self.inner.push(token)
    }

    pub fn last(&self) -> Option<&Token> {
        self.inner.last()
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.inner.pop()
    }

    pub fn reverse(&mut self) {
        self.inner.reverse()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn line(&mut self, until: Token) -> TokenStream {
        let mut selection = TokenStream::new();

        for token in &self.inner {
            selection.push(token.to_owned());

            if token == &until {
                break;
            }
        }

        return selection;
    }

    // Block swallows everything until the end token is found. It's designed to consume characters
    // before the actual block begins to include tokens like block identifiers and block names e.g.
    // `message MessageName { .. }`
    pub fn block(&mut self, begin: Token, end: Token) -> TokenStream {
        let mut selection = TokenStream::new();
        let mut counter = 0;

        for token in &self.inner {
            selection.push(token.to_owned());

            if token == &begin {
                counter += 1;
            }

            if token == &end {
                counter -= 1;

                if counter == 0 {
                    break;
                }
            }
        }

        return selection;
    }

    // Parser utils
    //
    pub fn peek_is(&self, expect: Token) -> bool {
        let next = self.inner.last();

        if next == Some(&expect) {
            return true;
        }

        return false;
    }

    pub fn next_is(&mut self, expect: Token, expect_msg: &str) -> Result<Token, ParserError> {
        let next = self.inner.pop();

        if next != Some(expect) {
            return Err(ParserError::Syntax(
                expect_msg.to_string(),
                format!("{:?}", next),
            ));
        }

        Ok(next.unwrap())
    }

    pub fn next_contains(
        &mut self,
        expect: &[Token],
        expect_msg: &str,
    ) -> Result<Token, ParserError> {
        match self.inner.pop() {
            Some(v) => {
                if expect.contains(&v) {
                    Ok(v)
                } else {
                    Err(ParserError::Syntax(
                        expect_msg.to_string(),
                        format!("{:?}", v),
                    ))
                }
            }
            None => Err(ParserError::Syntax(
                expect_msg.to_string(),
                "nothing".to_string(),
            )),
        }
    }

    pub fn next_is_intlit(&mut self, expect_msg: &str) -> Result<i32, ParserError> {
        match self.inner.pop() {
            Some(Token::IntLit(v)) => Ok(v),
            Some(invalid) => {
                Err(ParserError::Syntax(
                format!("{expect_msg}(intLit)"),
                format!("{:?}", invalid),
            ))
            },
            None => Err(ParserError::Syntax(
                format!("{expect_msg}(intLit)"),
                "nothing".to_string(),
            )),
        }
    }

    pub fn next_is_ident(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        match self.inner.pop() {
            Some(Token::Ident(v)) => Ok(v.to_string()),
            Some(invalid) => Err(ParserError::Syntax(
                format!("{expect_msg}(ident, check {SPEC_URL} for more info)"),
                format!("{:?}", invalid),
            )),
            None => Err(ParserError::Syntax(
                format!("{expect_msg}(ident, check {SPEC_URL} for more info)"),
                "nothing".to_string(),
            )),
        }
    }

    pub fn next_is_fullident(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        match self.inner.pop() {
            Some(Token::Ident(v)) => Ok(v.to_string()),
            Some(Token::FullIdent(v)) => Ok(v.to_string()),
            Some(invalid) => Err(ParserError::Syntax(
                format!("{expect_msg}(fullIdent, check {SPEC_URL} for more info)"),
                format!("{:?}", invalid),
            )),
            None => Err(ParserError::Syntax(
                format!("{expect_msg}(fullIdent, check {SPEC_URL} for more info)"),
                "nothing".to_string(),
            )),
        }
    }

    pub fn next_is_constant(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        match self.inner.pop() {
            Some(Token::Ident(v)) => Ok(v.to_string()),
            Some(Token::FullIdent(v)) => Ok(v.to_string()),
            Some(Token::Constant(v)) => Ok(v.to_string()),
            Some(invalid) => Err(ParserError::Syntax(
                format!("{expect_msg}(constant, check {SPEC_URL} for more info)"),
                format!("{:?}", invalid),
            )),
            None => Err(ParserError::Syntax(
                format!("{expect_msg}(constant, check {SPEC_URL} for more info)"),
                "nothing".to_string(),
            )),
        }
    }
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
