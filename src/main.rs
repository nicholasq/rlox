use std::env::args;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

mod expr;
mod parser;
mod rlox;
mod scanner;
mod token;
mod utils;

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
        let rlox = RLox::new();
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
    let mut rlox = RLox::new();

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
