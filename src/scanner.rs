use crate::token::*;

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct ScanState<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

fn scanToken<'a>(
    c: char,
    iter: &mut Peekable<Chars<'a>>,
    current: &mut usize,
    line: &mut usize,
) -> Token {
    let mut tern = |on: char, then: TokenType, other: TokenType| {
        *current += 1;
        if iter.next() == Some(on) {
            then
        } else {
            other
        }
    };

    Token {
        tokenType: match c {
            ' ' | '\r' | '\t' => TokenType::Whitespace(c.to_string()),
            '\n' => {
                *line += 1;
                TokenType::Whitespace(c.to_string())
            }
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,

            '!' => tern('=', TokenType::BangEqual, TokenType::Bang),
            '=' => tern('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => tern('=', TokenType::LessEqual, TokenType::Less),
            '>' => tern('=', TokenType::GreaterEqual, TokenType::Greater),

            '/' => {
                if iter.peek() == Some(&'/') {
                    let mut comment = String::new();
                    iter.next();
                    while let Some(&c) = iter.peek() {
                        if c != '\n' {
                            comment.push(c);
                            iter.next();
                        } else {
                            break
                        }
                        *current += 1;

                    }
                    TokenType::Comment(comment)
                } else {
                    TokenType::Slash
                }
            },
            
            // todo handle unsupported characters with an error type or something
            _ => panic!()
        },
        lexeme: c.to_string(),
        line: line.clone(),
    }
}

pub fn scanTokens<'a>(inputChars: Chars<'a>) -> Vec<Token> {
    let mut inputPeekable = inputChars.peekable();

    let mut current = 0;
    let mut line = 1;

    let mut out = vec![];
    while let Some(c) = inputPeekable.next() {
        out.push(scanToken(c, &mut inputPeekable, &mut current, &mut line))
    }
    out
}
