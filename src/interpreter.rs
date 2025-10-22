use crate::{
    environment::Environment,
    error::RuntimeError,
    expr::{Expr, Literal, LiteralBool},
    lit, stmt,
    token::{Token, TokenKind},
};
use anyhow::Result;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Interpreter { environment }
    }
    pub fn interpret(&mut self, stmts: Vec<stmt::Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: stmt::Stmt) -> Result<(), RuntimeError> {
        match stmt {
            stmt::Stmt::Expr(expression) => {
                self.evaluate(expression)?;
                Ok(())
            }
            stmt::Stmt::Print(expression) => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
            stmt::Stmt::Var { name, initializer } => {
                self.var_stmt(name, initializer)?;
                Ok(())
            }
            stmt::Stmt::Block(stmts) => {
                self.block_stmt(stmts)?;
                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(literal),
            Expr::Grouping { expression } => self.evaluate(*expression),
            Expr::Unary { operator, right } => self.unary(operator, *right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary(operator, *left, *right),
            Expr::Variable { name } => self.var_expr(name),
            Expr::Assign { name, value } => self.assign(name, *value),
        }
    }

    fn block_stmt(&mut self, stmts: Vec<stmt::Stmt>) -> Result<Literal, RuntimeError> {
        self.environment.add_scope();
        for stmt in stmts {
            self.execute(stmt)?;
        }
        self.environment.pop_scope();
        Ok(Literal::None)
    }

    fn unary(&mut self, operator: Token, right: Expr) -> Result<Literal, RuntimeError> {
        let right = self.evaluate(right)?;
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

    fn binary(
        &mut self,
        operator: Token,
        left: Expr,
        right: Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

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

    fn var_stmt(&mut self, name: Token, initializer: Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(initializer)?;
        self.environment.define(name.lexeme, value);
        Ok(Literal::None)
    }

    fn var_expr(&mut self, name: Token) -> Result<Literal, RuntimeError> {
        self.environment.get(&name).cloned()
    }

    fn assign(&mut self, name: Token, value: Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate(value)?;
        self.environment.assign(&name, &value)?;
        Ok(Literal::None)
    }
}

fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Boolean(b) => matches!(b, LiteralBool::True),
        Literal::None => false,
        _ => true,
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
            let environment = Environment::new();
            let mut interpreter = Interpreter::new(environment);
            let value = interpreter.evaluate(test_case.input);
            if test_case.expected == lit!("error") {
                assert!(value.is_err());
            } else {
                assert!(value.is_ok());
                assert_eq!(value.unwrap(), test_case.expected);
            }
        }
    }
}
