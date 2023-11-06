use std::{fs::File, io::BufReader};
use error::ProtobufError;
use parser::Parser;

use crate::buffer::Buffer;
use crate::types::proto::Proto;

mod buffer;
mod error;
mod indent;
mod lexer;
mod parser;
mod position;
mod token;
mod token_stream;
mod types;

pub fn load_file(filename: &str) -> Result<Proto, ProtobufError> {
    let file = File::open(filename).expect("open file");
    let inner = BufReader::new(file);
    let buf = Buffer::new(inner);

    let parser = Parser::new();

    Ok(parser.load(buf)?)
}

// TODO fix me
// #[cfg(test)]
// mod tests {
//     use crate::load_file;
// 
//     #[test]
//     fn example_file() {
//         let p = load_file("example.proto");
//         assert!(p.is_ok(), "failed to load file");
//     }
// 
//     #[test]
//     fn example_to_string() {
//         let p = load_file("example.proto");
//         assert!(p.is_ok(), "failed to load string");
//     }
// }
