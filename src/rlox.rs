use crate::{
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
        scanner.scan_tokens();
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
