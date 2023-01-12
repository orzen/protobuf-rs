use log::{debug, warn};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Sym {
    Invalid,
    // Basic symbols
    Equal,
    Dot,
    Comma,
    Semi,
    // Container symbols
    Curly0,
    Curly1,
    Hard0,
    Hard1,
    Quote1,
    Quote2,
    Lt,
    Gt,
    Par0,
    Par1,
}

impl Sym {
    pub fn to_char(a: &Sym) -> Option<char> {
         match a {
            // Basic symbols
            Sym::Equal  => Some('='),
            Sym::Dot    => Some('.'),
            Sym::Comma  => Some(','),
            Sym::Semi   => Some(';'),
            // Container symbols
            Sym::Curly0 => Some('{'),
            Sym::Curly1 => Some('}'),
            Sym::Hard0  => Some('['),
            Sym::Hard1  => Some(']'),
            Sym::Quote1 => Some('\''),
            Sym::Quote2 => Some('"'),
            Sym::Lt     => Some('<'),
            Sym::Gt     => Some('>'),
            Sym::Par0   => Some('('),
            Sym::Par1   => Some(')'),
            _ => None,
        }
    }

    pub fn from_char(a: &char) -> Option<Sym> {
        match a {
            // Basic symbols
            '='  => Some(Sym::Equal),
            '.'  => Some(Sym::Dot),
            ','  => Some(Sym::Comma),
            ';'  => Some(Sym::Semi),
            // C ontainer symbols
            '{'  => Some(Sym::Curly0),
            '}'  => Some(Sym::Curly1),
            '['  => Some(Sym::Hard0),
            ']'  => Some(Sym::Hard1),
            '\'' => Some(Sym::Quote1),
            '"'  => Some(Sym::Quote2),
            '<'  => Some(Sym::Lt),
            '>'  => Some(Sym::Gt),
            '('  => Some(Sym::Par0),
            ')'  => Some(Sym::Par1),
            _ => None,
        }
    }

    pub fn is_sym(ch: &char) -> bool {
        matches!(ch, '=' | '.'  | ';' | '{' | '}' | '[' | ']' | '\'' | '"' | '<' | '>' | '(' | ')')
    }
}

impl Sym {
    pub fn eq(&self, sym: &Sym) -> bool {
        self == sym
    }

    pub fn eq_ch(&self, ch: &char) -> bool {
        match Self::from_char(ch) {
            Some(sym) => self == &sym,
            None => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Keyword {
    Enum,
    Import,
    Map,
    Message,
    Oneof,
    Opt,
    Package,
    Public,
    Returns,
    Repeat,
    Rpc,
    Service,
    Stream,
    Syntax,
    Weak,
}

impl Keyword {
    pub fn from_str(a: &str) -> Option<Self> {
        match a {
            "enum"    => Some(Keyword::Enum),
            "import"  => Some(Keyword::Import),
            "map"     => Some(Keyword::Map),
            "message" => Some(Keyword::Message),
            "oneof"   => Some(Keyword::Oneof),
            "option"  => Some(Keyword::Opt),
            "package" => Some(Keyword::Package),
            "public"  => Some(Keyword::Public),
            "repeat"  => Some(Keyword::Repeat),
            "returns" => Some(Keyword::Returns),
            "rpc"     => Some(Keyword::Rpc),
            "service" => Some(Keyword::Service),
            "stream"  => Some(Keyword::Stream),
            "syntax"  => Some(Keyword::Syntax),
            "weak"    => Some(Keyword::Weak),
            _         => None,
        }
    }

    pub fn to_string(a: &Keyword) -> String {
        match a {
            Keyword::Enum    => String::from("enum"),
            Keyword::Import  => String::from("import"),
            Keyword::Map     => String::from("map"),
            Keyword::Message => String::from("message"),
            Keyword::Oneof   => String::from("oneof"),
            Keyword::Opt     => String::from("option"),
            Keyword::Package => String::from("package"),
            Keyword::Public  => String::from("public"),
            Keyword::Repeat  => String::from("repeat"),
            Keyword::Returns => String::from("returns"),
            Keyword::Rpc     => String::from("rpc"),
            Keyword::Service => String::from("service"),
            Keyword::Stream  => String::from("stream"),
            Keyword::Syntax  => String::from("syntax"),
            Keyword::Weak    => String::from("weak"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Bool(bool),
    Constant(String),
    FullIdent(String),
    Keyword(Keyword),
    Ident(String),
    Int(i64),
    Float(f64),
    Symbol(Sym),
}

impl Token {
    pub fn is_nested(nested: &i32, token: &Self) -> i32 {
        let mut n = *nested;

        // Check if block end was found or another nesting level
        if let Self::Symbol(sym) = token {
            match sym {
                Sym::Curly0 => n += 1,
                Sym::Curly1 => n -= 1,
                _ => (),
            }
        }

        n
    }

    pub fn as_line(tokens: &mut VecDeque<Self>) -> VecDeque<Self> {
        let mut line_tokens: VecDeque<Self> = VecDeque::new();

        // Collect all tokens for related to one service entry
        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Symbol(Sym::Semi) => {
                    line_tokens.push_back(token);
                    break;
                }
                _ => line_tokens.push_back(token),
            }
        }

        line_tokens
    }


    pub fn as_block(tokens: &mut VecDeque<Self>) -> VecDeque<Self> {
        let mut nested = 0;
        let mut block_tokens: VecDeque<Self> = VecDeque::new();

        if tokens.is_empty() {
            return block_tokens;
        }

        let block_name = tokens.pop_front().unwrap_or_else(|| {
            warn!("expected block name, got none");
            // Use invalid token that will cause crash later on
            Token::Bool(false)
        });

        while let Some(token) = tokens.pop_front() {
            nested = Self::is_nested(&nested, &token);

            block_tokens.push_back(token);

            if nested == 0 {
                break;
            }
        }

        block_tokens.push_front(block_name);

        block_tokens
    }

    pub fn constant(&self) -> Option<String> {
        if let Token::Constant(val) = self {
            return Some(val.to_string());
        }

        debug!("unpack constant, got {:?}", self);
        None
    }

    pub fn ident(&self) -> Option<String> {
        match self {
            Token::Ident(val) => Some(val.clone()),
            other => {
                debug!("unpack ident, got {:?}", other);
                None
            }
        }
    }

    pub fn full_ident(&self) -> Option<String> {
        match self {
            Token::Ident(val) => Some(val.clone()),
            Token::FullIdent(val) => Some(val.clone()),
            other => {
                debug!("unpack full ident, got {:?}", other);
                None
            }
        }
    }

    pub fn int(&self) -> Option<i64> {
        match self {
            Token::Int(val) => Some(*val),
            other => {
                debug!("unpack int, got {:?}", other);
                None
            }
        }
    }

    pub fn keyword(&self) -> Option<Keyword> {
        match self {
            Token::Keyword(val) => Some(*val),
            other => {
                debug!("unpack keyword, got {:?}", other);
                None
            }
        }
    }

    pub fn sym(&self) -> Option<Sym> {
        match self {
            Token::Symbol(val) => Some(*val),
            other => {
                debug!("unpack symbol, got {:?}", other);
                None
            }
        }
    }

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn sym_mapping() {
        let syms = [
            Sym::Equal,
            Sym::Dot,
            Sym::Comma,
            Sym::Semi,
            Sym::Curly0,
            Sym::Curly1,
            Sym::Hard0,
            Sym::Hard1,
            Sym::Lt,
            Sym::Gt,
            Sym::Par0,
            Sym::Par1,
        ];

        for sym in syms {
            let ch = match Sym::to_char(&sym) {
                Some(s) => s,
                None => {
                    assert!(true, "convert sym {:?} to char", sym);
                    '\0'
                }
            };

            match Sym::from_char(&ch) {
                Some(s) => assert_eq!(sym, s, "symbols do not match"),
                None => assert!(true, "expected sym got none"),
            }
        }
    }

    #[test]
    fn keyword_mapping() {
        let keywords = [
            Keyword::Enum,
            Keyword::Import,
            Keyword::Map,
            Keyword::Message,
            Keyword::Oneof,
            Keyword::Opt,
            Keyword::Package,
            Keyword::Public,
            Keyword::Returns,
            Keyword::Repeat,
            Keyword::Rpc,
            Keyword::Service,
            Keyword::Stream,
            Keyword::Syntax,
            Keyword::Weak,
        ];

        for keyword in keywords {
            let kw = Keyword::to_string(&keyword);

            match Keyword::from_str(kw.as_str()) {
                Some(kw) => assert_eq!(kw, keyword, "keyword do not match"),
                None => assert!(true, "expected keyword, got none"),
            }
        }
    }

}
