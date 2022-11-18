
pub enum Instruction {
    Add,
    Sub,
    Left,
    Right,
    Out,
    In,
    LoopStart(usize),
    LoopEnd(usize)
}
