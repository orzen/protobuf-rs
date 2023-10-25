use std::str::Chars;
use std::io::{BufReader, Read};

use log::warn;

use crate::error::BufferError;

pub struct Buffer<T> {
    inner: BufReader<T>,
    buf: String,
}

impl<T: Read> Buffer<T> {
    pub fn new(inner: BufReader<T>) -> Self {
        Self {
            inner,
            buf: String::new(),
        }
    }

    pub fn read(&mut self) -> Result<usize, BufferError> {
        self.buf.clear();
        let size = self.inner.read_to_string(&mut self.buf)?;
        if size == 0 {
            return Err(BufferError::EOF);
        }

        return Ok(size);
    }

    pub fn next(&mut self) -> Option<Chars<'_>> {
        match self.read() {
            Ok(size) => {
                if size == 0 {
                    None
                } else {
                    Some(self.buf.chars())
                }
            }
            Err(e) => {
                warn!("buffer read error: {e}");
                None
            }
        }
    }
}

