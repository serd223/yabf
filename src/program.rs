use super::{parse, Instruction};

pub enum ProgramStatus {
    Run,
    Exit,
}

/// A structure that contains the program logic and the program counter.
#[derive(Default)]
pub struct Program {
    instructions: Vec<Instruction>,
    pub counter: usize,
    len: usize,
}

impl Program {
    /// Returns the current instruction. Panics if there are no instructions or the program counter is out of bounds.
    pub fn current(&self) -> &Instruction {
        if self.len == 0 {
            panic!("Called .current() on Program but .len is zero.")
        }
        &self.instructions[self.counter.min(self.len - 1)]
    }

    /// Increases the program counter, returns `ProgramStatus::Exit` if the program counter is >= to the length of the program.
    /// Otherwise, returns `ProgramStatus::Run`
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
