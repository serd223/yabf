/// Enum to keep track of Brainfuck Instructions
pub enum Instruction {
    Add,
    Sub,
    Left,
    Right,
    Out,
    In,
    /// Contains the index of its matching LoopEnd
    LoopStart(usize),
    /// Contains the index of its matching LoopStart
    LoopEnd(usize),
}
