#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace, At, Comma, Dot, DoubleDot,
    Minus, Plus, Colon, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    FatArrow, PlusEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, Char, String, Number,

    And, Continue, Else, False, Fn, For, If, Let, Match,
    Or, Print, Return, True, While,

    Error, EOF
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub string: &'a str,
    pub line: usize
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, string: &'a str, line: usize) -> Token<'a> {
        Token {
            token_type,
            string,
            line
        }
    }
}