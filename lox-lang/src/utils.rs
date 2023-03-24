use crate::scanner::SourceTraverser;


type STTokenizer<TokenType, ErrorType> = fn(SourceTraverser<TokenType, ErrorType>) 
-> (
    Result<TokenType, ErrorType>,
    SourceTraverser<TokenType, ErrorType>,
);


pub trait Tokenizer {
    type TokenType;
    type ErrorType;
    fn get_next_token( &mut self, tokenize: STTokenizer<Self::TokenType, Self::ErrorType>) 
        -> Result<Self::TokenType, Self::ErrorType>;
}
