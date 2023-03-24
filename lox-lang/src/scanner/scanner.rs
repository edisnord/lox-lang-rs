type LoxSourceTraverser = SourceTraverser<TokenType, ScannerError>;

use crate::lox::KEYWORDS;

use super::SourceTraverser;
use crate::utils::Tokenizer;

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
        self.source.clear_source_buf();
        return Ok(&self.tokens);
    }
}

#[derive(Debug, Clone)]
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
    NumericLiteral(f64),

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
    Else,
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
    Unknown,
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
    use TokenType::*;

    let mut source = source;
    let curr_char = match source.next() {
        Some(next) => next,
        None => return (Ok(TokenType::EOF), source),
    };

    let mut matches_next = |to_match: char, next_type: TokenType, notmatch_type: TokenType| {
        if *(&mut source.next_peek()) == to_match {
            let _ = &mut source.next();
            next_type
        } else {
            notmatch_type
        }
    };

    let ret = match curr_char {
        '(' => LeftParen,
        ')' => RightParen,
        '{' => LeftBrace,
        '}' => RightBrace,
        ',' => Comma,
        '.' => Dot,
        '-' => Minus,
        '+' => Plus,
        ';' => Semicolon,
        '*' => Star,
        '!' => matches_next('=', BangEqual, Bang),
        '=' => matches_next('=', EqualEqual, Equal),
        '<' => matches_next('=', LessEqual, Less),
        '>' => matches_next('=', GreaterEqual, Greater),
        '/' => {
            if source.next_peek() == '/' {
                source.next();
                loop {
                    if source.next_peek() == '\n' {
                        break WhiteSpace;
                    }
                    source.next();
                }
            } else {
                Slash
            }
        }
        ' ' | '\r' | '\t' => {
            loop {
                match source.next_peek() {
                    ' ' | '\r' | '\t' => source.next(),
                    _ => break,
                };
            }
            //TODO fix
            WhiteSpace
        }
        '\n' => {
            source.line += 1;
            NewLine
        }
        '"' => loop {
            if source.next_peek() != '"' {
                source.next();
            } else {
                let ret = StringLiteral(
                    source
                        .get_lexeme()
                        .chars()
                        .skip(1)
                        .take_while(|x| *x != '"')
                        .collect(),
                );
                source.next();
                break ret;
            }
        },
        '1'..='9' => {
            let mut matched_dec = false;
            loop {
                match source.next_peek() {
                    '.' if !matched_dec && !source.get_lexeme().is_empty() => {
                        source.next();
                        matched_dec = true;
                    }
                    '1'..='9' => {
                        source.next();
                    }
                    _ => break NumericLiteral(source.get_lexeme().parse().unwrap()),
                }
            }
        }
        curr if (curr.is_alphabetic() || curr == '_') => {
            while source.next_peek().is_alphanumeric() || source.next_peek() == '_' {
                source.next();
            }

            KEYWORDS
                .get(&source.get_lexeme())
                .as_deref()
                .unwrap_or(&Identifier)
                .clone()
        }
        _ => Unknown
    };

    match ret {
        Unknown => (
            Err(ScannerError {
                line: source.line,
                location: source.get_lexeme(),
                message: "Unexpected Character.".to_owned(),
            }),
            source,
        ),
        token => (Ok(token), source),
    }
}