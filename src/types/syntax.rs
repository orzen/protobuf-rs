use log::debug;
use std::fmt::Display;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Type;
use crate::token_stream::TokenStream;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Syntax {
    pub value: String,
}

impl Syntax {
    pub fn new(value: String) -> Self {
        Syntax { value }
    }
}

impl TryFrom<TokenStream> for Syntax {
    type Error = ParserError;

    // Parsing backwards
    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("syntax({:?})", &tokens);

        tokens.next_eq(Type::Semicolon, "syntax line ending(';')")?;
        let value = tokens.constant_as_string("syntax value")?;
        tokens.next_eq(Type::Assign, "syntax assignment('=')")?;
        tokens.next_eq(Type::Syntax, "syntax identifier")?;

        if !value.eq("\"proto2\"") && !value.eq("\"proto3\"") {
            return Err(ParserError::Syntax(
                "syntax value to be '\"proto2\"' or '\"proto3\"'".to_string(),
                format!("{:?}", value),
            ));
        }

        Ok(Self::new(value))
    }
}

impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        indent(f)?;
        write!(f, "syntax = {};", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::*;

    #[test]
    fn from_ok() {
        let mut ts = TokenStream::new();

        let _ = &[
            Type::Syntax,
            Type::Assign,
            Type::Constant("\"proto3\"".to_string()),
            Type::Semicolon,
        ]
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>()
        .iter()
        .for_each(|t| ts.push(t.clone()));

        let res = Syntax::try_from(ts);
        assert!(res.is_ok(), "syntax parse error {:?}", res);

        let syntax = res.unwrap();
        assert_eq!(format!("{syntax}"), "syntax = \"proto3\";");
    }
}
