use crate::scanner::Scanner;

pub(crate) struct RLox {
    pub(crate) had_error: bool,
}

impl RLox {
    pub(crate) fn new() -> RLox {
        RLox { had_error: false }
    }

    pub(crate) fn run(&self, source: &str) {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
    }

    pub(crate) fn error(line: u32, message: &str) -> bool {
        RLox::report(line, "", message)
    }

    fn report(line: u32, location: &str, message: &str) -> bool {
        println!("[line {}] Error {} : {}", line, location, message);
        true
    }
}
