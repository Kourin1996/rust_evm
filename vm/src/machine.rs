use crate::instruction_set;
use crate::result;
use crate::result::InstructionResult;

pub struct Machine {
    // TODO: stop using trait objects
    pub instruction_sets: Vec<Box<dyn instruction_set::InstructionSet>>,

        pub codes: Vec<u8>,

    pub pc: usize,
}

impl Machine {
    // Input: config
    // Input:
    // pub fn new() -> Machine {
    //     Machine {}
    // }

    // Input: code
    // Input: data
    // Input: stack
    // Input: memory
    // Input: storage
    // Input: config
    pub fn run(&mut self) -> () {
        loop {
            match self.step() {
                InstructionResult::Continue => {
                    continue;
                },
                InstructionResult::Halt => {
                    break;
                }
            }
        }
    }

    fn step(&mut self) -> result::InstructionResult {
        match self.codes.get(self.pc) {
            Some(code) => {
                println!("[Code] {}", code);
                self.pc += 1;

                InstructionResult::Continue
            },
            None => {
                InstructionResult::Halt
            }
        }
    }
}