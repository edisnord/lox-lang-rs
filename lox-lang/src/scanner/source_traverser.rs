use std::{marker::PhantomData, vec};

use crate::utils::Tokenizer;

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

    pub fn clear_source_buf(&mut self) {
        self.source = vec![];
    }

    // pub fn prev_peek(&self) -> Option<char> {
    //     self.source.get(self.current - 1).as_deref().copied()
    // }

    pub fn next_peek(&self) -> char {
        match self.source.get(self.current) {
            Some(next) => *next,
            None => '\0',
        }
    }

    // pub fn match_word(&self, word: &str) -> bool {
    //     self.source
    //         .iter()
    //         .skip(self.start)
    //         .take(word.len())
    //         .collect::<String>()
    //         .eq(word)
    // }

    // pub fn reset(&mut self) {
    //     self.current = 0;
    // }

    // pub fn seek_back(&mut self, skip_by: usize) {
    //     self.current = min(0, self.current - skip_by)
    // }

    // pub fn seek_forward(&mut self, skip_by: usize) {
    //     self.current = min(self.source.len(), self.current + skip_by)
    // }

    pub fn is_finished(&self) -> bool {
        self.current == self.source.len()
    }

    // pub fn is_at_start(&self) -> bool {
    //     self.current == 0
    // }

    // pub fn prev(&mut self) -> Option<char> {
    //     if !self.is_at_start() {
    //         self.current -= 1;
    //     }
    //     self.current_char()
    // }

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

impl<TokenType, ErrorType> Tokenizer for SourceTraverser<TokenType, ErrorType> {
    type ErrorType = ErrorType;
    type TokenType = TokenType;

    fn get_next_token(
        &mut self,
        tokenize: fn(Self) -> (Result<Self::TokenType, Self::ErrorType>, Self),
    ) -> Result<Self::TokenType, Self::ErrorType> {
        self.start = self.current;
        let (token, new_self) = tokenize(self.clone());
        *self = new_self;
        return token;
    }
}

impl<TokenType, ErrorType> Clone for SourceTraverser<TokenType, ErrorType> {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            start: self.start,
            current: self.current,
            line: self.line,
            pl1: PhantomData,
            pl2: PhantomData,
        }
    }
}
