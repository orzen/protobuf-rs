use log::warn;
use regex::Regex;
use std::io::Read;

use crate::buffer::Buffer;
use crate::position::Position;
use crate::token::{
    Sym,
    Token,
};

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

pub struct Lexer<T> {
    inner: Buffer<T>,
    line_buf: String,
    stash: String,
    in_strlit: bool,
    strlit_quote: Sym,
    // Position
    column: i32,
    line: i32,
    ptn_boollit: Regex,
    ptn_intlit: Regex,
    ptn_floatlit: Regex,
    ptn_ident: Regex,
    ptn_full_ident: Regex,
    ptn_const: Regex,
}

impl<T: Read> Lexer<T> {
    pub fn new(inner: Buffer<T>) -> Self {
        let int_lit = format!("({}) | ({}) | ({})", DEC_LIT, OCT_LIT, HEX_LIT);
        let str_lit_base = format!(
            "({} | {} | {} )",
            HEX_ESCAPE, OCT_ESCAPE, CHAR_ESCAPE
        );
        let str_lit = format!("( \"{}\" | \'{}\' )", str_lit_base, str_lit_base);
        let float_lit = format!(
            "( {} | {} | {} | {})",
            FLOAT_LIT1, FLOAT_LIT2, FLOAT_LIT3, FLOAT_LIT4
        );
        let constant = format!(
            "{} | [+-]?{} | [+-]?{} | {} | {}",
            FULL_IDENT, int_lit, float_lit, str_lit, BOOL_LIT
        );

        // Patterns
        let ptn_boollit = Regex::new(BOOL_LIT).expect("compile bool pattern");
        let ptn_const =
            Regex::new(constant.as_str()).expect("compile const1 pattern");
        let ptn_floatlit = Regex::new(float_lit.as_str()).expect("compiler float pattern");
        let ptn_full_ident = Regex::new(FULL_IDENT).expect("compile full ident pattern");
        let ptn_ident = Regex::new(IDENT).expect("compile ident pattern");
        let ptn_intlit = Regex::new(int_lit.as_str()).expect("compile ident pattern");

        Self {
            inner,
            line_buf: String::new(),
            stash: String::new(),
            in_strlit: false,
            strlit_quote: Sym::Quote1,
            // Position
            column: 0,
            line: 0,
            ptn_boollit,
            ptn_intlit,
            ptn_floatlit,
            ptn_ident,
            ptn_full_ident,
            ptn_const,
        }
    }

    // Order for the following functions are based on their priority in patterns.
    fn as_bool(s: &str) -> Token {
        if s == "true" {
            return Token::Bool(true)
        }
        Token::Bool(false)
    }

    fn as_const(s: &str) -> Token {
        Token::Constant(s.to_string())
    }

    fn as_int(s: &str) -> Token {
        let int = s.parse::<i64>().unwrap_or_else(|e| {
            warn!("decode int from '{}': {}", s, e);
            0
        });
        Token::Int(int)
    }

    fn as_float(s: &str) -> Token {
        let float = s.parse::<f64>().unwrap_or_else(|e| {
            warn!("decode float from '{}': {}", s, e);
            0.0
        });
        Token::Float(float)
    }

    fn as_ident(s: &str) -> Token { Token::Ident(s.to_string()) }

    fn as_full_ident(s: &str) -> Token { Token::FullIdent(s.to_string()) }

    fn is_discard(ch: &char) -> bool {
        matches!(ch, '\r' | '\n')
    }

    fn is_separator(ch: &char) -> bool {
        matches!(ch, ' ' | '\t')
    }

    fn is_quote(ch: &char) -> Option<Sym> {
        let sym = match Sym::from_char(ch) {
                Some(Sym::Quote1) => Sym::Quote1,
                Some(Sym::Quote2) => Sym::Quote2,
                _ => return None,
        };

        Some(sym)
    }

    pub fn pos(&self) -> Position {
        Position::new(self.column, self.line)
    }

    // Toggles strlit accumulation state. Returns true if the state change.
    fn toggle_strlit(&mut self, sym: Sym) {
        self.strlit_quote = sym;
        self.in_strlit = !self.in_strlit;
    }

    fn match_token(&self) -> Option<Token> {
        // NOTE: The order in this list will dictate the token priority. Changing the order can
        // cause some tokens to become unreachable.
        let ptns = vec![
            (&self.ptn_boollit, Self::as_bool as fn(&str) -> Token),
            (&self.ptn_intlit, Self::as_int as fn(&str) -> Token),
            (&self.ptn_floatlit, Self::as_float as fn(&str) -> Token),
            (&self.ptn_ident, Self::as_ident as fn(&str) -> Token),
            (&self.ptn_full_ident, Self::as_full_ident as fn(&str) -> Token),
            (&self.ptn_const, Self::as_const as fn(&str) -> Token),
        ];

        for (ptn, f) in ptns {
            if ptn.is_match(self.stash.as_str()) {
                return Some(f(self.stash.as_str()))
            }
        }

        None
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut counter = 0;
        let mut is_strlit: Option<Sym> = None;

        // Feed a line into the line buffer if it's empty
        if self.line_buf.is_empty() {
            self.line_buf = match self.inner.next() {
                Some(line) => line,
                // TODO add additional error handling to handle EOF gracefully
                None => {
                    return None
                }
            }
        }

        // Consume from the line buffer
        for (i, ch) in self.line_buf.chars().enumerate() {
            // Update counter that's tracking how much of the line buffer that's been consumed.
            counter = i;

            // Update position
            self.column += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            }

            // Move on to the next character if ch is a character that should not end up in the
            // stash. This rule is ignored if the lexer is handling a string literal since the
            // literal can contain otherwise unwanted characters.
            if Self::is_discard(&ch) && !self.in_strlit {
                continue;
            }

            // Check if the lexer is in the strlit state
            if let Some(sym) = Self::is_quote(&ch) {
                // Return if we got a symbol mismatch and already inside a strlit. E.g. single quote in a
                // double quote string.
                if self.in_strlit && self.strlit_quote != sym {
                    return None;
                }

                is_strlit = Some(sym);

                break;
            }

            // String literals can contain other symbols that might otherwise be part of tokens.
            // This section handles when the lexer swallows almost every thing as part of the
            // string literal.
            if !self.in_strlit {
                if let Some(sym) = Sym::from_char(&ch) {
                    return Some(Token::Symbol(sym))
                }

                // Break if we found the separator character
                if Self::is_separator(&ch) {
                    break;
                }
            }

            self.stash.push(ch);
        }

        // Remove characters consumed characters from the line buffer
        if counter != 0 {
            self.line_buf.drain(..counter);
        }

        // Return string literal as constant if the lexer detected end of string literal.
        // toggle_strlit return a bool that represent if it changed strlit state. If it change
        // the state and in_strlit is false, this means that the lexer where parsing a string
        // literal but has now stopped. In other words, the string literal is complete.
        if let Some(sym) = is_strlit {
            self.toggle_strlit(sym);

            if !self.in_strlit {
                let constant = self.stash.clone();
                self.stash.clear();
                return Some(Token::Constant(constant))
            }
        }

        match self.match_token() {
            Some(token) => {
                self.stash.clear();
                Some(token)
            },
            None => {
                warn!("failed to match '{}' at '{}:{}'", self.stash, self.line, self.column);
                None
            }
        }
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
}
