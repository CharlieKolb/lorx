use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

const DEFAULT_PROGRAM_PATH : &'static str = "./programs/helloWorld.lorx";

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


fn main() -> std::io::Result<()> { 
    let text = read_program()?;

    Ok(())
}