use super::stack::{Stack, StackElement, StackError};

#[derive(Debug)]
pub struct Hub<S>
where
    S: Stack {
    stack: S,
}

impl<S> Hub<S>
    where
    S: Stack {
    pub fn new(stack: S) -> Self {
        Self {
            stack: stack,
        }
    }

    pub fn push_to_stack(&mut self, data: StackElement) -> Result<(), StackError> {
        self.stack.push(data)
    }

    pub fn pop_from_stack(&mut self) -> Result<StackElement, StackError> {
        self.stack.pop()
    }
}