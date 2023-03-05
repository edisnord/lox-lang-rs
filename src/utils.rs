use std::{cmp::min, marker::PhantomData};

#[derive(Clone)]
pub struct SourceTraverser<TokenType, ErrorType> {
    source: Vec<char>,
    start: usize,
    current: usize,
    pub line: u64,
    pl1: PhantomData<TokenType>,
    pl2: PhantomData<ErrorType>,
}

impl<TokenType, ErrorType> SourceTraverser<TokenType, ErrorType> {
    pub fn new(source: &str) -> Self {
        SourceTraverser {
            source: source.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            pl1: PhantomData,
            pl2: PhantomData,
        }
    }

    pub fn prev_peek(&self) -> Option<char> {
        self.source.get(self.current - 1).as_deref().copied()
    }

    pub fn next_peek(&self) -> Option<char> {
        self.source.get(self.current).as_deref().copied()
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }

    pub fn seek_back(&mut self, skip_by: usize) {
        self.current = min(0, self.current - skip_by)
    }

    pub fn seek_forward(&mut self, skip_by: usize) {
        self.current = min(self.source.len(), self.current + skip_by)
    }

    pub fn is_finished(&self) -> bool {
        self.current == self.source.len()
    }

    pub fn is_at_start(&self) -> bool {
        self.current == 0
    }

    pub fn prev(&mut self) -> Option<char> {
        if !self.is_at_start() {
            self.current -= 1;
        }
        self.current_char()
    }

    pub fn current_char(&self) -> Option<char> {
        self.source.get(self.current).as_deref().copied()
    }

    pub fn get_lexeme(&self) -> String {
        self.source
            .iter()
            .skip(self.start)
            .take(self.current - self.start)
            .collect()
    }
}

impl<TokenType, ErrorType> Iterator for SourceTraverser<TokenType, ErrorType> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current_char();
        if !self.is_finished() {
            self.current += 1;
        }
        ret
    }
}

pub trait Tokenizer {
    type TokenType;
    type ErrorType;
    fn peek_next_token(
        &self,
        tokenize: fn(Self) -> (Result<Self::TokenType, Self::ErrorType>, Self),
    ) -> Result<Self::TokenType, Self::ErrorType>;
    fn get_next_token(
        &mut self,
        tokenize: fn(Self) -> (Result<Self::TokenType, Self::ErrorType>, Self),
    ) -> Result<Self::TokenType, Self::ErrorType>;
}

impl<TokenType, ErrorType> Tokenizer for SourceTraverser<TokenType, ErrorType> {
    type ErrorType = ErrorType;
    type TokenType = TokenType;

    fn peek_next_token(
        &self,
        tokenize: fn(Self) -> (Result<Self::TokenType, Self::ErrorType>, Self),
    ) -> Result<Self::TokenType, Self::ErrorType> {
        tokenize(SourceTraverser::new(
            self.source.to_owned().iter().collect::<String>().as_str(),
        ))
        .0
    }

    fn get_next_token(
        &mut self,
        tokenize: fn(Self) -> (Result<Self::TokenType, Self::ErrorType>, Self),
    ) -> Result<Self::TokenType, Self::ErrorType> {
        self.start = self.current;
        let (token, new_self) = tokenize(Self {
            source: self.source.clone(),
            start: self.start,
            current: self.current,
            line: self.line,
            pl1: PhantomData,
            pl2: PhantomData,
        });
        *self = new_self;
        return token;
    }
}
