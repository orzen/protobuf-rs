use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("invalid character '{0}'")]
    InvalidChar(char),
    #[error("invalid string '{0}'")]
    InvalidStr(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers and literals
    BoolLit(bool),
    IntLit(i32),
    Constant(String),
    Ident(String),
    FullIdent(String),

    // Operators
    Assign,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrack,
    RBrack,
    LT,
    GT,
    DQuote,
    SQuote,
    Slash,
    Asterisk,

    // Keywords
    Enum,
    Import,
    Map,
    Message,
    Oneof,
    Option,
    Optional,
    Package,
    Public,
    Repeated,
    Reserved,
    Returns,
    RPC,
    Service,
    Stream,
    Syntax,
    Weak,
}

impl From<&char> for Token {
    fn from(value: &char) -> Self {
        match value {
            '=' => Token::Assign,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBrack,
            ']' => Token::RBrack,
            '<' => Token::LT,
            '>' => Token::GT,
            '"' => Token::DQuote,
            '\'' => Token::SQuote,
            _ => Token::Illegal,
        }
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            // Keywords
            "enum" => Token::Enum,
            "import" => Token::Import,
            "map" => Token::Map,
            "message" => Token::Message,
            "oneof" => Token::Oneof,
            "option" => Token::Option,
            "optional" => Token::Optional,
            "package" => Token::Package,
            "public" => Token::Public,
            "repeated" => Token::Repeated,
            "reserved" => Token::Reserved,
            "returns" => Token::Returns,
            "rpc" => Token::RPC,
            "service" => Token::Service,
            "stream" => Token::Stream,
            "syntax" => Token::Syntax,
            "weak" => Token::Weak,
            _ => Token::Illegal,
        }
    }
}
