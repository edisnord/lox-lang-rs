#![feature(trace_macros)]

use clap::{command, Parser};
use rustyline::error::ReadlineError;
use lox::Lox;

mod lox;
mod scanner;
mod utils;

/// Lox implementation in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path of the script to run
    #[arg(short, long)]
    script: Option<String>,
}

fn main() {
    let args = Args::parse();
    if let Some(path) = args.script {
        let lox = Lox::new_file(&path).unwrap();
    } else {
        let mut lox = Lox::new_prompt();
        match lox.start() {
            Ok(_) => (),
            Err(lox::LoxError::ReadError(ReadlineError::Interrupted | ReadlineError::Eof)) => (),
            Err(e) => {
                
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tree_builder::ast_gen;
    ast_gen!();

    #[test]
    fn aaaugh() {
        Asdf::Asdf1(12);
    }
}

