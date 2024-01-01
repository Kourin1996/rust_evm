#[derive(Debug)]
pub struct Interpreter {
    pub instruction_pointer: u16,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0
        }
    }

    pub fn get_instruction_pointer(&self) -> u16 {
        self.instruction_pointer
    }

    pub fn step(&mut self) {
        self.instruction_pointer += 1;
    }
}

