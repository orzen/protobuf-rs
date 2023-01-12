use crate::segment::Segment;
use crate::token::Sym;
use std::io::{BufReader, Read};
use log::warn;

pub struct Lexer<T> {
    buf: BufReader<T>,
    line_buf: String,
    is_nested: u8,
    // Position
    column: i32,
    line: i32,
    // Closures
    fn_discard: Box<dyn Fn(char) -> bool>,
    fn_separator: Box<dyn Fn(char, u8) -> (bool, u8)>,
}

impl<T> Lexer<T> {
    pub fn new(buf: BufReader<T>) -> Self {
        let fn_discard = Box::new(|ch: char| matches!(ch, '\r' | '\n'));
        // Second return value represents the level of nesting and is unused in the default
        // case.
        let fn_separator = Box::new(|ch: char, _nested: u8| (matches!(ch, ' ' | '\t') | Sym::is_sym(&ch), 0));


        Lexer {
            buf,
            line_buf: String::new(),
            is_nested: 0,
            // Position
            column: 0,
            line: 0,
            // Closures
            fn_discard,
            fn_separator,
        }
    }
}

impl<T: Read> Lexer<T> {
    pub fn as_block(&mut self) -> Option<Segment> {
        self.break_on_eob();

        let sub = self.feed();

        self.break_on_space();

        match sub {
            Some(segment) => Some(segment),
            None => None
        }
    }

    pub fn get_pos(&self) -> (i32, i32) {
        (self.column, self.line)
    }

    pub fn set_pos(&mut self, column:i32, line:i32) {
        self.column = column;
        self.line = line
    }

    // Break functions
    pub fn break_on_eob(&mut self) {
        self.fn_separator = Box::new(|ch: char, nested: u8| match ch {
            '{' => (false, nested + 1),
            '}' => {
                let n = nested - 1;
                if n == 0 {
                    return (true, n);
                }
                (false, n)
            }
            // TODO figure out how to ignore this instead of using todo!
            _ => todo!(),
        });
    }

    pub fn break_on_space(&mut self) {
        self.fn_separator = Box::new(|ch: char, _nested: u8| (matches!(ch, ' ' | '\t'), 0));
    }
}

impl<T: Read> Lexer<T> {
    pub fn feed(&mut self) -> Option<Segment> {
        let mut stash = String::new();
        let mut counter = 0;

        // Feed a line into the line buffer if it's empty
        if self.line_buf.len() == 0 {
            match self.buf.read_to_string(&mut self.line_buf) {
                Ok(_) => (),
                // TODO add additional error handling to handle EOF gracefully
                Err(err) => {
                    warn!("failed to fill up the line buffer {}", err);
                    return None
                }
            }
        }

        // Consume from the line buffer
        for (i, ch) in self.line_buf.chars().enumerate() {
            // Update position
            self.column += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 0;
            }

            // Update counter that's tracking how much of the line buffer that's been consumed.
            counter = i;

            // Move on to the next character if ch is a character that should not end up in the
            // stash.
            if (self.fn_discard)(ch) {
                continue;
            }

            // Check if ch is a separator character
            let (found_separator, nested) = (self.fn_separator)(ch, self.is_nested);

            // Update nesting counter which keeps track of the nested state in case the
            // separator function is trying to separate a block e.g. a message containing
            // another message.
            self.is_nested = nested;

            // Break if we found the separator character
            if found_separator {
                break;
            }

            stash.push(ch);
        }

        // Remove characters consumed characters from the line buffer
        if counter != 0 {
            self.line_buf.drain(..counter);
        }

        Some(Segment::new(stash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn new() {
        let file = Cursor::new(String::from("foobar"));
        let buf = BufReader::new(file);

        Lexer::new(buf);
    }
}
