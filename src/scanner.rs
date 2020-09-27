use crate::token::*;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scanTokens(mut self) -> Vec<Token> {
        while !self.isAtEnd() {
            self.start = self.current;
            self.scanToken();
        }

        self.tokens.push(Token {
            tokenType: TokenType::Eof,
            lexeme: "",
            line: self.line,
        });
        self.tokens
    }

    fn scanToken(&mut self) {}

    fn isAtEnd(&self) -> bool {
        self.source.len() <= self.current
    }
}
