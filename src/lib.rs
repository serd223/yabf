pub mod io;
pub use io::BfIO;

mod instructions;
pub use instructions::*;

mod interpreter;
pub use interpreter::*;

mod parser;
use parser::*;

mod program;
pub use program::*;

pub fn default_flush(out: &mut Vec<char>) -> Result<(), ()> {
    while let Some(c) = out.pop() {
        print!("{c}")
    }
    use std::io::Write;
    match std::io::stdout().flush() {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn default_input_source() -> char {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Couldn't read user input.");
    let c: char = s.chars().nth(0).unwrap();
    c
}
