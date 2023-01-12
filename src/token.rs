#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
    Quote,
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
            '=' => Some(Sym::Equal),
            '.' => Some(Sym::Dot),
            ',' => Some(Sym::Comma),
            ';' => Some(Sym::Semi),
            // Container symbols
            '{' => Some(Sym::Curly0),
            '}' => Some(Sym::Curly1),
            '[' => Some(Sym::Hard0),
            ']' => Some(Sym::Hard1),
            '<' => Some(Sym::Lt),
            '>' => Some(Sym::Gt),
            '(' => Some(Sym::Par0),
            ')' => Some(Sym::Par1),
            _ => None,
        }
    }

    pub fn is_sym(ch: &char) -> bool {
        matches!(ch, '=' | '.' | ')' | ';' | '{' | '}' | '[' | ']' | '<' | '>' | '(' | ')')
    }
}

impl Sym {
    pub fn eq(&self, sym: &Sym) -> bool {
        self == sym
    }

    pub fn eq_ch(&self, ch: &char) -> bool {
        match Self::from_char(ch) {
            Some(sym) => self == &sym,
            None => return false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    RPC,
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
            "rpc"     => Some(Keyword::RPC),
            "service" => Some(Keyword::Service),
            "stream"  => Some(Keyword::Stream),
            "syntax"  => Some(Keyword::Syntax),
            "weak"    => Some(Keyword::Weak),
            _         => None,
        }
    }

    pub fn to_string(a: Keyword) -> Option<String> {
        match a {
            Keyword::Enum    => Some(String::from("enum")),
            Keyword::Import  => Some(String::from("import")),
            Keyword::Map     => Some(String::from("map")),
            Keyword::Message => Some(String::from("message")),
            Keyword::Oneof   => Some(String::from("oneof")),
            Keyword::Opt     => Some(String::from("option")),
            Keyword::Package => Some(String::from("package")),
            Keyword::Public  => Some(String::from("public")),
            Keyword::Repeat  => Some(String::from("repeat")),
            Keyword::Returns => Some(String::from("returns")),
            Keyword::RPC     => Some(String::from("rpc")),
            Keyword::Service => Some(String::from("service")),
            Keyword::Stream  => Some(String::from("stream")),
            Keyword::Syntax  => Some(String::from("syntax")),
            Keyword::Weak    => Some(String::from("weak")),
            _                => None,
        }
    }
}


// TODO remove if unused
#[derive(Debug, PartialEq)]
pub enum Token {
    Bool(bool),
    Constant(String),
    FullIdent(String),
    Keyword(Keyword),
    Ident(String),
    Number(i32),
    Symbol(Sym),
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
            Keyword::RPC,
            Keyword::Service,
            Keyword::Stream,
            Keyword::Syntax,
            Keyword::Weak,
        ];

        for keyword in keywords {
            let string = match Keyword::to_string(keyword) {
                Some(kw) => kw,
                None => {
                    assert!(true, "expected keyword string, got none");
                    String::new()
                }
            };

            match Keyword::from_str(string.as_str()) {
                Some(kw) => assert_eq!(kw, keyword, "keyword do not match"),
                None => assert!(true, "expected keyword, got none"),
            }
        }
    }

}
