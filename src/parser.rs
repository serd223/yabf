use super::Instruction;

/// Internal function that parses a program string and produces a `Vec`tor of `Instruction`s
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
                    .unwrap_or_else(|| panic!("Unexpected \']\' @{}", program.len()));
                program[start] = Instruction::LoopStart(program.len());
                Instruction::LoopEnd(start)
            }
            _ => continue,
        };
        program.push(inst);
    }

    if let Some(i) = loop_starts.pop() {
        panic!("\'[\' with no matching \']\' @{i}");
    }

    program
}
