use log::{debug, warn};
use regex::Regex;
use std::io::Read;

use crate::buffer::Buffer;
use crate::error::LexerError;
use crate::position::{Point, Position};
use crate::token::{Token, Type};
use crate::token_stream::TokenStream;

// Boolean
const BOOL_LIT: &str = r"(true | false)";

// Identifiers
const IDENT: &str = r"[a-zA-Z0-9_]+";
const FULL_IDENT: &str = r"([a-zA-Z0-9_]+[.]?)+";
const OPT_LIT: &str = r"([a-zA-Z0-9\(\)\.]+)";

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
//  Hex escape values
const HEX_ESCAPE: &str = "(\\[xX]{1}[a-fA-F0-9]{2})";
//  Oct escape values
const OCT_ESCAPE: &str = "(\\[0-7]{3})";
// Char escape values
const CHAR_ESCAPE: &str = "\\[abfnrtv]{1}";

pub struct Lexer {
    pos: Point,

    ptn_intlit: Regex,
    ptn_ident: Regex,
    ptn_full_ident: Regex,
    ptn_const: Regex,
    ptn_optlit: Regex,
}

impl Lexer {
    pub fn new() -> Result<Self, LexerError> {
        let int_lit = format!("({}) | ({}) | ({})", DEC_LIT, OCT_LIT, HEX_LIT);
        let str_lit_base = format!("({} | {} | {} )", HEX_ESCAPE, OCT_ESCAPE, CHAR_ESCAPE);
        let str_lit = format!("( \"{}\" | \'{}\' )", str_lit_base, str_lit_base);
        let float_lit = format!(
            "( {} | {} | {} | {} )",
            FLOAT_LIT1, FLOAT_LIT2, FLOAT_LIT3, FLOAT_LIT4
        );
        let constant = format!(
            "{} | [+-]?{} | [+-]?{} | {} | {}",
            FULL_IDENT, int_lit, float_lit, str_lit, BOOL_LIT
        );

        // Patterns
        let ptn_const = Regex::new(&constant)?;
        let ptn_full_ident = Regex::new(FULL_IDENT)?;
        let ptn_ident = Regex::new(IDENT)?;
        let ptn_intlit = Regex::new(&int_lit)?;
        let ptn_optlit = Regex::new(OPT_LIT)?;

        Ok(Lexer {
            pos: Point::new(0, 0),

            // Patterns
            ptn_intlit,
            ptn_ident,
            ptn_full_ident,
            ptn_const,
            ptn_optlit,
        })
    }

    fn range(&self, stash: &str) -> Position {
        Position::range(self.pos) + stash.len()
    }

    fn is_discardable(&self, ch: &char) -> bool {
        matches!(ch, '\n' | '\r' | '\t' | ' ')
    }

    fn is_match(ptn: &Regex, stash: &str) -> bool {
        if let Some(matched) = ptn.find(stash) {
            if stash.eq(matched.as_str()) {
                return true;
            }
        }
        return false;
    }

    fn match_literals(&self, stash: &str) -> Type {
        // NOTE: The order in this list will dictate the token priority. Changing the order can
        // cause some tokens to become unreachable.

        match stash {
            "true" => return Type::BoolLit(true),
            "false" => return Type::BoolLit(false),
            _ => (),
        }

        if Self::is_match(&self.ptn_intlit, stash) {
            match stash.parse::<i32>() {
                Ok(v) => {
                    return Type::IntLit(v);
                }
                Err(e) => {
                    warn!("failed to convert string to int literal {e}");
                    return Type::Illegal;
                }
            }
        }

        if Self::is_match(&self.ptn_ident, stash) {
            return Type::Ident(stash.to_string());
        }

        if Self::is_match(&self.ptn_full_ident, stash) {
            return Type::FullIdent(stash.to_string());
        }

        if Self::is_match(&self.ptn_const, stash) {
            return Type::Constant(stash.to_string());
        }

        if Self::is_match(&self.ptn_optlit, stash) {
            return Type::OptionName(stash.to_string());
        }

        return Type::Illegal;
    }

    pub fn tokenize(&self, ch: char, stash: &mut String) -> Option<Token> {
        // Ignored characters
        if self.is_discardable(&ch) {
            return None;
        }

        // Match single character tokens
        if stash.is_empty() {
            match Type::from(&ch) {
                Type::Illegal => (),
                typ => return Some(Token::from(typ)),
            }
        }

        // Stash character to build multi-character tokens
        stash.push(ch);

        // Match keywords against stash
        match Type::from(stash.as_str()) {
            Type::Illegal => (),
            typ => return Some(Token::from(typ)),
        }

        match self.match_literals(stash) {
            Type::Illegal => (),
            typ => return Some(Token::from(typ)),
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
                    self.pos *= 0;
                }

                let mut token = match self.tokenize(ch, &mut stash) {
                    Some(v) => v,
                    None => {
                        debug!("stashing: {ch}, stash: {stash}");
                        continue;
                    }
                };

                if token.typ().eq(&Type::Illegal) {
                    debug!("invalid token: '{stash}' {}", self.range(&stash));
                    return Err(LexerError::Invalid(stash.clone()));
                }

                // Complete the token by assigning it the correct position
                let pos = self.range(&stash);
                debug!("identified token '{token}' and updating its position to {pos}");
                token.set_position(pos);

                return Ok(token);
            }
        }

        return Err(LexerError::Invalid(format!("stash: {stash}")));
    }

    pub fn token_stream<T>(&mut self, mut buf: Buffer<T>) -> Result<TokenStream, LexerError>
    where
        T: Read,
    {
        let mut tokens = TokenStream::new();
        let eof = Token::from(Type::EOF);

        loop {
            match self.next_token(&mut buf) {
                Ok(token) => {
                    if token == eof {
                        break;
                    }

                    tokens.push(token);
                }
                Err(e) => return Err(e),
            }
        }

        return Ok(tokens);
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::io::BufReader;
//    use std::io::Cursor;
//
//    #[test]
//    fn new() {
//        let file = Cursor::new(String::from("foobar"));
//        let inner = BufReader::new(file);
//        let buf = Buffer::new(inner);
//
//        Lexer::new(buf);
//    }
//
//    #[test]
//    fn syntax() {
//        let pb = "syntax = \"proto3\'";
//        let cursor = Cursor::new(pb);
//        let bf = BufReader::new(cursor);
//        let inner = Buffer::new(bf);
//
//        let mut lexer = Lexer::new(inner);
//
//        let mut expected = TokenStream::new();
//        expected.push(Token::from(Type::Syntax));
//        expected.push(Token::from(Type::Assign));
//        expected.push(Token::constant("proto3".to_string()));
//
//        let actual = match lexer.token_stream() {
//            Ok(v) => v,
//            Err(e) => panic!("got error '{e }'"),
//        };
//
//        assert_eq!(expected, actual);
//    }
//}
