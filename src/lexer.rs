use log::{debug, warn};
use regex::Regex;
use std::io::Read;

use crate::buffer::Buffer;
use crate::error::LexerError;
use crate::position::Position;
use crate::token::Token;
use crate::token_stream::TokenStream;

// Boolean
const BOOL_LIT: &str = r"(true | false)";

// Identifiers
const IDENT: &str = r"[a-zA-Z0-9_]+";
const FULL_IDENT: &str = r"([a-zA-Z0-9_]+[.]?)+";

// Integer literals
const DEC_LIT: &str = r"([1-9]?[0-9]+)";
const OCT_LIT: &str = r"(0[0-7]+)";
const HEX_LIT: &str = r"(0[xX]?[a-fA-F0-9]+)";

// Floating-point literals
const FLOAT_LIT1: &str = r"([0-9]+\.[0-9]*([eE]?[+-]?[0-9]+)?)";
const FLOAT_LIT2: &str = r"([0-9]+([eE]?[+-]?[0-9]+)?)";
const FLOAT_LIT3: &str = r"(\.[0-9]+([eE]?[+-]?[0-9]+)?)";
const FLOAT_LIT4: &str = r"(inf | nan)";

// String literals
//const INVALID_CHAR_BEGIN: &str = "[^\0\n\\]";
//  Hex escape values
const HEX_ESCAPE: &str = "(\\[xX]{1}[a-fA-F0-9]{2})";
//  Oct escape values
const OCT_ESCAPE: &str = "(\\[0-7]{3})";
// Char escape values
const CHAR_ESCAPE: &str = "\\[abfnrtv]{1}";

enum State<'s> {
    Default,
    Comment,
    BlockComment,
    Constant(&'s char),
}

impl<'s> Default for State<'s> {
    fn default() -> Self {
        Self::Default
    }
}

pub struct Lexer<'l> {
    pos: Position,
    state: State<'l>,

    ptn_intlit: Regex,
    ptn_ident: Regex,
    ptn_full_ident: Regex,
    ptn_const: Regex,
}

impl<'l> Lexer<'l> {
    pub fn new() -> Lexer<'l> {
        let int_lit = format!("({}) | ({}) | ({})", DEC_LIT, OCT_LIT, HEX_LIT);
        let str_lit_base = format!("({} | {} | {} )", HEX_ESCAPE, OCT_ESCAPE, CHAR_ESCAPE);
        let str_lit = format!("( \"{}\" | \'{}\' )", str_lit_base, str_lit_base);
        let float_lit = format!(
            "( {} | {} | {} | {} )",
            FLOAT_LIT1, FLOAT_LIT2, FLOAT_LIT3, FLOAT_LIT4
        );
        // TODO fix constant
        // - Add quote to contant
        // - Add second constant pattern with single quote
        let constant = format!(
            "{} | [+-]?{} | [+-]?{} | {} | {}",
            FULL_IDENT, int_lit, float_lit, str_lit, BOOL_LIT
        );

        // Patterns
        let ptn_const = Regex::new(&constant).expect("compile const1 pattern");
        let ptn_full_ident = Regex::new(FULL_IDENT).expect("compile full ident pattern");
        let ptn_ident = Regex::new(IDENT).expect("compile ident pattern");
        let ptn_intlit = Regex::new(&int_lit).expect("compile ident pattern");

        Lexer {
            pos: Position::new(0, 0),
            state: State::default(),

            // Patterns
            ptn_intlit,
            ptn_ident,
            ptn_full_ident,
            ptn_const,
        }
    }

    fn end_state(&self, ch: &char) -> bool {
        match self.state {
            State::Default => false,
            State::Comment => matches!(ch, '\n' | '\r' | '\t' | ' '),
            State::BlockComment => true,
            State::Constant(delimiter) => ch == delimiter,
        }
    }

    fn is_discardable(&self, ch: &char) -> bool {
        match self.state {
            State::Default => matches!(ch, '\n' | '\r' | '\t' | ' '),
            State::Comment => matches!(ch, '\n' | '\r' | '\t' | ' '),
            State::BlockComment => true,
            State::Constant(delimiter) => true,
        }
    }

    fn match_literals(&self, stash: &'l str) -> Token {
        // NOTE: The order in this list will dictate the token priority. Changing the order can
        // cause some tokens to become unreachable.

        match stash {
            "true" => return Token::BoolLit(true),
            "false" => return Token::BoolLit(false),
            _ => (),
        }

        if self.ptn_intlit.is_match(stash) {
            match stash.parse::<i32>() {
                Ok(v) => {
                    return Token::IntLit(v);
                }
                Err(e) => {
                    warn!("failed to convert string to int literal {e}");
                    return Token::Illegal;
                }
            }
        }

        if self.ptn_ident.is_match(stash) {
            return Token::Ident(stash.to_string());
        }

        if self.ptn_full_ident.is_match(stash) {
            return Token::FullIdent(stash.to_string());
        }

        if self.ptn_const.is_match(stash) {
            return Token::Constant(stash.to_string());
        }

        return Token::Illegal;
    }

    pub fn tokenize(&self, ch: char, stash: &mut String) -> Option<Token> {
        // Ignored characters
        if self.is_discardable(&ch) {
            return None;
        }

        // Match single character tokens
        if stash.is_empty() {
            match Token::from(&ch) {
                Token::Illegal => (),
                token => return Some(token),
            }
        }

        // Stash character to build multi-character tokens
        stash.push(ch);

        // Match keywords against stash
        match Token::from(stash.as_str()) {
            Token::Illegal => (),
            token => return Some(token),
        }

        match self.match_literals(stash) {
            Token::Illegal => (),
            token => return Some(token),
        }

        return None;
    }

    pub fn next_token<T>(&mut self, buf: &mut Buffer<T>) -> Result<Token, LexerError>
    where
        T: Read,
    {
        let mut stash = String::new();
        while let Some(chars) = buf.next() {
            for ch in chars {
                if ch.eq(&'\n') {
                    self.pos *= 1;
                } else {
                    self.pos += 1;
                }

                let token = self.tokenize(ch, &mut stash);

                match token {
                    Some(Token::Illegal) => {
                        debug!("invalid token '{stash}'");
                        return Err(LexerError::Invalid(stash.clone()));
                    }
                    Some(token) => {
                        debug!("identified token: {:?}", token);
                        return Ok(token);
                    }
                    None => {
                        debug!("stashing: {ch}, stash: {stash}");
                        continue;
                    }
                }
            }
        }

        Ok(Token::Illegal)
    }

    pub fn token_stream<T>(&mut self, mut buf: Buffer<T>) -> Result<TokenStream, LexerError>
    where
        T: Read,
    {
        let mut tokens = TokenStream::new();
        while let Ok(token) = self.next_token(&mut buf) {
            tokens.push(token.clone());

            if token == Token::EOF {
                break;
            }
        }

        return Ok(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn new() {
        let file = Cursor::new(String::from("foobar"));
        let inner = BufReader::new(file);
        let buf = Buffer::new(inner);

        Lexer::new(buf);
    }

    #[test]
    fn syntax() {
        let pb = "syntax = \"proto3\'";
        let cursor = Cursor::new(pb);
        let bf = BufReader::new(cursor);
        let inner = Buffer::new(bf);

        let mut lexer = Lexer::new(inner);

        let mut expected = TokenStream::new();
        expected.push(Token::Syntax);
        expected.push(Token::Assign);
        expected.push(Token::DQuote);
        expected.push(Token::Ident("proto3".to_string()));
        expected.push(Token::DQuote);

        let actual = match lexer.token_stream() {
            Ok(v) => v,
            Err(e) => panic!("got error '{e }'"),
        };

        assert_eq!(expected, actual);
    }
}
