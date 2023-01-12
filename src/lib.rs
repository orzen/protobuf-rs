use std::{fs::File, io::BufReader};
use parser::Parser;

mod lexer;
mod parser;
mod segment;
mod token;
mod types;

pub fn parse<E>(filename: &str) -> Result<(), E> {
    let file = File::open(filename).expect("open file");
    let buf = BufReader::new(file);

    Parser::new(buf);

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
