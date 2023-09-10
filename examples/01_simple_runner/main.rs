extern crate vm;

use vm::EVM;

fn main() {
    let evm: vm::machine::Machine = vm::machine::Machine::new();

    evm.run();
}