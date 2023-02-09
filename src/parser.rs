use super::Instruction;

/// Internal function that parses a program string and produces a `Vec`tor of `Instruction`s
/// Not meant for public use but marked as public just in case a user needs it.
pub fn parse<T: AsRef<str>>(code: T) -> Vec<Instruction> {
    let mut program = vec![];
    let mut loop_starts = vec![];
    let code_chars = code.as_ref().chars();
    for c in code_chars {
        let inst = match c {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            '+' => Instruction::Add,
            '-' => Instruction::Sub,
            '.' => Instruction::Out,
            ',' => Instruction::In,
            '[' => {
                loop_starts.push(program.len());
                Instruction::LoopStart(0)
            }
            ']' => {
                let start = loop_starts
                    .pop()
                    .expect(format!("Unexpected \']\' @{}", program.len()).as_str());
                program[start] = Instruction::LoopStart(program.len());
                Instruction::LoopEnd(start)
            }
            _ => continue,
        };
        program.push(inst);
    }
    if program.len() <= 0 {
        panic!("Not valid BF program.");
    }
    match loop_starts.pop() {
        Some(i) => panic!("\'[\' with no matching \']\' @{i}"),
        None => (),
    }
    program
}
