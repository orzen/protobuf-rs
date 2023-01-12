use regex::Regex;

use crate::token::Keyword;
use crate::token::Sym;

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

// Constant
// FullIdent
// - intLit
// + intLit
// - floatLit
// + floatLit
// strLit
// boolLit

pub struct Segment {
    inner: String,
    // Patterns
    ptn_const: Regex,
    ptn_ident: Regex,
    ptn_full_ident: Regex,
    ptn_intlit: Regex,
}

impl Segment {
    pub fn new(inner: String) -> Self {
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
        let ptn_const =
            Regex::new(format!("{}", constant).as_str()).expect("compile const1 pattern");
        Regex::new(format!("'{}'", IDENT).as_str()).expect("compile const2 pattern");
        let ptn_ident = Regex::new(IDENT).expect("compile ident pattern");
        let ptn_full_ident = Regex::new(FULL_IDENT).expect("compile full ident pattern");
        let ptn_intlit = Regex::new(int_lit.as_str()).expect("compile ident pattern");

        Segment {
            inner,
            // Patterns
            ptn_const,
            ptn_ident,
            ptn_full_ident,
            ptn_intlit,
        }
    }
}

// Lexical elements
impl Segment {
    pub fn into_inner(&self) -> String {
        self.inner.clone()
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self.inner.as_str() {
            "true" => {
                return Some(true);
            }
            "false" => {
                return Some(false);
            }
            _ => None,
        }
    }

    pub fn as_constant(&self) -> Option<String> {
        if self.ptn_const.is_match(self.inner.as_str()) {
            return Some(self.inner.clone());
        }

        None
    }

    pub fn as_full_ident(&self) -> Option<String> {
        if self.ptn_full_ident.is_match(self.inner.as_str()) {
            return Some(self.inner.clone());
        }

        None
    }

    pub fn as_ident(&self) -> Option<String> {
        if self.ptn_ident.is_match(self.inner.as_str()) {
            return Some(self.inner.clone());
        }

        None
    }

    pub fn as_keyword(&self) -> Option<Keyword> {
        match Keyword::from_str(self.inner.as_str()) {
            Some(kw) => Some(kw),
            None => None,
        }
    }

    pub fn as_number(&self) -> Option<i64> {
        if self.ptn_intlit.is_match(&self.inner.as_str()) {
            let value = self.inner.parse::<i64>().expect("convert to number");

            Some(value);
        }

        None
    }

    pub fn as_sym(& self) -> Option<Sym> {
        let sym = self.inner.clone().as_bytes()[0] as char;

        Sym::from_char(&sym)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Sym;
    use crate::token::Token;

    #[test]
    fn bool() {
        // Verify true value
        let true_seg = Segment::new(String::from("true"));
        let true_bool = match true_seg.as_bool() {
            Some(v) => v,
            None => false,
        };
        assert_eq!(true_bool, true, "expected value true");

        let false_seg = Segment::new(String::from("false"));
        let false_bool = match false_seg.as_bool() {
            Some(v) => v,
            None => true,
        };
        assert_eq!(false_bool, false, "expected value false");

        let invalid_bool = Segment::new(String::from("foo")).as_bool();
        assert_eq!(invalid_bool, None, "expected value None");
    }

    #[test]
    fn constant_float() {
        let float_lits = vec![
            "123.",
            "123.123",
            "123.e123",
            "123.e+123",
            "123.e-123",
            "123.E123",
            "123.E+123",
            "123.E-123",
            "123.123e123",
            "123.123e+123",
            "123.123e-123",
            "123.123E123",
            "123.123E+123",
            "123.123E-123",
            "123e123",
            "123e+123",
            "123e-123",
            "123E123",
            "123E+123",
            "123E-123",
            ".123e123",
            ".123e+123",
            ".123e-123",
            ".123E123",
            ".123E+123",
            ".123E-123",
            "inf",
            "nan",
        ];

        for s in float_lits {
            let seg = Segment::new(String::from(s));

            match seg.as_constant() {
                Some(value) => assert_eq!(s, value.as_str(), "expected {}, got {}", s, value),
                None => assert!(true, "expected constant, got none")
            }
        }
    }

    #[test]
    fn constant_int() {
        let int_lits = vec![
            "123",
            "007",
            "0123",
            "0x1FF",
            "0X1FF",
        ];

        for s in int_lits {
            let seg = Segment::new(String::from(s));

            match seg.as_constant() {
                Some(value) => assert_eq!(s, value.as_str(), "expected {}, got {}", s, value),
                None => assert!(true, "expected constant, got none")
            }
        }
    }

    #[test]
    /// Testing string literals and full Identifier
    fn constant_str() {
        let strs = vec![
            // Hex
            "\"\\xFf\"",
            "'\\xFf'",
            "\"\\XFf\"",
            "'\\XFf'",
            // Oct
            "\"\\012\"",
            "'\\012'",
            // Char
            "\"\\a\\b\\f\\n\\r\\t\\v\"",
            "'\\a\\b\\f\\n\\r\\t\\v'",
            "\"abcdefghijklmnopqrstuvxyz0123456789_.\"",
            "'abcdefghijklmnopqrstuvxyz0123456789_.'",
        ];

        for s in strs {
            let seg = Segment::new(String::from(s));

            match seg.as_constant() {
                Some(value) => assert_eq!(s, value.as_str(), "expected {}, got {}", s, value),
                None => assert!(true, "expected constant, got none")
            }
        }
    }

    #[test]
    fn sym() {
        // Check with valid characters
        let valid_input: Vec<char> =
            vec!['=', '.', ',', ';', '{', '}', '[', ']', '<', '>', '(', ')'];

        for ch in valid_input {
            let mut inner = String::new();
            inner.push(ch);

            let sym = match Segment::new(inner).as_sym() {
                Some(s) => s,
                None => {
                    assert!(true, "expected sym, got none");
                    Sym::Equal
                }
            };

            assert_eq!(Sym::to_char(&sym).unwrap(), ch, "expected symbol {}", ch)
        }

        // Check with invalid characters
        let invalid_input: Vec<char> = vec!['@', '!', '#'];

        for ch in invalid_input {
            let mut inner = String::new();
            inner.push(ch);

            let sym = Segment::new(inner).as_sym();
            assert_eq!(sym, None, "expected None, got value");
        }
    }

    #[test]
    fn full_ident() {
        let inner = String::from("abcdefghijklmnopqrstuvxyz0123456789_.");

         match Segment::new(inner.clone()).as_full_ident() {
            Some(value) => assert_eq!(inner, value, "expected {}, got {}", inner, value),
            None => assert!(true, "expected sym, got none"),
        };
    }

    #[test]
    fn ident() {
        let inner = String::from("abcdefghijklmnopqrstuvxyz0123456789_");

         match Segment::new(inner.clone()).as_ident() {
            Some(value) => assert_eq!(inner, value, "expected {}, got {}", inner, value),
            None => assert!(true, "expected sym, got none"),
        };
    }

    #[test]
    fn keyword() {
        let words = vec![
            "enum",
            "import",
            "map",
            "message",
            "oneof",
            "option",
            "package",
            "public",
            "repeat",
            "returns",
            "rpc",
            "service",
            "stream",
            "syntax",
            "weak",
        ];

        for word in words {
            let inner = String::from(word);

             match Segment::new(inner.clone()).as_keyword() {
                Some(value) => assert_eq!(inner, Keyword::to_string(value).unwrap(), "expected {}, got {:?}", inner, value),
                None => assert!(true, "expected sym, got none"),
            };
        }
    }

    #[test]
    fn number() {
        let valid_numbers = vec![
            ("1234567890", 1234567890),
            ("9876543210", 987654321),
            ("1", 1),
        ];

        for (as_str, as_i32) in valid_numbers {
            let inner = String::from(as_str);

             match Segment::new(inner.clone()).as_number() {
                Some(value) => assert_eq!(value, as_i32, "expected {}, got {:?}", as_i32, value),
                None => assert!(true, "expected sym, got none"),
            };
        }
    }
}
