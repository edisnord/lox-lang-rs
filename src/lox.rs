use rustyline::error::ReadlineError;
use crate::scanner::{*, self};

use std::{
    fmt::{Display, format},
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf::phf_map!(
    "and" => TokenType::And,
    "class" => TokenType::Class,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "super" => TokenType::Super,
    "this" => TokenType::This,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While
);
enum RunningMode {
    File(String),
    REPL,
}
pub struct Lox {
    mode: RunningMode,
    scanner: Scanner,
}

impl Lox {
    pub fn new_file(path: &str) -> std::io::Result<Self> {
        let mut source = String::new();
        File::open(Path::new(path)).and_then(|mut file| file.read_to_string(&mut source))?;
        Ok(Lox {
            mode: (RunningMode::File(source.clone())),
            scanner: scanner::Scanner::new(source),
        })
    }

    pub fn new_prompt() -> Self {
        Lox {
            mode: RunningMode::REPL,
            scanner: scanner::Scanner::new(String::new()),
        }
    }

    pub fn start(&mut self) -> Result<(), LoxError> {
        let mut log_file = File::create("log.txt").unwrap();
        match &self.mode {
            RunningMode::File(source) => Ok(()),
            RunningMode::REPL => {
                let mut rl = rustyline::DefaultEditor::new().unwrap();
                loop {
                    match rl.readline("> ") {
                        Ok(line) => {
                            if line.is_empty() {
                                continue;
                            }
                            let line = line + "\n";
                            match self.scanner.scan_line(line) {
                                Ok(token_list) => {
                                    let output = format!("\n{:?}", token_list
                                        .into_iter()
                                        .filter(|tok| match tok.typ {
                                                    TokenType::EOF
                                                    | TokenType::WhiteSpace
                                                    | TokenType::NewLine => false,
                                                    _ => true,
                                                }).collect::<Vec<&Token>>());
                                    log_file.write_all(output.as_bytes()).unwrap();
                                },
                                Err(scanner_error) => println!("{}", scanner_error),
                            }
                        }
                        Err(e) => break Err(LoxError::ReadError(e)),
                    }
                }
            }
        }
    }

    fn run(source: String) -> Result<(), ScannerError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum LoxError {
    ReadError(ReadlineError),
    ScannerError(ScannerError),
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::ReadError(error) => {
                write!(f, "Lox IO Error: {}", error)
            }
            LoxError::ScannerError(error) => {
                write!(f, "Lox Scanner Error: {}", error)
            }
        }
    }
}

