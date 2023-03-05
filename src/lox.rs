use log::info;
use rustyline::error::ReadlineError;
pub use scanner::TokenType;
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use self::scanner::Token;

enum RunningMode {
    File(String),
    REPL,
}
pub struct Lox {
    mode: RunningMode,
    scanner: scanner::Scanner,
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
                            log_file
                                .write_all(
                                    format!(
                                        "{:?}",
                                        self.scanner
                                            .scan_line(line)
                                            .unwrap()
                                            .into_iter()
                                            .filter(|tok| match tok.typ {
                                                TokenType::EOF
                                                | TokenType::WhiteSpace
                                                | TokenType::NewLine => false,
                                                _ => true,
                                            })
                                            .collect::<Vec<&Token>>()
                                    )
                                    .as_bytes(),
                                )
                                .unwrap();
                        }
                        Err(e) => break Err(LoxError::ReadError(e)),
                    }
                }
            }
        }
    }

    fn run(source: String) -> Result<(), scanner::ScannerError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum LoxError {
    ReadError(ReadlineError),
    ScannerError(scanner::ScannerError),
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

mod scanner {

    type LoxSourceTraverser = SourceTraverser<TokenType, ScannerError>;

    use crate::utils::{SourceTraverser, Tokenizer};

    pub struct Scanner {
        source: LoxSourceTraverser,
        tokens: Vec<Token>,
    }

    impl Scanner {
        pub fn new(source: String) -> Self {
            Scanner {
                source: SourceTraverser::new(&source),
                tokens: vec![],
            }
        }

        pub fn scan_line(&mut self, line: String) -> Result<&Vec<Token>, ScannerError> {
            self.source = SourceTraverser::new(&line);
            self.tokens = vec![];
            let tokens = self.scan_tokens();
            return tokens;
        }

        pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
            while !self.source.is_finished() {
                self.tokens
                    .push(match self.source.get_next_token(scan_token) {
                        Ok(token_type) => {
                            Token::new(token_type, self.source.get_lexeme(), self.source.line)
                        }
                        Err(e) => return Err(e),
                    });
            }

            self.tokens
                .push(Token::new(TokenType::EOF, "".to_owned(), self.source.line));
            self.source = SourceTraverser::new("");
            return Ok(&self.tokens);
        }

        fn is_finished(&self) -> bool {
            self.source.next_peek().is_none()
        }
    }

    #[derive(Debug)]
    pub enum TokenType {
        WhiteSpace,
        NewLine,

        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,

        StringLiteral(String),
        NumericLiteral(String),

        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        Identifier,
        String,
        Number,

        And,
        Class,
        Elde,
        False,
        Fun,
        For,
        If,
        Nil,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Var,
        While,
        EOF,
    }

    #[derive(Debug)]
    pub struct Token {
        pub typ: TokenType,
        pub lexeme: String,
        // literal: Any ??,
        pub line: u64,
    }

    impl Token {
        pub fn new(typ: TokenType, lexeme: String, line: u64) -> Self {
            Token { typ, lexeme, line }
        }
    }

    #[derive(Debug)]
    pub struct ScannerError {
        line: u64,
        location: String,
        message: String,
    }

    impl std::error::Error for ScannerError {}

    impl std::fmt::Display for ScannerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "[Line: {}], Error {}: {}",
                self.line, self.location, self.message
            )
        }
    }

    fn scan_token(
        source: LoxSourceTraverser,
    ) -> (Result<TokenType, ScannerError>, LoxSourceTraverser) {
        let mut source = source;
        let curr_char = match source.next() {
            Some(next) => next,
            None => return (Ok(TokenType::EOF), source),
        };

        let mut matches_next = |to_match: char, next_type: TokenType, notmatch_type: TokenType| {
            if let Some(next) = &mut source.next_peek() {
                if *next == to_match {
                    let _ = &mut source.next();
                    next_type
                } else {
                    notmatch_type
                }
            } else {
                notmatch_type
            }
        };

        match curr_char {
            '(' => (Ok(TokenType::LeftParen), source),
            ')' => (Ok(TokenType::RightParen), source),
            '{' => (Ok(TokenType::LeftBrace), source),
            ',' => (Ok(TokenType::Comma), source),
            '.' => (Ok(TokenType::Dot), source),
            '-' => (Ok(TokenType::Minus), source),
            '+' => (Ok(TokenType::Plus), source),
            ';' => (Ok(TokenType::Semicolon), source),
            '*' => (Ok(TokenType::Star), source),
            '!' => (
                Ok(matches_next('=', TokenType::BangEqual, TokenType::Bang)),
                source,
            ),
            '=' => (
                Ok(matches_next('=', TokenType::EqualEqual, TokenType::Equal)),
                source,
            ),
            '<' => (
                Ok(matches_next('=', TokenType::LessEqual, TokenType::Less)),
                source,
            ),
            '>' => (
                Ok(matches_next('=', TokenType::GreaterEqual, TokenType::Greater)),
                source,
            ),
            '/' => (
                Ok(if source.next_peek().unwrap() == '/' {
                    source.next();
                    loop {
                        if let Some(next) = source.next_peek() {
                            if next == '\n' {
                                break TokenType::WhiteSpace;
                            }
                            source.next();
                        } else {
                            break TokenType::WhiteSpace;
                        }
                    }
                } else {
                    TokenType::Slash
                }),
                source,
            ),
            ' ' | '\r' | '\t' => {
                loop {
                    if let Some(next) = source.next_peek() {
                        match next {
                            ' ' | '\r' | '\t' => source.next(),
                            _ => break,
                        };
                    } else {
                        break;
                    }
                }
                (Ok(TokenType::WhiteSpace), source)
            }
            '\n' => {
                source.line += 1;
                (Ok(TokenType::NewLine), source)
            }
            '"' => (
                Ok({
                    loop {
                        if let Some(next) = source.next_peek() {
                            if next != '"' {
                                source.next();
                            } else {
                                let ret = TokenType::StringLiteral(source.get_lexeme());
                                source.next();
                                break ret;
                            }
                        } else {
                            break TokenType::EOF;
                        }
                    }
                }),
                source,
            ),
            _ => (
                Err(ScannerError {
                    line: source.line,
                    location: source.get_lexeme(),
                    message: "Unexpected Character.".to_owned(),
                }),
                source,
            ),
        }
    }
}
