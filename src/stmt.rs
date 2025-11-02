use std::fmt::{self, Display, Formatter};

use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print(Expr),
    Var {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "if {} then {} ", condition, then_branch)?;
                if let Some(else_branch) = else_branch {
                    write!(f, "else {}", else_branch)?;
                }
                Ok(())
            }
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::Var { name, initializer } => {
                write!(f, "var {} = {}", name.lexeme, initializer)
            }
            Stmt::While { condition, body } => {
                write!(f, "while {} {}", condition, body)
            }
            Stmt::Block(statements) => {
                write!(f, "{{ ")?;
                for stmt in statements {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "}}")
            }
        }
    }
}
