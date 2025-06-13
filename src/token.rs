use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    Identifier(&'a str),
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub literal: Literal<'a>,
    pub line: usize,
}

pub(crate) static KEYWORD_MAP: LazyLock<HashMap<&str, TokenKind>> = LazyLock::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert("and", TokenKind::And);
    keywords.insert("class", TokenKind::Class);
    keywords.insert("else", TokenKind::Else);
    keywords.insert("false", TokenKind::False);
    keywords.insert("fun", TokenKind::Fun);
    keywords.insert("for", TokenKind::For);
    keywords.insert("if", TokenKind::If);
    keywords.insert("nil", TokenKind::Nil);
    keywords.insert("or", TokenKind::Or);
    keywords.insert("print", TokenKind::Print);
    keywords.insert("return", TokenKind::Return);
    keywords.insert("super", TokenKind::Super);
    keywords.insert("this", TokenKind::This);
    keywords.insert("true", TokenKind::True);
    keywords.insert("var", TokenKind::Var);
    keywords.insert("while", TokenKind::While);
    keywords
});
