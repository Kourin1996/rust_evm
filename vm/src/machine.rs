use crate::EVM;

pub struct Machine {}

impl Machine {
    pub fn new() -> Machine {
        Machine {}
    }
}

impl EVM for Machine{
    fn run(&self) -> () {
        println!("Hello, EVM!!");
    }
}