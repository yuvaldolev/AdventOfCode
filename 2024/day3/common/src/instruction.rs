#[derive(Debug)]
pub enum Instruction {
    Multiply(u32, u32),
    Enable,
    Disable,
}
