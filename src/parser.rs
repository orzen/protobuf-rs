use std::io::Read;

use crate::buffer::Buffer;
use crate::error::ParserError;
use crate::lexer::Lexer;
use crate::types::proto::Proto;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load<T>(&self, buf: Buffer<T>) -> Result<Proto, ParserError>
    where
        T: Read,
    {
        let mut lexer = Lexer::new()?;
        let tokens = lexer.token_stream(buf)?;

        Proto::try_from(tokens)
    }
}


//#[cfg(test)]
//mod tests {
//    use std::fs::File;
//    use std::io::BufReader;
//
//    use super::*;
//
//    #[test]
//    fn new_from_file() {
//    }
//}
