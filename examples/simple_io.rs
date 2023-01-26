use yabf::*;

const CODE: &str = r#"
>+++++++++[<++++++++>-]<.,+.
"#;

fn main() {
    let program = Program::from(CODE);
    let mut bf: BfInstance<256> = BfInstance::from(program);

    bf.run();
}
