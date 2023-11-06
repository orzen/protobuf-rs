use std::fmt::Display;
use std::ops::Deref;

use crate::error::ParserError;
use crate::token::{Token, Type};

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

    pub fn peek(&self) -> Option<&Token> {
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

    pub fn set_inner(&mut self, inner: Vec<Token>) {
        self.inner = inner;
    }

    // Selections

    pub fn select_until(&mut self, until: Type) -> TokenStream {
        let mut selection = TokenStream::new();

        for token in &self.inner {
            selection.push(token.to_owned());

            if token.typ() == &until {
                break;
            }
        }

        return selection;
    }

    // Block swallows everything until the end token is found. It's designed to consume characters
    // before the actual block begins to include tokens like block identifiers and block names e.g.
    // `message MessageName { .. }`
    pub fn select_block(&mut self, begin: Type, end: Type) -> TokenStream {
        let mut selection = TokenStream::new();
        let mut counter = 0;

        for token in &self.inner {
            selection.push(token.to_owned());

            if token.typ() == &begin {
                counter += 1;
            }

            if token.typ() == &end {
                counter -= 1;

                if counter == 0 {
                    break;
                }
            }
        }

        return selection;
    }

    // Needs to be handled by token stream since we don't know how to make a selection until we
    // reached the end.
    pub fn select_block_comment(&mut self) -> Result<TokenStream, ParserError> {
        let mut ts = TokenStream::new();

        let open_slash = self.next_eq(Type::Slash, "block comment opening('/')")?;
        let open_aster = self.next_eq(Type::Asterisk, "block comment opening('*')")?;

        if open_slash.position().line() != open_aster.position().line()
            && (open_slash.position().char() + 1) != open_aster.position().char()
        {
            return Err(ParserError::Syntax(
                "block comment opening(\"/*\")".to_string(),
                format!("broken block comment opening near {open_aster}"),
            ));
        }

        ts.push(open_slash);
        ts.push(open_aster);

        while let Some(token) = self.pop() {
            let is_aster = token.typ().eq(&Type::Asterisk);
            let line = token.position().line();
            let character = token.position().char();
            ts.push(token);

            // Check for ending
            if is_aster {
                if let Some(peek_token) = self.peek() {
                    if peek_token.typ().eq(&Type::Slash)
                        && line == peek_token.position().line()
                        && (character + 1) == peek_token.position().char()
                    {
                        break;
                    }
                }
            }
        }

        // We already consumed the closing asterisk and only a slash should remain.
        let close_slash = self.next_eq(Type::Slash, "block comment closing('/')")?;
        ts.push(close_slash);

        Ok(ts)
    }

    pub fn select_line_comment(&mut self) -> Result<TokenStream, ParserError> {
        let mut tokens = TokenStream::new();

        let slash1 = self.next_eq(Type::Slash, "line comment first opening slash")?;
        let slash2 = self.next_eq(Type::Slash, "line comment second opening slash")?;

        if slash1.position().line() != slash2.position().line()
            || (slash1.position().char() + 1) != slash2.position().char()
        {
            return Err(ParserError::Syntax(
                "line comment opening(\"//\")".to_string(),
                format!("broken line comment opening near {slash2}"),
            ));
        }

        let line_num = slash2.position().line();

        tokens.push(slash1);
        tokens.push(slash2);

        while let Some(peek_token) = self.peek() {
            // Break if we found line ending
            if peek_token.position().line() > line_num {
                break;
            }

            match self.pop() {
                Some(v) => tokens.push(v),
                None => {
                    return Err(ParserError::Syntax(
                        "line comment values".to_string(),
                        "nothing".to_string(),
                    ))
                }
            }
        }

        Ok(tokens)
    }

    // Parser utils

    pub fn peek_eq(&self, expect: Type) -> bool {
        let next = self.inner.last();

        match next {
            Some(token) => token.typ() == &expect,
            None => return false,
        }
    }

    pub fn peeks_eq(&self, expect: &[Type]) -> bool {
        let needle: Vec<Token> = expect.iter().map(|typ| Token::from(typ.clone())).collect();
        self.inner.ends_with(needle.as_slice())
    }

    pub fn is_line_comment(&self) -> bool {
        self.peeks_eq(&[Type::Slash, Type::Slash])
    }

    pub fn is_block_comment(&mut self) -> bool {
        self.peeks_eq(&[Type::Slash, Type::Asterisk])
    }

    pub fn next_eq(&mut self, expect: Type, expect_msg: &str) -> Result<Token, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        if *token == expect {
            return Ok(token);
        } else {
            return Err(ParserError::Syntax(
                expect_msg.to_string(),
                format!("{token}"),
            ));
        }
    }

    pub fn next_contains(
        &mut self,
        expect: &[Type],
        expect_msg: &str,
    ) -> Result<Token, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        if expect.contains(&token) {
            Ok(token)
        } else {
            Err(ParserError::Syntax(
                expect_msg.to_string(),
                format!("{token}"),
            ))
        }
    }

    // Convertions

    pub fn intlit_as_i32(&mut self, expect_msg: &str) -> Result<i32, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        match token.typ() {
            Type::IntLit(v) => Ok(*v),
            invalid => Err(ParserError::Syntax(
                format!("{expect_msg}(intLit)"),
                format!("{invalid}"),
            )),
        }
    }

    pub fn optname_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        token.as_option_name()
    }

    pub fn ident_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        token.as_ident()
    }

    pub fn fullident_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        token.as_full_ident()
    }

    pub fn constant_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
        let token = match self.inner.pop() {
            Some(v) => v,
            None => {
                return Err(ParserError::Syntax(
                    expect_msg.to_string(),
                    "nothing".to_string(),
                ))
            }
        };

        token.as_const()
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TokenStream {{")?;
        for token in &self.inner {
            writeln!(f, "    {token}")?;
        }
        writeln!(f, "}}")
    }
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        position::{Point, Position},
        token::Token,
    };

    use super::*;

    fn types_to_tokens(types: &[Type]) -> Vec<Token> {
        types
            .iter()
            .map(|typ| Token::from(typ.clone()))
            .collect::<Vec<Token>>()
    }

    fn line_stream() -> TokenStream {
        let mut ts = TokenStream::new();

        let _ = &[
            Type::Syntax,
            Type::Ident("foo".to_string()),
            Type::Semicolon,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        return ts;
    }

    fn block_stream() -> TokenStream {
        let mut ts = TokenStream::new();

        let _ = &[
            Type::Message,
            Type::Ident("foo".to_string()),
            Type::LBrace,
            Type::Ident("bar".to_string()),
            Type::RBrace,
            Type::Package,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        return ts;
    }

    fn nested_block_stream() -> TokenStream {
        let mut ts = TokenStream::new();

        let _ = &[
            Type::Message,
            Type::Ident("foo".to_string()),
            Type::LBrace,
            Type::Ident("bar".to_string()),
            Type::Enum,
            Type::Ident("baz".to_string()),
            Type::LBrace,
            Type::Ident("inner".to_string()),
            Type::RBrace,
            Type::RBrace,
            Type::Package,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        return ts;
    }

    #[test]
    fn push_ok() {
        let mut ts = TokenStream::new();
        let mut expected = vec![];

        let tokens = &[Type::Syntax, Type::Ident("foo".to_string())]
            .iter()
            .map(|t| Token::from(t.clone()))
            .collect::<Vec<Token>>();

        for token in tokens {
            ts.push(token.clone());
            expected.push(token.clone());
        }

        assert_eq!(expected, ts.inner);
    }

    #[test]
    fn peek_and_pop_and_is_empty_ok() {
        let mut ts = line_stream();

        let peek = ts.peek();
        assert_eq!(Some(&Token::from(Type::Semicolon)), peek);
        assert_eq!(ts.is_empty(), false);

        let _discarded = ts.pop();
        let _discarded = ts.pop();
        let _discarded = ts.pop();

        let peek = ts.peek();
        assert_eq!(None, peek);
        assert_eq!(ts.is_empty(), true);
    }

    #[test]
    fn reverse_ok() {
        let mut ts = line_stream();
        let expected = &[
            Type::Semicolon,
            Type::Ident("foo".to_string()),
            Type::Syntax,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>();

        ts.reverse();

        assert_eq!(expected, &ts.inner);
    }

    #[test]
    fn select_until_ok() {
        let mut ts = line_stream();

        let expected = ts.clone();
        let actual = ts.select_until(Type::Semicolon);

        assert_eq!(expected, actual);
    }

    #[test]
    fn select_block_ok() {
        let mut ts = block_stream();

        let expected = &[
            Type::Message,
            Type::Ident("foo".to_string()),
            Type::LBrace,
            Type::Ident("bar".to_string()),
            Type::RBrace,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>();

        let actual = ts.select_block(Type::LBrace, Type::RBrace);

        assert_eq!(expected, &actual.inner);
    }

    #[test]
    fn select_nested_block_ok() {
        let mut ts = nested_block_stream();

        let expected = &[
            Type::Message,
            Type::Ident("foo".to_string()),
            Type::LBrace,
            Type::Ident("bar".to_string()),
            Type::Enum,
            Type::Ident("baz".to_string()),
            Type::LBrace,
            Type::Ident("inner".to_string()),
            Type::RBrace,
            Type::RBrace,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>();

        let actual = ts.select_block(Type::LBrace, Type::RBrace);

        assert_eq!(expected, &actual.inner);
    }

    #[test]
    fn select_block_comment() {
        let mut ts = TokenStream::new();

        // Add tokens for line 0
        ts.set_inner(types_to_tokens(&[
            Type::Slash,
            Type::Asterisk,
            Type::Ident("foo".to_string()),
        ]));

        // Add tokens for line 1
        let _line1 = &[
            Type::Asterisk,
            Type::Ident("bar".to_string()),
            Type::Slash,
            Type::Asterisk,
        ]
        .iter()
        .map(|t| {
            let mut token = Token::from(t.clone());
            token.set_position(Position::from(Point::new(1, 0)));
            token
        })
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        // The last slash needs to be located at the character after the closing asterisk.
        let mut last = Token::from(Type::Slash);
        last.set_position(Position::from(Point::new(1, 1)));
        ts.push(last);

        let expected = types_to_tokens(&[
            Type::Slash,
            Type::Asterisk,
            Type::Ident("foo".to_string()),
            Type::Asterisk,
            Type::Ident("bar".to_string()),
            Type::Slash,
            Type::Asterisk,
            Type::Slash,
        ]);

        ts.reverse();
        let actual = ts.select_block_comment();
        assert!(actual.is_ok());

        let actual = actual.unwrap();
        let it = expected.iter().zip(actual.inner.iter());

        for (e, a) in it {
            assert_eq!(e.typ(), a.typ());
        }
    }

    #[test]
    fn select_line_comment() {
        let mut ts = TokenStream::new();

        // TODO Continue here
        let slash1 = Token::from(Type::Slash);
        let mut slash2 = Token::from(Type::Slash);

        // Set correct position of slash2
        slash2.set_position(Position::from(Point::new(0,1)));

        ts.push(slash1);
        ts.push(slash2);

        let _ts_tokens = &[
            Type::Ident("foo".to_string()),
            Type::Slash,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        // `last` should be excluded from the comment selection since it's located on another line
        let mut last = Token::from(Type::Ident("bar".to_string()));
        last.set_position(Position::from(Point::new(1, 0)));

        ts.push(last);
        ts.reverse();

        let expected = types_to_tokens(&[
            Type::Slash,
            Type::Slash,
            Type::Ident("foo".to_string()),
            Type::Slash,
        ]);

        let actual = ts.select_line_comment();

        assert!(actual.is_ok());
        assert_eq!(expected, actual.unwrap().inner);
    }

    //    pub fn peek_eq(&self, expect: Type) -> bool {
    //    pub fn peeks_eq(&self, expect: &[Type]) -> bool {
    //    pub fn is_line_comment(&self) -> bool {
    //    pub fn is_block_comment(&mut self) -> bool {
    //    pub fn next_eq(&mut self, expect: Type, expect_msg: &str) -> Result<Token, ParserError> {
    //    pub fn next_contains(
    //    pub fn intlit_as_i32(&mut self, expect_msg: &str) -> Result<i32, ParserError> {
    //    pub fn optname_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
    //    pub fn ident_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
    //    pub fn fullident_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
    //    pub fn constant_as_string(&mut self, expect_msg: &str) -> Result<String, ParserError> {
    //    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //    fn deref(&self) -> &Self::Target {
}
