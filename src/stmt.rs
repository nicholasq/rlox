use std::fmt::{self, Display, Formatter};

use crate::{expr::Expr, token::Token};

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
    Block(Vec<Stmt>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Stmt::Expr(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {}", expr),
            Stmt::Var { name, initializer } => {
                write!(f, "var {} = {}", name.lexeme, initializer)
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
