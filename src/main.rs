use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn parse_program() -> std::io::Result<String> {
    // Prints each argument on a separate line
    if env::args().len() != 2 {
        panic!("Expected exactly one argument!");
    }

    let file = File::open(env::args().nth(1).unwrap())?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}


fn main() -> std::io::Result<()> { 
    let text = parse_program()?;

    Ok(())
}