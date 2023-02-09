use yabf::*;

const CODE: &str = r#"
>+++++++++[<++++++++>-]<.,+.
"#;

fn main() {
    let mut input_buf = String::from("eeeee");
    let mut program_output = String::new();

    let program = Program::from(CODE);
    let mut bf: BfInstance<256> = BfInstance::from(program);
    let mut io = BfIO::new(
        || input_buf.pop().unwrap(),
        |out| {
            while let Some(c) = out.pop() {
                print!("{c}");
                program_output.push(c);
            }
            use std::io::Write;
            match std::io::stdout().flush() {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        },
    );
    bf.run(&mut io);
    println!("\nProgram Output: {program_output}");
}
