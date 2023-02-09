pub mod io;
pub use io::*;

mod instructions;
pub use instructions::*;

mod interpreter;
pub use interpreter::*;

mod parser;
use parser::*;

mod program;
pub use program::*;

/// Default `flush` function that writes the output to regular stdout.
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

/// Default `input_source` that gets its input from stdin.
pub fn default_input_source() -> Option<char> {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Couldn't read user input.");
    s.chars().next()
}
