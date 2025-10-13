use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Identifier(String),
    None,
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String(value.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

pub(crate) static KEYWORD_MAP: LazyLock<HashMap<&str, TokenKind>> = LazyLock::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert(keywords::AND, TokenKind::And);
    keywords.insert(keywords::CLASS, TokenKind::Class);
    keywords.insert(keywords::ELSE, TokenKind::Else);
    keywords.insert(keywords::FALSE, TokenKind::False);
    keywords.insert(keywords::FUN, TokenKind::Fun);
    keywords.insert(keywords::FOR, TokenKind::For);
    keywords.insert(keywords::IF, TokenKind::If);
    keywords.insert(keywords::NIL, TokenKind::Nil);
    keywords.insert(keywords::OR, TokenKind::Or);
    keywords.insert(keywords::PRINT, TokenKind::Print);
    keywords.insert(keywords::RETURN, TokenKind::Return);
    keywords.insert(keywords::SUPER, TokenKind::Super);
    keywords.insert(keywords::THIS, TokenKind::This);
    keywords.insert(keywords::TRUE, TokenKind::True);
    keywords.insert(keywords::VAR, TokenKind::Var);
    keywords.insert(keywords::WHILE, TokenKind::While);
    keywords
});

pub mod keywords {
    pub const AND: &str = "and";
    pub const CLASS: &str = "class";
    pub const ELSE: &str = "else";
    pub const FALSE: &str = "false";
    pub const FUN: &str = "fun";
    pub const FOR: &str = "for";
    pub const IF: &str = "if";
    pub const NIL: &str = "nil";
    pub const OR: &str = "or";
    pub const PRINT: &str = "print";
    pub const RETURN: &str = "return";
    pub const SUPER: &str = "super";
    pub const THIS: &str = "this";
    pub const TRUE: &str = "true";
    pub const VAR: &str = "var";
    pub const WHILE: &str = "while";
}
