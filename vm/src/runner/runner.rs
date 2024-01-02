use super::result::{ExecutionResult, ExecutionError, InstructionResult, InstructionError};
use crate::hub::hub::Hub;
use crate::hub::stack::Stack;

#[derive(Debug)]
pub struct Runner<'a, S>
where
    S: Stack,
{
    codes: &'a [u8],

    pub hub: Hub<S>,

    pub instruction_pointer: usize,
}

impl<'a, S> Runner<'a, S>
where
    S: Stack {
    pub fn new(
        codes: &'a [u8],
        stack: S,
    ) -> Self {
        Self {
            codes: codes,

            hub: Hub::new(
                stack
            ),

            // TODO: Give initial instruction_pointer from args
            instruction_pointer: 0
        }
    }

    pub fn run(&mut self) -> Result<ExecutionResult, ExecutionError> {
        loop {
            match self.step() {
                Ok(res) => {
                    if res == InstructionResult::Continue {
                        continue
                    }

                    return Ok(ExecutionResult::Success);
                },
                Err(err) => {
                    return Err(ExecutionError::Instruction(err));
                }
            }
        }
    }

    pub fn step(&mut self) -> Result<InstructionResult, InstructionError> {
        if self.instruction_pointer >= self.codes.len() {
            // TODO: check error type
            return Err(InstructionError::InvalidOpCode);
        }

        println!("Runner::step ip={} code={:}", self.instruction_pointer, self.codes[self.instruction_pointer]);

        // pre process
        match self.codes[self.instruction_pointer] {
            0x00 => {
                return Ok(InstructionResult::Halt);
            },
            0x01 => {
                // TODO: error handling
                let val1 = self.hub.pop_from_stack().map_err(|e| InstructionError::StackError)?;
                let val2 = self.hub.pop_from_stack().map_err(|e| InstructionError::StackError)?;

                let mut new_val = [0 as u8; 32];
                new_val[31] = val1[31] + val2[31];

                println!("Add {:?} + {:?} = {:?}", val1, val2, new_val);

                self.hub.push_to_stack(new_val);
                self.instruction_pointer += 1;
            },
            0x60 => {
                let value = self.codes[self.instruction_pointer + 1];

                let mut stack_element = [0 as u8; 32];
                stack_element[stack_element.len() - 1] = value;

                println!("push to stack => {:?}", stack_element);

                self.hub.push_to_stack(stack_element);

                self.instruction_pointer += 2;
            },
            _ => {
                self.instruction_pointer += 1;
            }
        }

        // execution

        // post process

        Ok(InstructionResult::Continue)
    }
}

