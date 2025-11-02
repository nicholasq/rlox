use crate::{
    environment::Environment,
    error::RuntimeError,
    expr::{Expr, Literal, LiteralBool},
    lit, runtime_err, stmt,
    token::{Token, TokenKind},
};
use anyhow::Result;
use std::io::Write;

pub struct Interpreter<'a, W: Write> {
    pub environment: Environment,
    pub output: &'a mut W,
}

impl<'a, W: Write> Interpreter<'a, W> {
    pub fn new(environment: Environment, output: &'a mut W) -> Self {
        Interpreter {
            environment,
            output,
        }
    }
    pub fn interpret(&mut self, stmts: &[stmt::Stmt]) -> Result<(), RuntimeError> {
        for stmt in stmts {
            self.execute_stmt(stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &stmt::Stmt) -> Result<(), RuntimeError> {
        match stmt {
            stmt::Stmt::Expr(expression) => {
                self.evaluate_expr(expression)?;
                Ok(())
            }
            stmt::Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.if_stmt(condition, then_branch, else_branch.as_deref())?;
                Ok(())
            }
            stmt::Stmt::Print(expression) => {
                let value = self.evaluate_expr(expression)?;
                writeln!(self.output, "{}", value.printable()).map_err(|_| {
                    runtime_err!(
                        format!("failed to write output for expr: {}", expression),
                        None
                    )
                })?;
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
            stmt::Stmt::While { condition, body } => {
                self.while_stmt(condition, body)?;
                Ok(())
            }
        }
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(literal.clone()),
            Expr::Grouping { expression } => self.evaluate_expr(expression),
            Expr::Unary { operator, right } => self.unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.binary(operator, left, right),
            Expr::Variable { name } => self.var_expr(name),
            Expr::Assign { name, value } => self.assign(name, value),
            Expr::Logical {
                left,
                operator,
                right,
            } => self.logical(operator, left, right),
        }
    }

    fn if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &stmt::Stmt,
        else_branch: Option<&stmt::Stmt>,
    ) -> Result<Literal, RuntimeError> {
        let condition = self.evaluate_expr(condition)?;
        if is_truthy(&condition) {
            self.execute_stmt(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute_stmt(else_branch)?;
        }
        Ok(Literal::None)
    }

    fn while_stmt(&mut self, condition: &Expr, body: &stmt::Stmt) -> Result<Literal, RuntimeError> {
        let mut cond = self.evaluate_expr(condition)?;
        while is_truthy(&cond) {
            let stmt = body;
            self.execute_stmt(stmt)?;
            cond = self.evaluate_expr(condition)?;
        }
        Ok(Literal::None)
    }

    fn block_stmt(&mut self, stmts: &[stmt::Stmt]) -> Result<Literal, RuntimeError> {
        self.environment.add_scope();
        for stmt in stmts {
            self.execute_stmt(stmt)?;
        }
        self.environment.pop_scope();
        Ok(Literal::None)
    }

    fn unary(&mut self, operator: &Token, right: &Expr) -> Result<Literal, RuntimeError> {
        let right = self.evaluate_expr(right)?;
        match operator.kind {
            TokenKind::Minus => match right {
                Literal::Number(n) => Ok(lit!(-n)),
                _ => Err(runtime_err!(
                    format!("Operand must be a number, got {:?}", right),
                    Some(operator.clone())
                )),
            },
            TokenKind::Bang => {
                if is_truthy(&right) {
                    Ok(lit!(false))
                } else {
                    Ok(lit!(true))
                }
            }
            kind => Err(runtime_err!(
                format!("Unsupported operator kind: {:?}", kind),
                Some(operator.clone())
            )),
        }
    }

    fn binary(
        &mut self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left = self.evaluate_expr(left)?;
        let right = self.evaluate_expr(right)?;

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
                _ => Err(runtime_err!(
                    format!(
                        "Unsupported binary operator {:?} for numbers",
                        operator.kind
                    ),
                    Some(operator.clone())
                )),
            },
            (Literal::String(l), Literal::String(r)) => match operator.kind {
                TokenKind::Plus => {
                    let mut value = l.clone();
                    value.push_str(r);
                    Ok(lit!(value))
                }
                _ => Err(runtime_err!(
                    format!(
                        "Unsupported binary operator {:?} for strings",
                        operator.kind
                    ),
                    Some(operator.clone())
                )),
            },
            (left, right) => match operator.kind {
                TokenKind::BangEqual => Ok(lit!(!is_equal(left, right))),
                TokenKind::EqualEqual => Ok(lit!(is_equal(left, right))),
                _ => Err(runtime_err!(
                    format!(
                        "Unsupported binary operator {:?} for types ({:?}, {:?})",
                        operator.kind, left, right
                    ),
                    Some(operator.clone())
                )),
            },
        }
    }

    fn var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate_expr(initializer)?;
        self.environment.define(name.lexeme.clone(), value);
        Ok(Literal::None)
    }

    fn var_expr(&mut self, name: &Token) -> Result<Literal, RuntimeError> {
        self.environment.get(name).cloned()
    }

    fn assign(&mut self, name: &Token, value: &Expr) -> Result<Literal, RuntimeError> {
        let value = self.evaluate_expr(value)?;
        self.environment.assign(name, &value)?;
        Ok(Literal::None)
    }

    fn logical(
        &mut self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<Literal, RuntimeError> {
        let left_val = self.evaluate_expr(left)?;

        match operator.kind {
            TokenKind::And => {
                if !is_truthy(&left_val) {
                    Ok(left_val)
                } else {
                    self.evaluate_expr(right)
                }
            }
            TokenKind::Or => {
                if is_truthy(&left_val) {
                    Ok(left_val)
                } else {
                    self.evaluate_expr(right)
                }
            }
            _ => Err(runtime_err!(
                format!("Unsupported logical operator: {:?}", operator.kind),
                Some(operator.clone())
            )),
        }
    }
}

/// Checks if a given `Literal` value is truthy.
///
/// A value is considered truthy if:
/// - It is a `Literal::Boolean` with the value `LiteralBool::True`.
/// - It is not `Literal::None`.
/// - All other values are considered truthy.
///
/// # Parameters
/// - `literal`: A reference to the `Literal` to evaluate.
///
/// # Returns
/// - `true` if the given literal is truthy.
/// - `false` otherwise.
fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Boolean(b) => matches!(b, LiteralBool::True),
        Literal::None => false,
        _ => true,
    }
}

/// Returns true if the two `Literal` values are equal.
///
/// - Returns `true` if both are `Literal::None`.
/// - Returns `false` if only one is `Literal::None`.
/// - Otherwise, compares the values for equality.
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
            let environment = Environment::default();
            let mut output = Vec::new();
            let mut interpreter = Interpreter::new(environment, &mut output);
            let value = interpreter.evaluate_expr(&test_case.input);
            if test_case.expected == lit!("error") {
                assert!(value.is_err());
            } else {
                assert!(value.is_ok());
                assert_eq!(value.unwrap(), test_case.expected);
            }
        }
    }
}
