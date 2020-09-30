use crate::token::*;

use std::iter::Peekable;
use std::str::Chars;

extern crate itertools;

#[derive(Debug, Clone)]
pub struct ScanState<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn get_next_token_type<'a>(
    c: char,
    mut iter: &mut itertools::MultiPeek<Chars<'a>>,
    mut current: &mut usize,
    mut line: &mut usize,
) -> Result<TokenType, usize> {
    let mut tern = |on: char, then: TokenType, other: TokenType| {
        *current += 1;
        if iter.next() == Some(on) {
            then
        } else {
            other
        }
    };

    Ok(match c {
        ' ' | '\r' | '\t' => get_next_token_type(
            iter.next().ok_or::<usize>(5)?,
            &mut iter,
            &mut current,
            &mut line,
        )?,
        '\n' => {
            *line += 1;
            get_next_token_type(
                iter.next().ok_or::<usize>(6)?,
                &mut iter,
                &mut current,
                &mut line,
            )?
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
                iter.next();
                while let Some(c) = iter.next() {
                    if c == '\n' {
                        return get_next_token_type('\n', &mut iter, &mut current, &mut line);
                    }
                    *current += 1;
                }
                return Err(8);
            } else {
                TokenType::Slash
            }
        }
        '"' => {
            let mut res = String::new();
            while let Some(&c) = iter.peek() {
                if c == '"' {
                    iter.reset_peek();
                    break;
                }

                if c == '\n' {
                    *line += 1;
                }
                res.push(c);
                iter.next();
            }

            // closing quote missing
            if iter.peek() == None {
                panic!();
            }

            // skip closing quote
            iter.next();

            TokenType::Text(res)
        }
        c if is_digit(c) => {
            let mut res = c.to_string();
            while let Some(&next_c) = iter.peek() {
                if !is_digit(next_c) {
                    iter.reset_peek();
                    break;
                }

                res.push(next_c);
                iter.next();
            }

            if iter.peek() == Some(&'.') && is_digit(*iter.peek().unwrap_or(&'x')) {
                iter.next();
                res.push('.');

                while let Some(&next_c) = iter.peek() {
                    if !is_digit(next_c) {
                        iter.reset_peek();
                        break;
                    }

                    res.push(next_c);
                    iter.next();
                }
            }

            if let Ok(f) = res.parse::<f64>() {
                TokenType::Number(f)
            } else {
                panic!();
                // todo return an Error token here
            }
        }
        c if is_alpha(c) => {
            let mut res = c.to_string();
            while let Some(&c) = iter.peek() {
                if !(is_alpha(c) || is_digit(c)) {
                    iter.reset_peek();
                    break;
                }

                res.push(c);
                iter.next();
            }

            if let Some(keyword_token) = keyword_to_token_type(&res) {
                keyword_token
            } else {
                TokenType::Identifier(res)
            }
        }
        // todo handle unsupported characters with an error type or something
        _ => return Err(0),
    })
}

fn scan_token<'a>(
    c: char,
    mut iter: &mut itertools::MultiPeek<Chars<'a>>,
    mut current: &mut usize,
    mut line: &mut usize,
) -> Result<Token, usize> {
    Ok(Token {
        token_type: get_next_token_type(c, &mut iter, &mut current, &mut line)?,
        line: line.clone(),
    })
}

pub fn scan_tokens<'a>(input_chars: Chars<'a>) -> Result<Vec<Token>, usize> {
    let mut input_mpeek = itertools::multipeek(input_chars);

    let mut current = 0;
    let mut line = 1;

    let mut out = vec![];
    while let Some(c) = input_mpeek.next() {
        out.push(scan_token(c, &mut input_mpeek, &mut current, &mut line)?)
    }
    Ok(out)
}
