use std::{fmt::Display, ops::Deref};

use crate::{
    error::ParserError,
    position::{Point, Position},
};

const OPTION_NAME_URL: &str = "https://protobuf.com/docs/language-spec#option-names";
const IDENT_URL: &str = "https://protobuf.dev/reference/protobuf/proto3-spec/#identifiers";
const CONST_URL: &str = "https://protobuf.dev/reference/protobuf/proto3-spec/#constant";

#[derive(Clone, Debug)]
pub struct Token {
    typ: Type,
    pos: Position,
}

impl Token {
    pub fn new(typ: Type, pos: Position) -> Self {
        Self { typ, pos }
    }

    pub fn illegal() -> Self {
        Self {
            typ: Type::Illegal,
            pos: Position::default(),
        }
    }

    pub fn bool(v: bool) -> Self {
        Self {
            typ: Type::from(v),
            pos: Position::default(),
        }
    }

    pub fn i32(v: i32) -> Self {
        Self {
            typ: Type::IntLit(v),
            pos: Position::default(),
        }
    }

    pub fn constant(v: String) -> Self {
        Self {
            typ: Type::Constant(v),
            pos: Position::default(),
        }
    }

    pub fn ident(v: String) -> Self {
        Self {
            typ: Type::Ident(v),
            pos: Position::default(),
        }
    }

    pub fn full_ident(v: String) -> Self {
        Self {
            typ: Type::FullIdent(v),
            pos: Position::default(),
        }
    }

    pub fn option_name(v: String) -> Self {
        Self {
            typ: Type::OptionName(v),
            pos: Position::default(),
        }
    }

    pub fn as_const(&self) -> Result<String, ParserError> {
        let s = match &self.typ {
            Type::Ident(v) => v,
            Type::FullIdent(v) => v,
            Type::Constant(v) => v,
            invalid => {
                return Err(ParserError::Syntax(
                    format!("constant, check {CONST_URL} for more info"),
                    format!("{invalid}"),
                ))
            }
        };

        Ok(s.to_string())
    }

    pub fn as_ident(&self) -> Result<String, ParserError> {
        let s = match &self.typ {
            Type::Ident(v) => v,
            invalid => {
                return Err(ParserError::Syntax(
                    format!("Ident, check {IDENT_URL} for more info"),
                    format!("{invalid}"),
                ))
            }
        };

        Ok(s.to_string())
    }

    pub fn as_full_ident(&self) -> Result<String, ParserError> {
        let s = match &self.typ {
            Type::Ident(v) => v,
            Type::FullIdent(v) => v,
            invalid => {
                return Err(ParserError::Syntax(
                    format!("FullIdent, check {IDENT_URL} for more info"),
                    format!("{invalid}"),
                ))
            }
        };

        Ok(s.to_string())
    }

    pub fn as_option_name(&self) -> Result<String, ParserError> {
        let s = match &self.typ {
            Type::Ident(v) => v,
            Type::FullIdent(v) => v,
            Type::OptionName(v) => v,
            invalid => {
                return Err(ParserError::Syntax(
                    format!("OptionName, check {OPTION_NAME_URL} for more info"),
                    format!("{invalid}"),
                ))
            }
        };

        Ok(s.to_string())
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn set_position(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn typ(&self) -> &Type {
        &self.typ
    }
}

impl Deref for Token {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        self.typ()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({}, {})", self.typ, self.pos)
    }
}

impl From<Type> for Token {
    fn from(value: Type) -> Self {
        Self {
            typ: value,
            pos: Position::from(Point::new(0, 0)),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
    }
}

// Type

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Illegal,
    EOF,

    // Identifiers and literals
    BoolLit(bool),
    IntLit(i32),
    Constant(String),
    Ident(String),
    FullIdent(String),
    OptionName(String),

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

impl From<bool> for Type {
    fn from(value: bool) -> Self {
        Self::BoolLit(value)
    }
}

impl From<i32> for Type {
    fn from(value: i32) -> Self {
        Self::IntLit(value)
    }
}

impl From<&char> for Type {
    fn from(value: &char) -> Self {
        match value {
            '=' => Type::Assign,
            ',' => Type::Comma,
            ';' => Type::Semicolon,
            '*' => Type::Asterisk,
            '/' => Type::Slash,
            '(' => Type::LParen,
            ')' => Type::RParen,
            '{' => Type::LBrace,
            '}' => Type::RBrace,
            '[' => Type::LBrack,
            ']' => Type::RBrack,
            '<' => Type::LT,
            '>' => Type::GT,
            _ => Type::Illegal,
        }
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        match value {
            // Keywords
            "enum" => Type::Enum,
            "import" => Type::Import,
            "map" => Type::Map,
            "message" => Type::Message,
            "oneof" => Type::Oneof,
            "option" => Type::Option,
            "optional" => Type::Optional,
            "package" => Type::Package,
            "public" => Type::Public,
            "repeated" => Type::Repeated,
            "reserved" => Type::Reserved,
            "returns" => Type::Returns,
            "rpc" => Type::RPC,
            "service" => Type::Service,
            "stream" => Type::Stream,
            "syntax" => Type::Syntax,
            "weak" => Type::Weak,
            _ => Type::Illegal,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Illegal => "".to_string(),
            Self::EOF => "".to_string(),

            // Identifiers and literals
            Self::BoolLit(v) => format!("{v}"),
            Self::IntLit(v) => format!("{v}"),
            Self::Constant(v) => format!("{v}"),
            Self::Ident(v) => format!("{v}"),
            Self::FullIdent(v) => format!("{v}"),
            Self::OptionName(v) => format!("{v}"),

            // Operators
            Self::Assign => "=".to_string(),

            // Delimiters
            Self::Comma => ",".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::LParen => "(".to_string(),
            Self::RParen => ")".to_string(),
            Self::LBrace => "{".to_string(),
            Self::RBrace => "}".to_string(),
            Self::LBrack => "[".to_string(),
            Self::RBrack => "]".to_string(),
            Self::LT => "<".to_string(),
            Self::GT => ">".to_string(),
            Self::Slash => "/".to_string(),
            Self::Asterisk => "*".to_string(),

            // Keywords
            Self::Enum => "enum".to_string(),
            Self::Import => "import".to_string(),
            Self::Map => "map".to_string(),
            Self::Message => "message".to_string(),
            Self::Oneof => "oneof".to_string(),
            Self::Option => "option".to_string(),
            Self::Optional => "optional".to_string(),
            Self::Package => "package".to_string(),
            Self::Public => "public".to_string(),
            Self::Repeated => "repeated".to_string(),
            Self::Reserved => "reserved".to_string(),
            Self::Returns => "returns".to_string(),
            Self::RPC => "rpc".to_string(),
            Self::Service => "service".to_string(),
            Self::Stream => "stream".to_string(),
            Self::Syntax => "syntax".to_string(),
            Self::Weak => "weak".to_string(),
        };

        write!(f, "{s}")
    }
}
