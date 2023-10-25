use std::fmt::Display;
use log::debug;

use crate::error::ParserError;
use crate::indent::indent;
use crate::token::Token;
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

    fn try_from(mut tokens: TokenStream) -> Result<Self, Self::Error> {
        debug!("syntax({:?})", &tokens);

        tokens.next_is(Token::Semicolon, "syntax line ending(';')")?;
        let value = tokens.next_is_constant("syntax value")?;
        tokens.next_is(Token::Assign, "syntax assignment('=')")?;
        tokens.next_is(Token::Syntax, "syntax identifier")?;

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


//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn from_tokens_missing_ident() {
//        let mut ts = TokenStream::new();
//        ts.push(Token::Assign);
//        ts.push(Token::DQuote);
//        ts.push(Token::Ident("proto3"));
//        ts.push(Token::DQuote);
//        ts.push(Token::Semicolon);
//
//        let res = Syntax::try_from(&ts);
//
//        assert!(res.is_err());
//    }
//}
