use std::fmt::{Formatter, Result};

pub fn level<'a>(f: &Formatter<'a>) -> usize {
    if let Some(width) = f.width() {
        width
    } else {
        0
    }
}

pub fn indent<'a>(f: &mut Formatter<'a>) -> Result {
    if let Some(width) = f.width() {
        write!(f, "{:indent$}", "\t", indent=width)
    } else {
        Ok(())
    }
}
