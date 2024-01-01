use vm::runner::interpreter::Interpreter;

fn main() {
    println!("Hello, World");

    let mut i = Interpreter::new();

    i.step();
    println!("ip={}", i.get_instruction_pointer());

    i.step();
    println!("ip={}", i.get_instruction_pointer());
}