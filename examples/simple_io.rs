use yabf::*;

const CODE: &str = r#"
>+++++++++[<++++++++>-]<.,+.
"#;

fn main() {
    let mut input_buf = String::from("eeeee");
    let mut program_output = String::new();

    let program = Program::from(CODE);
    let mut bf: BfInstance<256> = BfInstance::from(program);
    bf.run(
        || input_buf.pop(),
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
    println!("\nProgram Output: {program_output}");
}
