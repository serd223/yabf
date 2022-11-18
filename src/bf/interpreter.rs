use std::io::Write;

use crate::bf::Instruction;

pub struct BfInstance<const MEMSIZE: usize> {

    mem_ptr: usize,
    mem: [u8; MEMSIZE],

    program_counter: usize,
    program: Vec<Instruction>,
    program_len: usize
}

impl<const MEMSIZE: usize> Default for BfInstance<MEMSIZE> {
    fn default() -> Self {
        Self {
            mem_ptr: 0,
            mem: [0; MEMSIZE],
            program_counter: 0,
            program: vec![],
            program_len: 0
        }
    }
}

impl<const MEMSIZE: usize> From<Vec<Instruction>> for BfInstance<MEMSIZE> {
    fn from(v: Vec<Instruction>) -> Self {
        Self {
            program_len: v.len(),
            program: v,
            ..Default::default()
        }
    }
}

impl<const MEMSIZE: usize> BfInstance<MEMSIZE> {
    pub fn step(&mut self) -> bool {
        match self.program[self.program_counter] {
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
                    format!("Couldn't flush stdout @{}", self.program_counter).as_str()
                );
            },
            Instruction::In => {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).expect(
                    format!("Couldn't read user input @{}", self.program_counter).as_str()
                );
                let c = s.chars().nth(0).unwrap();
                self.mem[self.mem_ptr] = c as u8;
            },
            Instruction::LoopEnd(l) => if self.mem[self.mem_ptr] > 0 {
                self.program_counter = l;
            },
            _ => ()
        };
        self.program_counter += 1;
        if self.program_counter >= self.program_len {
            return false;
        }
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}