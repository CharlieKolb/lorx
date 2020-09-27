use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

use std::iter::FromIterator;
use std::collections::HashSet;

const DEFAULT_PROGRAM_PATH : &'static str = "./programs/helloWorld.lorx";

#[macro_use]
extern crate lazy_static;


lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = HashSet::from_iter([ "print", "if", "elif", "else" ].iter().cloned());
    static ref SYMBOLS: HashSet<&'static str> = HashSet::from_iter([ ":", "=", "<", ">", "<=", ">=", "==", "!", "!=", "&", "|", "and", "or" ].iter().cloned());

}

fn read_program() -> std::io::Result<String> {
    // Prints each argument on a separate line
    // if env::args().len() != 2 {
    //     panic!("Expected exactly one argument!");
    // }

    let path = env::args().nth(1).unwrap_or(DEFAULT_PROGRAM_PATH.to_owned());

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

       
#[derive(Debug)]
enum Token {
    Keyword(String),
    Name(String),
    Symbol(String),
}

fn tokenize(program: &String) -> Vec<Token> {
    let mut tokens : Vec<Token> = vec![];
    let mut curr_word = String::new();
    let mut next_escaped = false;
    for c in program.chars() {
        if next_escaped {
            next_escaped = false;
            curr_word.push(c);
            continue
        }

        match c {
            | '\\' => { next_escaped = true; continue }
            | ' ' => {
                if curr_word.len() > 0 {
                    tokens.push(
                        match &curr_word {
                           | word if KEYWORDS.contains(&word.as_str()) => Token::Keyword(word.to_string()),
                           | word if SYMBOLS.contains(&word.as_str()) => Token::Symbol(word.to_string()),
                           | word => Token::Name(word.to_string()) // ToDo: Check for valid name and panic otherwise
                        }
                    );
                    curr_word.clear();
                }
            }
            | _ => { curr_word.push(c) }
        }
    }
    tokens
}


fn main() -> std::io::Result<()> { 
    let text = read_program()?;
    let tokens = tokenize(&text);
    println!("{:?}", tokens);
    Ok(())
}