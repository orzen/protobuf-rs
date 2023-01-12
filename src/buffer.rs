use std::io::{BufReader, Read};
use log::info;

pub struct Buffer<T> {
    inner: BufReader<T>,
}

impl<T: Read> Buffer<T> {
    pub fn new(inner: BufReader<T>) -> Self {
        Self {
            inner,
        }
    }

    pub fn next(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.inner.read_to_string(&mut line) {
            Ok(_) => Some(line),
            Err(e) => {
                info!("buffer read line: {}", e);
                None
            }
        }
    }
}
