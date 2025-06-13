use crate::rlox::RLox;
use crate::token::{Literal, Token, TokenKind, KEYWORD_MAP};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<String>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: "",
            literal: Literal::None,
            line: self.line,
        });
        dbg!(&self.tokens);
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(
                TokenKind::LeftParen,
                Literal::String(&self.source[self.start..self.current]),
            ),
            ')' => self.add_token(
                TokenKind::RightParen,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '{' => self.add_token(
                TokenKind::LeftBrace,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '}' => self.add_token(
                TokenKind::RightBrace,
                Literal::String(&self.source[self.start..self.current]),
            ),
            ',' => self.add_token(
                TokenKind::Comma,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '.' => self.add_token(
                TokenKind::Dot,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '-' => self.add_token(
                TokenKind::Minus,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '+' => self.add_token(
                TokenKind::Plus,
                Literal::String(&self.source[self.start..self.current]),
            ),
            ';' => self.add_token(
                TokenKind::Semicolon,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '*' => self.add_token(
                TokenKind::Star,
                Literal::String(&self.source[self.start..self.current]),
            ),
            '!' => {
                let token_kind = if self.char_match('=') {
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(
                    token_kind,
                    Literal::String(&self.source[self.start..self.current]),
                );
            }
            '=' => {
                let token_kind = if self.char_match('=') {
                    self.advance();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(
                    token_kind,
                    Literal::String(&self.source[self.start..self.current]),
                )
            }
            '<' => {
                let token_kind = if self.char_match('=') {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(
                    token_kind,
                    Literal::String(&self.source[self.start..self.current]),
                )
            }
            '>' => {
                let token_kind = if self.char_match('=') {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(
                    token_kind,
                    Literal::String(&self.source[self.start..self.current]),
                );
            }
            '/' => {
                if self.char_match('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(
                        TokenKind::Slash,
                        Literal::String(&self.source[self.start..self.current]),
                    );
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if Self::is_digit(c as u8) {
                    self.number();
                } else if Self::is_alpha(c as u8) {
                    self.identifier();
                } else {
                    const ERROR: &str = "Unexpected character";
                    self.errors.push(format!("{} at line {}", ERROR, self.line));
                    RLox::error(self.line as u32, ERROR);
                }
            }
        }
    }

    /// Advances the current position in the source and returns the next character.
    ///
    /// This method increments the `current` index and retrieves the next character
    /// from the `source` starting at the updated position. If the `source` is empty
    /// this method will panic.
    fn advance(&mut self) -> char {
        let old = self.current;
        self.current += 1;
        self.source[old..].chars().next().unwrap()
    }

    fn add_token(&mut self, kind: TokenKind, literal: Literal<'a>) {
        self.tokens.push(Token {
            kind,
            literal,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let c = self.source[self.current..]
            .chars()
            .next()
            .expect("error getting next char");

        c == expected
    }

    /// This method returns the next character in the source without consuming it.
    ///
    /// ```
    ///   1        2
    ///   |        |
    /// current  peek (returns this)
    /// ```
    fn peek(&mut self) -> char {
        self.source[self.current..].chars().next().unwrap()
    }

    /// This method returns the character after the next in the source without consuming it.
    ///
    /// ```
    ///   1        2        3
    ///   |        |        |
    /// current   peek   peek_next(returns this)
    /// ```
    fn peek_next(&mut self) -> char {
        self.source[self.current..].chars().nth(1).unwrap()
    }

    fn string(&mut self) {
        loop {
            if self.is_at_end() || '"' == self.peek() {
                break;
            }
            if '\n' == self.peek() {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            RLox::error(self.line as u32, "Unterminated string.");
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenKind::String, Literal::String(value));
    }

    fn is_digit(c: u8) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn is_alpha_numeric(c: u8) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn number(&mut self) {
        while !self.is_at_end() && Scanner::is_digit(self.peek() as u8) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' && Scanner::is_digit(self.peek_next() as u8) {
            self.advance();
            while !self.is_at_end() && Scanner::is_digit(self.peek() as u8) {
                self.advance();
            }
        }

        self.add_token(
            TokenKind::Number,
            Literal::Number(self.source[self.start..self.current].parse().unwrap()),
        );
    }

    fn identifier(&mut self) {
        while !self.is_at_end() && Self::is_alpha_numeric(self.peek() as u8) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_kind) = KEYWORD_MAP.get(text) {
            self.add_token(*token_kind, Literal::String(text));
        } else {
            self.add_token(TokenKind::Identifier, Literal::String(text));
        }
    }
}
