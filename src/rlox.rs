use crate::{
    interpreter::interpret,
    parser::Parser,
    scanner::Scanner,
    token::{self, TokenKind},
};

pub(crate) struct RLox {
    pub(crate) had_error: bool,
}

impl RLox {
    pub(crate) fn new() -> Self {
        RLox { had_error: false }
    }

    pub(crate) fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();

        match expr {
            Ok(expr) => match interpret(expr) {
                Ok(value) => println!("{}", value),
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
