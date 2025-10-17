use std::fmt::Display;

/// Macro to convert a value into a Literal using `.into()`.
/// Usage: `lit!(value)`
#[macro_export]
macro_rules! lit {
    ($value:expr) => {
        $crate::expr::Literal::from($value)
    };
}

/// Macro to create an Expr::Literal from a value that can be converted into a Literal.
/// Usage: `expr_lit!(value)`
#[macro_export]
macro_rules! expr_lit {
    ($value:expr) => {
        $crate::expr::Expr::Literal($crate::expr::Literal::from($value))
    };
}

use crate::token::Token;

/// Represents an expression in the abstract syntax tree (AST).
///
/// The `Expr` enum covers all possible expression types:
/// - `Binary`: A binary operation (e.g., addition, subtraction) with a left and right operand and an operator.
/// - `Grouping`: An expression wrapped in parentheses to control precedence.
/// - `Literal`: A literal value (e.g., number, string, boolean).
/// - `Unary`: A unary operation (e.g., negation) with an operator and a right operand.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A binary operation expression.
    ///
    /// # Fields
    /// - `left`: The left-hand side expression.
    /// - `operator`: The operator token (e.g., '+', '-', '*', '/').
    /// - `right`: The right-hand side expression.
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    /// A grouping expression, typically used for parenthesized expressions.
    ///
    /// # Fields
    /// - `expression`: The inner expression.
    Grouping { expression: Box<Expr> },
    /// A literal value expression.
    ///
    /// # Fields
    /// - `0`: The literal value.
    Literal(Literal),
    /// A unary operation expression.
    ///
    /// # Fields
    /// - `operator`: The operator token (e.g., '-', '!').
    /// - `right`: The operand expression.
    Unary { operator: Token, right: Box<Expr> },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal(literal) => {
                write!(f, "{}", literal)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator.lexeme, right)
            }
        }
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number(value)
    }
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String(value.to_string())
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Boolean(if value {
            LiteralBool::True
        } else {
            LiteralBool::False
        })
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Literal(value.into())
    }
}

impl From<String> for Expr {
    fn from(value: String) -> Self {
        Expr::Literal(value.into())
    }
}

impl From<&str> for Expr {
    fn from(value: &str) -> Self {
        Expr::Literal(value.into())
    }
}

impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Expr::Literal(value.into())
    }
}

impl Expr {
    pub fn literal<T>(value: T) -> Self
    where
        T: Into<Literal>,
    {
        Expr::Literal(value.into())
    }

    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping {
            expression: Box::new(expr),
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LiteralBool {
    True,
    False,
}

impl Display for LiteralBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralBool::True => write!(f, "true"),
            LiteralBool::False => write!(f, "false"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(LiteralBool),
    Identifier(String),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Identifier(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::None => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests::test_case::TestCase;

    #[test]
    fn test_expr_display() {
        let test_cases = vec![
            TestCase {
                input: Expr::unary("-".into(), 456.0.into()),
                expected: "(- 456)",
            },
            TestCase {
                input: Expr::grouping(Expr::binary(456f64.into(), "*".into(), 789f64.into())),
                expected: "(group (* 456 789))",
            },
            TestCase {
                input: "hello".into(),
                expected: "\"hello\"",
            },
            TestCase {
                input: Expr::binary(
                    Expr::unary("-".into(), (123f64).into()),
                    "*".into(),
                    Expr::grouping(45.67.into()),
                ),
                expected: "(* (- 123) (group 45.67))",
            },
        ];

        for case in test_cases {
            assert_eq!(case.input.to_string(), case.expected);
        }
    }
}
