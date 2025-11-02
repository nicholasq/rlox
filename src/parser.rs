use crate::{
    expr, expr_lit,
    stmt::{self},
    token::{self, TokenKind},
};
use anyhow::{anyhow, Result};

/// The `Parser` struct is responsible for parsing a sequence of tokens.
///
/// # Fields
/// - `tokens`: A vector containing the tokens to be parsed.
/// - `current`: The index of the current token being processed.
pub struct Parser<'a> {
    tokens: &'a Vec<token::Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<token::Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses an expression and returns the resulting AST node.
    /// This is the entry point for parsing expressions.
    pub fn parse(&mut self) -> Result<Vec<stmt::Stmt>> {
        let mut statements = Vec::new();
        while !self.at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    /// Parses a declaration statement, such as a variable declaration.
    /// This method attempts to parse a `var` declaration or falls back to a generic statement.
    fn declaration(&mut self) -> Result<stmt::Stmt> {
        match self.match_kinds(&[TokenKind::Var]) {
            true => self.var_declaration().inspect_err(|_| {
                self.synchronize();
            }),
            false => self.statement().inspect_err(|_| {
                self.synchronize();
            }),
        }
    }

    /// Parses a variable declaration statement.
    /// If an initializer is present, it is parsed; otherwise, the initializer is set to `None`.
    /// This method expects a semicolon after the declaration.
    fn var_declaration(&mut self) -> Result<stmt::Stmt> {
        let name = self
            .consume(&TokenKind::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.match_kinds(&[TokenKind::Equal]) {
            self.expression()?
        } else {
            expr::Expr::literal(expr::Literal::None)
        };

        self.consume(
            &TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(stmt::Stmt::Var { name, initializer })
    }

    /// Parses a statement. This can be a print statement, a block statement, or an expression statement.
    fn statement(&mut self) -> Result<stmt::Stmt> {
        if self.match_kinds(&[TokenKind::If]) {
            return self.if_statement();
        }
        if self.match_kinds(&[TokenKind::Print]) {
            return self.print_statement();
        }
        if self.match_kinds(&[TokenKind::LeftBrace]) {
            return self.block_statement();
        }
        if self.match_kinds(&[TokenKind::While]) {
            return self.while_statement();
        }
        if self.match_kinds(&[TokenKind::For]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(&TokenKind::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenKind::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_kinds(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(stmt::Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn for_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(&TokenKind::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_kinds(&[TokenKind::Semicolon]) {
            None
        } else if self.match_kinds(&[TokenKind::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenKind::Semicolon) {
            self.expression()?
        } else {
            expr_lit!(true)
        };
        self.consume(&TokenKind::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenKind::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(&TokenKind::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            body = stmt::Stmt::Block(vec![body, stmt::Stmt::Expr(inc)]);
        }

        body = stmt::Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = stmt::Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<stmt::Stmt> {
        self.consume(&TokenKind::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenKind::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        Ok(stmt::Stmt::While { condition, body })
    }

    /// Parses a block statement, which consists of multiple statements enclosed in braces (`{}`).
    fn block_statement(&mut self) -> Result<stmt::Stmt> {
        let mut statements = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenKind::RightBrace, "Expect '}' after block.")?;
        Ok(stmt::Stmt::Block(statements))
    }

    /// Parses a print statement. This method assumes the `print` keyword has already been matched.
    fn print_statement(&mut self) -> Result<stmt::Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenKind::Semicolon, "Expect ';' after expression.")?;
        Ok(stmt::Stmt::Print(expr))
    }

    /// Parses an expression statement. This method expects a semicolon after the expression.
    fn expression_statement(&mut self) -> Result<stmt::Stmt> {
        let expr = self.expression()?;
        self.consume(
            &TokenKind::Semicolon,
            "Expect ';' after expression statement.",
        )?;
        Ok(stmt::Stmt::Expr(expr))
    }

    /// Parses an expression, currently delegates to equality parsing.
    fn expression(&mut self) -> Result<expr::Expr> {
        self.assignment()
    }

    /// Parses an assignment expression. This includes parsing variable assignments.
    /// If the left-hand side is not a valid assignment target, an error is raised.
    fn assignment(&mut self) -> Result<expr::Expr> {
        let expr = self.or()?;

        if self.match_kinds(&[TokenKind::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            return match expr {
                expr::Expr::Variable { name } => Ok(expr::Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => {
                    crate::error_token(&equals, "invalid assignment target.");
                    Err(anyhow!("Invalid assignment target."))
                }
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<expr::Expr> {
        let mut expr = self.and()?;

        while self.match_kinds(&[TokenKind::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = expr::Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<expr::Expr> {
        let mut expr = self.equality()?;

        while self.match_kinds(&[TokenKind::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = expr::Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    /// Parses equality expressions (==, !=).
    /// Continues parsing as long as equality operators are found,
    /// building a binary expression tree.
    fn equality(&mut self) -> Result<expr::Expr> {
        let mut expr = self.comparison()?;

        while self.match_kinds(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = expr::Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses comparison expressions (>, >=, <, <=).
    /// Continues parsing as long as comparison operators are found,
    /// building a binary expression tree.
    fn comparison(&mut self) -> Result<expr::Expr> {
        let mut expr = self.term()?;

        while self.match_kinds(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = expr::Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a term expression, handling addition and subtraction.
    ///
    /// A term consists of one or more factors separated by '+' or '-'.
    /// For example: `a + b - c`
    fn term(&mut self) -> Result<expr::Expr> {
        let mut expr = self.factor()?;

        // Continue parsing as long as the next token is '+' or '-'.
        while self.match_kinds(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = expr::Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a factor expression, handling multiplication and division.
    ///
    /// A factor consists of one or more unary expressions separated by '*' or '/'.
    /// For example: `a * b / c`
    fn factor(&mut self) -> Result<expr::Expr> {
        let mut expr = self.unary()?;

        // Continue parsing as long as the next token is '*' or '/'.
        while self.match_kinds(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = expr::Expr::binary(expr, operator, right);
        }

        Ok(expr)
    }

    /// Parses a unary expression, handling negation and logical NOT.
    ///
    /// A unary expression starts with '-' or '!' followed by another unary or primary expression.
    /// For example: `-a`, `!b`
    fn unary(&mut self) -> Result<expr::Expr> {
        // If the next token is '!' or '-', parse as a unary expression.
        if self.match_kinds(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(expr::Expr::unary(operator, right));
        }
        // Otherwise, parse as a primary expression.
        self.primary()
    }

    /// Parses a primary expression from the token stream.
    ///
    /// This method handles literals (`true`, `false`, `nil`, numbers, strings),
    /// and parenthesized expressions. If the next token matches one of these,
    /// it consumes the token and returns the corresponding expression node.
    ///
    /// # Returns
    ///
    /// * `Ok(expr::Expr)` - The parsed primary expression.
    /// * `Err(anyhow)` - If no valid primary expression is found.
    fn primary(&mut self) -> Result<expr::Expr> {
        if self.match_kinds(&[TokenKind::False]) {
            return Ok(false.into());
        }
        if self.match_kinds(&[TokenKind::True]) {
            return Ok(true.into());
        }
        if self.match_kinds(&[TokenKind::Nil]) {
            return Ok(expr::Expr::literal(expr::Literal::None));
        }
        if self.match_kinds(&[TokenKind::Number, TokenKind::String]) {
            match &self.previous().literal {
                token::Literal::String(str) => {
                    return Ok(str.clone().into());
                }
                token::Literal::Number(num) => return Ok((*num).into()),
                _ => panic!(),
            }
        }
        if self.match_kinds(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenKind::RightParen, "Expect ')' after expression.")
                .unwrap();
            return Ok(expr::Expr::grouping(expr));
        }

        if self.match_kinds(&[TokenKind::Identifier]) {
            let name = self.previous().clone();
            return Ok(expr::Expr::Variable { name });
        }

        Err(anyhow!("Expect expression."))
    }

    /// Attempts to match any of the provided `TokenKind`s against the current token.
    /// If a match is found, advances the parser and returns `true`.
    /// Otherwise, returns `false` without advancing.
    fn match_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consumes the current token if it matches the given `TokenKind`.
    /// Returns a reference to the consumed token on success.
    /// If the token does not match, reports an error and returns an `Err`.
    ///
    /// # Arguments
    /// * `kind` - The expected kind of token.
    /// * `message` - The error message to display if the token does not match.
    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<&token::Token> {
        if self.check(kind) {
            return Ok(self.advance());
        }
        crate::error_token(self.peek(), message);
        Err(anyhow!("{}", message))
    }

    /// Checks if the current token matches the given `TokenKind`.
    /// Returns `false` if at the end of the token stream.
    /// Otherwise, returns `true` if the current token matches, `false` otherwise.
    fn check(&self, kind: &TokenKind) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().kind == *kind
    }

    /// Advances the current token pointer to the next token if not at the end.
    /// Returns a reference to the previous token after advancing.
    fn advance(&mut self) -> &token::Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Checks if the current token is the end-of-file (EOF) token.
    /// Returns true if at the end, false otherwise.
    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Returns a reference to the current token without advancing the position.
    ///
    /// # Returns
    /// A reference to the [`token::Token`] at the current position in the token stream.
    ///
    /// # Panics
    /// Panics if `self.current` is out of bounds of the `tokens` vector.
    fn peek(&self) -> &token::Token {
        &self.tokens[self.current]
    }

    /// Returns a reference to the previous token in the token stream.
    ///
    /// # Panics
    /// This function will panic if `self.current` is 0, as it would attempt to access
    /// an out-of-bounds index in the `tokens` vector.
    ///
    /// # Returns
    /// A reference to the token immediately before the current position.
    fn previous(&self) -> &token::Token {
        &self.tokens[self.current - 1]
    }

    /// Synchronizes the parser state after encountering an error.
    ///
    /// This method advances the parser until it finds a statement boundary,
    /// such as a semicolon or the start of a new statement (e.g., 'class', 'fun', 'var', etc.).
    /// This helps the parser recover and continue parsing subsequent statements.
    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            // If the previous token was a semicolon, assume statement boundary and return.
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            // If the next token is the start of a new statement, return.
            match self.peek().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => {}
            }

            // Otherwise, keep advancing.
            self.advance();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{self};
    use crate::utils::tests::{test_case::TestCase, token_eof};

    #[test]
    fn test_expression_parsing() {
        let test_cases = vec![
            TestCase {
                input: vec![
                    token::Token {
                        kind: token::TokenKind::Minus,
                        lexeme: "-".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Number,
                        lexeme: "456".to_string(),
                        literal: 456f64.into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Semicolon,
                        lexeme: ";".to_string(),
                        literal: ";".into(),
                        line: 1,
                    },
                    token_eof(1),
                ],
                expected: "(- 456)",
            },
            TestCase {
                input: vec![
                    token::Token {
                        kind: token::TokenKind::LeftParen,
                        lexeme: "(".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Number,
                        lexeme: "456".to_string(),
                        literal: 456f64.into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Star,
                        lexeme: "*".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Number,
                        lexeme: "789".to_string(),
                        literal: 789f64.into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::RightParen,
                        lexeme: ")".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Semicolon,
                        lexeme: ";".to_string(),
                        literal: ";".into(),
                        line: 1,
                    },
                    token_eof(1),
                ],
                expected: "(group (* 456 789))",
            },
            TestCase {
                input: vec![
                    token::Token {
                        kind: token::TokenKind::String,
                        lexeme: "\"hello\"".to_string(),
                        literal: "hello".into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Semicolon,
                        lexeme: ";".to_string(),
                        literal: ";".into(),
                        line: 1,
                    },
                    token_eof(1),
                ],
                expected: "\"hello\"",
            },
            TestCase {
                input: vec![
                    token::Token {
                        kind: token::TokenKind::Minus,
                        lexeme: "-".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Number,
                        lexeme: "123".to_string(),
                        literal: 123f64.into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Star,
                        lexeme: "*".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::LeftParen,
                        lexeme: "(".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Number,
                        lexeme: "45.67".to_string(),
                        literal: 45.67f64.into(),
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::RightParen,
                        lexeme: ")".to_string(),
                        literal: token::Literal::None,
                        line: 1,
                    },
                    token::Token {
                        kind: token::TokenKind::Semicolon,
                        lexeme: ";".to_string(),
                        literal: ";".into(),
                        line: 1,
                    },
                    token_eof(1),
                ],
                expected: "(* (- 123) (group 45.67))",
            },
        ];

        for test_case in test_cases {
            let mut parser = Parser::new(&test_case.input);
            let result = parser.parse();

            match result {
                Err(e) => panic!("Parsing failed: {} input: {:?}", e, test_case.input),
                Ok(expr) => {
                    assert!(expr.len() == 1);
                    assert_eq!(expr.first().unwrap().to_string(), test_case.expected);
                }
            };
        }
    }
}
