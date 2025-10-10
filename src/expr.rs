use std::fmt::Display;

use crate::token::{self, Token};

pub trait Expr: Display {}

struct Binary<'a> {
    left: Box<dyn Expr>,
    operator: Token<'a>,
    right: Box<dyn Expr>,
}

impl Expr for Binary<'_> {}

impl Display for Binary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator.lexeme, self.left, self.right)
    }
}

struct Grouping {
    expression: Box<dyn Expr>,
}

impl Expr for Grouping {}

impl Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

struct Literal<'a> {
    value: token::Literal<'a>,
}

impl Expr for Literal<'_> {}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            token::Literal::Number(n) => write!(f, "{}", n),
            token::Literal::String(s) => write!(f, "\"{}\"", s),
            token::Literal::Identifier(s) => write!(f, "{}", s),
            token::Literal::None => write!(f, "nil"),
        }
    }
}

struct Unary<'a> {
    operator: Token<'a>,
    right: Box<dyn Expr>,
}

impl Expr for Unary<'_> {}

impl Display for Unary<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator.lexeme, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Literal as TokenLiteral, Token, TokenKind};

    fn literal_number(value: f64) -> Literal<'static> {
        Literal {
            value: TokenLiteral::Number(value),
        }
    }

    fn literal_string(value: &'static str) -> Literal<'static> {
        Literal {
            value: TokenLiteral::String(value),
        }
    }

    fn unary_expr(operator: Token<'static>, right: Box<dyn Expr>) -> Unary<'static> {
        Unary { operator, right }
    }

    fn binary_expr(
        left: Box<dyn Expr>,
        operator: Token<'static>,
        right: Box<dyn Expr>,
    ) -> Binary<'static> {
        Binary {
            left,
            operator,
            right,
        }
    }

    #[test]
    fn test_expr_display() {
        struct TestCase<'a> {
            expr: &'a dyn Expr,
            expected: &'a str,
        }

        let token_plus = Token {
            kind: TokenKind::Plus,
            lexeme: "+",
            literal: TokenLiteral::None,
            line: 1,
        };
        let unary_plus = unary_expr(token_plus, Box::new(literal_number(456.0)));

        let token_star = Token {
            kind: TokenKind::Star,
            lexeme: "*",
            literal: TokenLiteral::None,
            line: 1,
        };
        let binary = binary_expr(
            Box::new(literal_number(456.0)),
            token_star,
            Box::new(literal_number(789.0)),
        );

        let grouping = Grouping {
            expression: Box::new(binary),
        };

        let literal_str = literal_string("hello");

        let token_minus = Token {
            kind: TokenKind::Minus,
            lexeme: "-",
            literal: TokenLiteral::None,
            line: 1,
        };
        let unary_minus = unary_expr(token_minus, Box::new(literal_number(123.0)));

        let grouping_45_67 = Grouping {
            expression: Box::new(literal_number(45.67)),
        };

        let token_star = Token {
            kind: TokenKind::Star,
            lexeme: "*",
            literal: TokenLiteral::None,
            line: 1,
        };
        let binary_complex =
            binary_expr(Box::new(unary_minus), token_star, Box::new(grouping_45_67));

        let test_cases = vec![
            TestCase {
                expr: &unary_plus,
                expected: "(+ 456)",
            },
            TestCase {
                expr: &grouping,
                expected: "(group (* 456 789))",
            },
            TestCase {
                expr: &literal_str,
                expected: "\"hello\"",
            },
            TestCase {
                expr: &binary_complex,
                expected: "(* (- 123) (group 45.67))",
            },
        ];
        for case in test_cases {
            assert_eq!(case.expr.to_string(), case.expected);
        }
    }
}
