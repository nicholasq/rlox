use crate::{
    expr::{Expr, Literal, LiteralBool},
    lit,
    token::{Token, TokenKind},
};
use anyhow::Result;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub token: Token,
}

impl RuntimeError {
    fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }
}

pub fn interpret(expr: Expr) -> Result<Literal, RuntimeError> {
    evaluate(expr)
}

fn evaluate(expr: Expr) -> Result<Literal, RuntimeError> {
    match expr {
        Expr::Literal(literal) => Ok(literal),
        Expr::Grouping { expression } => evaluate(*expression),
        Expr::Unary { operator, right } => unary(operator, *right),
        Expr::Binary {
            left,
            operator,
            right,
        } => binary(operator, *left, *right),
    }
}

fn unary(operator: Token, right: Expr) -> Result<Literal, RuntimeError> {
    let right = evaluate(right)?;
    match operator.kind {
        TokenKind::Minus => match right {
            Literal::Number(n) => Ok(lit!(-n)),
            _ => Err(RuntimeError::new(
                format!("Operand must be a number, got {:?}", right),
                operator,
            )),
        },
        TokenKind::Bang => {
            if is_truthy(&right) {
                Ok(lit!(false))
            } else {
                Ok(lit!(true))
            }
        }
        kind => Err(RuntimeError::new(
            format!("Unsupported operator kind: {:?}", kind),
            operator,
        )),
    }
}

fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Boolean(b) => matches!(b, LiteralBool::True),
        Literal::None => false,
        _ => true,
    }
}

fn binary(operator: Token, left: Expr, right: Expr) -> Result<Literal, RuntimeError> {
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    match (&left, &right) {
        (Literal::Number(l), Literal::Number(r)) => match operator.kind {
            TokenKind::Greater => Ok(lit!(l > r)),
            TokenKind::GreaterEqual => Ok(lit!(l >= r)),
            TokenKind::Less => Ok(lit!(l < r)),
            TokenKind::LessEqual => Ok(lit!(l <= r)),
            TokenKind::BangEqual => Ok(lit!(l != r)),
            TokenKind::EqualEqual => Ok(lit!(l == r)),
            TokenKind::Plus => Ok(lit!(l + r)),
            TokenKind::Minus => Ok(lit!(l - r)),
            TokenKind::Slash => Ok(lit!(l / r)),
            TokenKind::Star => Ok(lit!(l * r)),
            _ => Err(RuntimeError::new(
                format!(
                    "Unsupported binary operator {:?} for numbers",
                    operator.kind
                ),
                operator,
            )),
        },
        (Literal::String(l), Literal::String(r)) => match operator.kind {
            TokenKind::Plus => {
                let mut value = l.clone();
                value.push_str(r);
                Ok(lit!(value))
            }
            _ => Err(RuntimeError::new(
                format!(
                    "Unsupported binary operator {:?} for strings",
                    operator.kind
                ),
                operator,
            )),
        },
        (left, right) => match operator.kind {
            TokenKind::BangEqual => Ok(lit!(!is_equal(left, right))),
            TokenKind::EqualEqual => Ok(lit!(is_equal(left, right))),
            _ => Err(RuntimeError::new(
                format!(
                    "Unsupported binary operator {:?} for types ({:?}, {:?})",
                    operator.kind, left, right
                ),
                operator,
            )),
        },
    }
}

fn is_equal(left: &Literal, right: &Literal) -> bool {
    match (left, right) {
        (Literal::None, Literal::None) => true,
        (Literal::None, _) => false,
        (left, right) => left == right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::test_case;
    use crate::{
        expr,
        token::{Literal, Token},
    };

    fn token(kind: TokenKind, literal: Literal) -> Token {
        Token {
            kind,
            literal,
            lexeme: "".to_string(),
            line: 0,
        }
    }

    #[test]
    fn test_valid_expressions() {
        let test_cases = vec![
            test_case::TestCase {
                input: Expr::Unary {
                    operator: token(TokenKind::Minus, "-".into()),
                    right: Box::new(Expr::Literal(lit!(5.0))),
                },
                expected: lit!(-5.0),
            },
            test_case::TestCase {
                input: Expr::Unary {
                    operator: token(TokenKind::Bang, "!".into()),
                    right: Box::new(Expr::Literal(lit!(true))),
                },
                expected: lit!(false),
            },
            test_case::TestCase {
                input: Expr::Unary {
                    operator: token(TokenKind::Bang, "!".into()),
                    right: Box::new(Expr::Literal(lit!(false))),
                },
                expected: lit!(true),
            },
            test_case::TestCase {
                input: Expr::Unary {
                    operator: token(TokenKind::Bang, "!".into()),
                    right: Box::new(Expr::Literal(expr::Literal::None)),
                },
                expected: lit!(true),
            },
            test_case::TestCase {
                input: Expr::Unary {
                    operator: token(TokenKind::Minus, "-".into()),
                    right: Box::new(Expr::Literal(lit!(true))),
                },
                expected: lit!("error"),
            },
        ];

        for test_case in test_cases {
            let value = interpret(test_case.input);
            if test_case.expected == lit!("error") {
                assert!(value.is_err());
            } else {
                assert!(value.is_ok());
                assert_eq!(value.unwrap(), test_case.expected);
            }
        }
    }
}
