use crate::{
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
    token::{self, TokenKind},
};
use std::io::Write;

pub struct RLox<'a, W: Write> {
    pub had_error: bool,
    interpreter: Interpreter<'a, W>,
}

impl<'a, W: Write> RLox<'a, W> {
    pub fn new(interpreter: Interpreter<'a, W>) -> Self {
        RLox {
            had_error: false,
            interpreter,
        }
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        match stmts {
            Ok(stmts) => match self.interpreter.interpret(&stmts) {
                Ok(_) => {}
                Err(err) => eprintln!(
                    "{}\n{} ",
                    err.message,
                    err.token
                        .map(|token| format!("[line {}]", token.line))
                        .unwrap_or("".to_string())
                ),
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}

pub fn error_token(token: &token::Token, message: &str) -> bool {
    if token.kind == TokenKind::Eof {
        report(token.line, " at end", message)
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message)
    }
}

pub fn error_line(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) -> bool {
    println!("[line {}] Error {} : {}", line, location, message);
    true
}
