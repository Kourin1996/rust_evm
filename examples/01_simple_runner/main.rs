use vm;
use eth;

fn main() {
    let mut machine = vm::machine::Machine {
        instruction_sets: vec![],
        codes: vec![1, 2, 3],
        pc: 0,
    };

    machine.run();
}