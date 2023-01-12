use crate::token::Keyword;

#[derive(Clone, Debug, PartialEq)]
pub struct Import {
    pub value: String,
    pub scope: Option<Keyword>,
}

impl Import {
    pub fn new(value: String, scope: Option<Keyword>) -> Self {
        Import { value, scope }
    }
}
