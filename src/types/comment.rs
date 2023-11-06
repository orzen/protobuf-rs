use std::fmt::Display;

use crate::{
    indent::{indent, level},
    token_stream::TokenStream,
};

pub trait Comment {}

// Line comment

#[derive(Clone, Debug, PartialEq)]
pub struct LineComment {
    inner: String,
}

impl LineComment {
    pub fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl From<TokenStream> for LineComment {
    fn from(tokens: TokenStream) -> Self {
        let inner: String = tokens
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join("");

        Self::new(inner)
    }
}

impl Display for LineComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;
        writeln!(f, "//{}", &self.inner)
    }
}

// Block comment

#[derive(Clone, Debug, PartialEq)]
pub struct BlockComment {
    inner: Vec<String>,
}

impl BlockComment {
    pub fn new(inner: Vec<String>) -> Self {
        Self { inner }
    }
}

impl From<TokenStream> for BlockComment {
    fn from(mut tokens: TokenStream) -> Self {
        // Discard opening and closing tokens
        let _close_slash = tokens.pop();
        let _close_aster = tokens.pop();
        tokens.reverse();
        let _open_slash = tokens.pop();
        let open_aster = tokens.pop().unwrap();

        let mut lines = vec![];
        let mut line = vec![];
        let mut line_num = open_aster.position().line();

        while let Some(value) = tokens.pop() {
            if value.position().line() != line_num {
                lines.push(line.join(" "));
                line.clear();
                line_num = value.position().line();
            }

            line.push(format!("{}", value.typ()));
        }

        Self::new(lines)
    }
}

impl Display for BlockComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = level(f);

        writeln!(f, "{:indent$}", "/*", indent = i + 1)?;

        for line in &self.inner {
            writeln!(f, "{:indent$}", line, indent = i + 1)?;
        }

        writeln!(f, "{:indent$}", "*/", indent = i + 1)
    }
}
