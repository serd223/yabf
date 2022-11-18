

use super::Instruction;

pub fn parse<T: AsRef<str>>(code: T) -> Vec<Instruction> {
    let mut program = vec![];
    let mut loop_starts = vec![];
    let code_chars = code.as_ref().chars();
    for (i, c) in code_chars.enumerate() {
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
                let start = loop_starts.pop().expect(
                    format!("Unexpected \']\' @char{}", i).as_str()
                );
                program[start] = Instruction::LoopStart(program.len()); 
                Instruction::LoopEnd(start)
            },
            _ => continue
        };
        program.push(inst);
    }
    if program.len() <= 0 {
        panic!("Not valid BF program.");
    }
    match loop_starts.pop() {
        Some(i) => panic!("\'[\' with no matching \']\' @{i}"),
        None => ()
    }
    program
}
