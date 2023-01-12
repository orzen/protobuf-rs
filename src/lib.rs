use std::{fs::File, io::BufReader};

use crate::buffer::Buffer;
use crate::parser::Parser;
use crate::types::proto::Proto;

mod buffer;
mod lexer;
mod macros;
mod parser;
mod position;
mod token;
mod types;

pub fn load_file(filename: &str) -> Option<Proto>
{
    let file = File::open(filename).expect("open file");
    let inner = BufReader::new(file);
    let buf = Buffer::new(inner);
    let mut parser: Parser<_> = Parser::new(buf);

    parser.as_proto()
}

#[cfg(test)]
mod tests {
    use crate::load_file;


    #[test]
    fn example_file() {
        let p = load_file("example.proto");
        assert_eq!(p, None, "empty proto");
    }
}
