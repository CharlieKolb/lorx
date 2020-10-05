use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod environment;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod value;

const DEFAULT_PROGRAM_PATH: &'static str = "./programs/testProgram.lox";

fn read_program() -> std::io::Result<String> {
    // Prints each argument on a separate line
    // if env::args().len() != 2 {
    //     panic!("Expected exactly one argument!");
    // }

    let path = env::args()
        .nth(1)
        .unwrap_or(DEFAULT_PROGRAM_PATH.to_owned());

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let text = read_program()?;
    let tokens = scanner::scan_tokens(text.chars()); // todo handle tokenization errors
    println!("{:?}", tokens);
    let parse_tree = parser::parse(tokens);
    println!("{:#?}", parse_tree);
    let mut interpreter: interpreter::Interpreter = Default::default();
    for stmt in parse_tree {
        let res = interpreter.evaluate(&stmt);
        if res.is_err() {
            println!("{:#?}", res);
            println!("{:#?}", interpreter);
        }
    }
    println!("{:#?}", interpreter);
    Ok(())
}
