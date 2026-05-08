#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Const,
    Identifier,
    Number,

    Colon,
    Equal,
    Plus,
    Semicolon,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}