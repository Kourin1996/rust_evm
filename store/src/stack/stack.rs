use vm::hub::stack::{Stack, StackElement, StackError};

#[derive(Debug)]
pub struct InMemoryStack {
    pub data: Vec<StackElement>,
}

impl InMemoryStack {
    pub fn new() -> Self {
        Self {
            data: vec![],
        }
    }
}

impl Stack for InMemoryStack {
    fn push(&mut self, data: StackElement) -> Result<(), StackError> {
        // todo: limit check
        self.data.push(data);

        Ok(())
    }

    fn pop(&mut self) -> Result<StackElement, StackError> {
        self.data.pop().ok_or(StackError::StackUnderflow)
    }
}