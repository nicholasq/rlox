#[cfg(test)]
pub mod tests {
    use crate::token::{self, Token, TokenKind};

    pub fn token_eof(line: usize) -> token::Token {
        token::Token {
            kind: token::TokenKind::Eof,
            lexeme: "".to_string(),
            literal: token::Literal::None,
            line,
        }
    }

    impl From<&str> for Token {
        fn from(value: &str) -> Self {
            match value {
                "+" => Token {
                    literal: value.into(),
                    kind: TokenKind::Plus,
                    lexeme: "+".to_string(),
                    line: 1,
                },
                "*" => Token {
                    literal: value.into(),
                    kind: TokenKind::Star,
                    lexeme: "*".to_string(),
                    line: 1,
                },
                "-" => Token {
                    literal: value.into(),
                    kind: TokenKind::Minus,
                    lexeme: "-".to_string(),
                    line: 1,
                },
                "/" => Token {
                    literal: value.into(),
                    kind: TokenKind::Slash,
                    lexeme: "/".to_string(),
                    line: 1,
                },
                _ => panic!("{} not a valid token value", value),
            }
        }
    }

    pub mod test_case {
        pub struct TestCase<I, E> {
            pub input: I,
            pub expected: E,
        }
    }
}
