pub type StackElement = [u8; 32];

pub enum StackError {
    StackOverflow,
    StackUnderflow,
}

pub trait Stack {
    fn push(&mut self, data: StackElement) -> Result<(), StackError>;

    fn pop(&mut self) -> Result<StackElement, StackError>;
}