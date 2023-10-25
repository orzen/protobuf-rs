use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BufferError {
    #[error("convert byte {0} to character")]
    Convert(u8),
    #[error("end of file")]
    EOF,
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("buffer error: {0}")]
    Buffer(#[from] BufferError),
    #[error("invalid token: '{0}'")]
    Invalid(String),
    //#[error(transparent)]
    //Parse(#[from] ParseIntError),
    #[error("{0}")]
    Generic(String), // Rename to Tokenize
}

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("lexer error: {0}")]
    Lexer(#[from] LexerError),
    #[error("syntax error: expected '{0}', got '{1}'")]
    Syntax(String, String),
}

#[derive(Debug, Error)]
pub enum ProtobufError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Lexer(#[from] LexerError),
    #[error(transparent)]
    Parser(#[from] ParserError),
}

