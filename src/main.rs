use std::env::args;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

mod environment;
mod error;
mod expr;
mod interpreter;
mod parser;
mod rlox;
mod scanner;
mod stmt;
mod token;
mod utils;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::rlox::RLox;

fn main() {
    match args().len() {
        0..=1 => run_prompt(),
        2 => {
            let args: Vec<String> = args().collect();
            run_file(&args[1]);
        }
        _ => {
            println!("Usage: rlox [script]");
            exit(64);
        }
    }
}

fn run_file(file_name: &str) {
    if let Ok(content) = fs::read_to_string(file_name) {
        let environment = Environment::new();
        let interpreter = Interpreter::new(environment);
        let mut rlox = RLox::new(interpreter);
        rlox.run(&content);
        if rlox.had_error {
            exit(65);
        }
    } else {
        eprintln!("Could not open: {file_name}");
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();
    let environment = Environment::new();
    let interpreter = Interpreter::new(environment);
    let mut rlox = RLox::new(interpreter);

    loop {
        print!("> ");
        stdout.flush().unwrap();

        if let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                println!("Exiting...");
                break;
            }

            rlox.run(&line);
            rlox.had_error = false;
        } else {
            eprintln!("Error reading line or EOF reached");
            break;
        }
    }
}
