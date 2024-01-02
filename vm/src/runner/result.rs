// 1 call level result
#[derive(Debug, PartialEq)]
pub enum ExecutionResult {
    Success = 0x0
}

// 1 call level error
#[derive(Debug)]
pub enum ExecutionError {
    Instruction(InstructionError),
}

// Op-code level result
#[derive(Debug, PartialEq)]
pub enum InstructionResult {
    Continue = 0x0,
    Halt = 0x1,
}

// Op-code level error
#[derive(Debug)]
pub enum InstructionError {
    InvalidOpCode,
    StackError,
}