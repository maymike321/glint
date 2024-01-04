use std::iter::Peekable;
use std::str::Chars;
use crate::token::TokenType::*;

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    text: &'a str,
    iterator: Peekable<Chars<'a>>,
    start_position: usize,
    current_position: usize,
    current: Option<char>,
    line: usize
}

impl<'a> Scanner<'a> {
    pub fn new(text: &'a str)-> Scanner<'a> {
        let mut iterator = text.chars().peekable();
        let current = iterator.next();
        Scanner {
            text,
            iterator,
            start_position: 0,
            current_position: 0,
            current,
            line: 1
        }
    }

    pub fn scan(&mut self) -> Vec<Token<'a>> {
        let mut tokens: Vec<Token<'a>> = Vec::new();
        self.skip_whitespace();

        while self.current != None {
            tokens.push(self.next());
            self.skip_whitespace();
        }

        self.current_position = self.start_position;
        tokens.push(self.make_token(EOF));

        tokens
    }

    fn next(&mut self) -> Token<'a> {
        self.start_position = self.current_position;

        if self.is_alpha() { return self.identifier(); }

        if self.is_digit() { return self.number(); }

        let previous = self.current;
        self.advance();

        match previous {
            Some('(') => self.make_token(LeftParen),
            Some(')') => self.make_token(RightParen),
            Some('{') => self.make_token(LeftBrace), 
            Some('}') => self.make_token(RightBrace),
            Some(';') => self.make_token(Semicolon),
            Some(':') => self.make_token(Colon),
            Some(',') => self.make_token(Comma),
            Some('-') => self.make_token(Minus),
            Some('/') => self.make_token(Slash),
            Some('*') => self.make_token(Star),
            Some('&') => self.make_token(At),
            Some('+') => {
                match self.current {
                    Some('=') => {
                        self.advance();
                        self.make_token(PlusEqual)
                    },
                    _ => self.make_token(Plus)
                }
            }
            Some('.') => {
                match self.current {
                    Some('.') => {
                        self.advance();
                        self.make_token(DoubleDot)
                    },
                    _ => self.make_token(Dot)
                }
            }
            Some('!') => {
                match self.current {
                    Some('=') => {
                        self.advance();
                        self.make_token(BangEqual)
                    },
                    _ => self.make_token(Bang)
                }
            },
            Some('=') => {
                match self.current {
                    Some('=') => {
                        self.advance();
                        self.make_token(EqualEqual)
                    },
                    Some('>') => {
                        self.advance();
                        self.make_token(FatArrow)
                    },
                    _ => self.make_token(Equal)
                }
            },
            Some('<') => {
                match self.current {
                    Some('=') => {
                        self.advance();
                        self.make_token(LessEqual)
                    },
                    _ => self.make_token(Less)
                }
            }
            Some('>') => {
                match self.current {
                    Some('=') => {
                        self.advance();
                        self.make_token(GreaterEqual)
                    },
                    _ => self.make_token(Greater)
                }
            }
            Some('"') => self.string(),
            Some('\'') => {
                self.advance();
                if self.current != Some('\'') {
                    return self.error_token("Unterminated character.");
                }
                self.advance();
                self.make_token(Char)
            }
            _ => self.error_token("Unexpected character.")
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                    continue;
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                    continue;
                }
                Some('/') => {
                    if self.iterator.peek() == Some(&'/') {
                        while self.current != Some('\n') && self.current != None {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token::new(token_type, &self.text[self.start_position..self.current_position], self.line)
    }

    fn advance(&mut self) -> Option<char> {
        if self.current == None {
            panic!("Reached end of file unexpectedly.");
        }
        self.current = self.iterator.next();
        self.current_position += 1;
        self.current
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.is_alpha() || self.is_digit() {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token<'a> {
        while self.is_digit() { self.advance(); }
        if self.current == Some('.') { self.advance(); }
        while self.is_digit() { self.advance(); }

        return self.make_token(Number);
    }

    fn string(&mut self) -> Token<'a> {
        self.advance();

        while self.current != Some('"') && self.current != None {
            if self.current == Some('\n') { self.line += 1; }
            self.advance();
        }

        if self.current == None { return self.error_token("Unterminated string."); }

        self.advance();
        self.make_token(String)
    }

    fn is_alpha(&self) -> bool {
        match self.current {
            Some(val) => {
                val.is_ascii_alphabetic() || val == '_'
            },
            None => { false }
        }
    }

    fn is_digit(&self) -> bool {
        match self.current {
            Some(val) => {
                val.is_digit(10)
            },
            None => { false }
        }
    }

    fn identifier_type(&self) -> TokenType {
        match self.text[self.start_position..self.current_position].to_lowercase().as_str() {
            "and" => And,
            "continue" => Continue,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fn" => Fn,
            "if" => If,
            "let" => Let,
            "match" => Match,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "true" => True,
            "while" => While,
            _ => Identifier
        }
    }

    fn error_token(&mut self, message: &'a str) -> Token<'a> {
        let token = Token::new(Error, message, self.line);
        while self.current != Some('\n') && self.current != None {
            self.advance();
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_empty() {
        let program = "";
        let expected_tokens = [EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_thorough() {
        let program = "(){}&;:,-+/*. .. ! != = == => += < <= > >= \"hello\"             and continue else false for fn if let match or print return true while a b c d efg";
        let expected_tokens = [LeftParen, RightParen, LeftBrace, RightBrace, At, Semicolon, Colon, Comma, Minus, Plus, Slash, Star, Dot, DoubleDot, Bang, BangEqual, Equal, EqualEqual, FatArrow, PlusEqual, Less, LessEqual, Greater, GreaterEqual, String, And, Continue, Else, False, For, Fn, If, Let, Match, Or, Print, Return, True, While, Identifier, Identifier, Identifier, Identifier, Identifier, EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_eof_after_line_break() {
        let program = "\n";
        let expected_tokens = [EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_case() {
        let program = "AnD aNd ANd anD";
        let expected_tokens = [And, And, And, And, EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_unterminated_string() {
        let program = "\"hi";
        let expected_tokens = [Error, EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_number() {
        let program = "2934882.50349";
        let expected_tokens = [Number, EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_characters() {
        let program = "\'h\'";
        let expected_tokens = [Char, EOF];
        let tokens = test_scanner(program, &expected_tokens);
        assert_eq!(tokens[0].string, "\'h\'");
    }

    #[test]
    fn test_scanner_characters_multiple_characters() {
        let program = "\'hi\'";
        let expected_tokens = [Error, EOF];
        test_scanner(program, &expected_tokens);
    }

    #[test]
    fn test_scanner_underscore_in_name() {
        let program = "self.make_token";
        let expected_tokens = [Identifier, Dot, Identifier, EOF];
        test_scanner(program, &expected_tokens);
    }

    fn test_scanner<'a>(program: &'a str, expected_tokens: &'a[TokenType]) -> Vec<Token<'a>> {
        let mut scanner = Scanner::new(program);
        let tokens = scanner.scan();
        assert_eq!(tokens.len(), expected_tokens.len());
        let mut index = 0;
        for token in &tokens {
            assert_eq!(token.token_type, expected_tokens[index]);
            index += 1;
        }
        tokens
    }
}