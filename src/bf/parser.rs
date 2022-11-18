
pub struct Parser {
    loop_starts: Vec<usize>
}

use crate::bf::Instruction;
impl Parser {
    pub fn new() -> Self {
        Self {
            loop_starts: vec![]
        }
    }
    pub fn parse<T: AsRef<str>>(&mut self, code: T) -> Vec<Instruction> {
        let mut program = vec![];

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
                    self.loop_starts.push(program.len());
                    Instruction::LoopStart
                }
                ']' => Instruction::LoopEnd(self.loop_starts.pop().expect(
                    format!("Unexpected \']\' @{}", i).as_str()
                )),
                _ => continue
            };
            program.push(inst);
        }
        if self.loop_starts.len() > 0 {
            panic!("\'[\' with no matching \']\'")
        }
        program
    }
}