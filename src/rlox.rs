use crate::{
    interpreter::Interpreter,
    parser::Parser,
    scanner::Scanner,
    token::{self, TokenKind},
};

pub(crate) struct RLox {
    pub(crate) had_error: bool,
    interpreter: Interpreter,
}

impl RLox {
    pub(crate) fn new(interpreter: Interpreter) -> Self {
        RLox {
            had_error: false,
            interpreter,
        }
    }

    pub(crate) fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let stmts = parser.parse();

        match stmts {
            Ok(stmts) => match self.interpreter.interpret(stmts) {
                Ok(_) => {}
                Err(err) => eprintln!("{}\n[line {}] ", err.message, err.token.line),
            },
            Err(err) => eprintln!("{}", err),
        }
    }

    pub(crate) fn error_token(token: &token::Token, message: &str) -> bool {
        if token.kind == TokenKind::Eof {
            RLox::report(token.line, " at end", message)
        } else {
            RLox::report(token.line, &format!(" at '{}'", token.lexeme), message)
        }
    }

    pub(crate) fn error_line(line: usize, message: &str) {
        RLox::report(line, "", message);
    }

    fn report(line: usize, location: &str, message: &str) -> bool {
        println!("[line {}] Error {} : {}", line, location, message);
        true
    }
}
