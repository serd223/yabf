use super::{parse, Instruction};

pub enum ProgramStatus {
    Run,
    Exit,
}

/// A structure that contains the logic of the program.
pub struct Program {
    instructions: Vec<Instruction>,
    pub counter: usize,
    len: usize,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            instructions: vec![],
            counter: 0,
            len: 0,
        }
    }
}

impl Program {
    pub fn current(&self) -> &Instruction {
        &self.instructions[self.counter]
    }
    pub fn step(&mut self) -> ProgramStatus {
        self.counter += 1;
        if self.counter >= self.len {
            return ProgramStatus::Exit;
        }
        ProgramStatus::Run
    }
}

impl<T: AsRef<str>> From<T> for Program {
    fn from(s: T) -> Self {
        let iv = parse(s);
        Self {
            len: iv.len(),
            instructions: iv,
            counter: 0,
        }
    }
}
