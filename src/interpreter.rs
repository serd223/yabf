use crate::io::BfIO;

use super::{Instruction, Program, ProgramStatus};

/// A structure that stores everything related to the program.
pub struct BfInstance<const MEMSIZE: usize> {
    pub mem_ptr: usize,
    pub mem: [u8; MEMSIZE],

    pub program: Program,
    pub io_buf: BfIO,
}

impl<const MEMSIZE: usize> Default for BfInstance<MEMSIZE> {
    fn default() -> Self {
        Self {
            mem_ptr: 0,
            mem: [0; MEMSIZE],
            program: Program::default(),
            io_buf: BfIO::default(),
        }
    }
}

impl<const MEMSIZE: usize> From<Program> for BfInstance<MEMSIZE> {
    fn from(p: Program) -> Self {
        Self {
            program: p,
            ..Default::default()
        }
    }
}

impl<const MEMSIZE: usize> BfInstance<MEMSIZE> {
    /// Executes the current instruction and increases the program counter.
    /// If the program couldn't receive an input via `input_source`, the program counter won't increase.
    pub fn step<SOURCE, FLUSH>(
        &mut self,
        mut input_source: SOURCE,
        mut flush: FLUSH,
    ) -> ProgramStatus
    where
        SOURCE: FnMut() -> Option<char>,
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        let mut block = false;
        match self.program.current() {
            Instruction::Add => self.mem[self.mem_ptr] += 1,
            Instruction::Sub => self.mem[self.mem_ptr] -= 1,
            Instruction::Right => {
                self.mem_ptr += 1;
                if self.mem_ptr >= MEMSIZE {
                    self.mem_ptr = 0;
                }
            }
            Instruction::Left => {
                if self.mem_ptr <= 0 {
                    self.mem_ptr = MEMSIZE - 1;
                } else {
                    self.mem_ptr -= 1;
                }
            }
            Instruction::Out => {
                self.io_buf.write_out(self.mem[self.mem_ptr] as char);
            }
            Instruction::In => match self.io_buf.read_in(&mut input_source, &mut flush) {
                Some(c) => self.mem[self.mem_ptr] = c as u8,
                None => block = true,
            },
            Instruction::LoopEnd(l) => {
                if self.mem[self.mem_ptr] > 0 {
                    self.program.counter = *l;
                }
            }
            Instruction::LoopStart(l) => {
                if self.mem[self.mem_ptr] <= 0 {
                    self.program.counter = *l;
                }
            }
        };

        if !block {
            self.program.step()
        } else {
            ProgramStatus::Run
        }
    }

    /// Convenience function that handles stepping/flushing for you.
    /// Not recommended if you need to do extra operations between or during steps.
    pub fn run<SOURCE, FLUSH>(&mut self, mut input_source: SOURCE, mut flush: FLUSH)
    where
        SOURCE: FnMut() -> Option<char>,
        FLUSH: FnMut(&mut Vec<char>) -> Result<(), ()>,
    {
        loop {
            match self.step(&mut input_source, &mut flush) {
                ProgramStatus::Exit => break,
                _ => (),
            }
        }

        self.io_buf
            .flush(flush)
            .expect("Couldn't flush output buffer.");
    }
}
