use std::io::Write;

use super::{Instruction, Program, ProgramStatus};

pub struct BfInstance<const MEMSIZE: usize> {

    mem_ptr: usize,
    mem: [u8; MEMSIZE],

    program: Program
}

impl<const MEMSIZE: usize> Default for BfInstance<MEMSIZE> {
    fn default() -> Self {
        Self {
            mem_ptr: 0,
            mem: [0; MEMSIZE],
            program: Program::default()
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
    pub fn step(&mut self) -> ProgramStatus {
        match self.program.current() {
            Instruction::Add => self.mem[self.mem_ptr] += 1,
            Instruction::Sub => self.mem[self.mem_ptr] -= 1,
            Instruction::Right => {
                self.mem_ptr += 1;
                if self.mem_ptr >= MEMSIZE {
                    self.mem_ptr = 0;
                }
            },
            Instruction::Left => {
                if self.mem_ptr <= 0 {
                    self.mem_ptr = MEMSIZE - 1;
                } else {
                    self.mem_ptr -= 1;
                }
            },
            Instruction::Out => {
                print!("{}", self.mem[self.mem_ptr] as char);
                std::io::stdout().flush().expect(
                    format!("Couldn't flush stdout @{}", self.program.counter).as_str()
                );
            },
            Instruction::In => {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect(
                    format!("Couldn't read user input @{}", self.program.counter).as_str()
                );
                let c = s.chars().nth(0).unwrap();
                self.mem[self.mem_ptr] = c as u8;
            },
            Instruction::LoopEnd(l) => if self.mem[self.mem_ptr] > 0 {
                self.program.counter = *l;
            },
            Instruction::LoopStart(l) => if self.mem[self.mem_ptr] <= 0 {
                self.program.counter = *l;
            }
        };
        self.program.step()
    }

    pub fn run(&mut self) {
        loop {
            match self.step() {
                ProgramStatus::Exit => break,
                _ => ()
            }
        }
    }
}