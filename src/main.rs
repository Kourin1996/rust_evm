use store::stack::stack::InMemoryStack;
use vm::runner::runner::Runner;

fn main() {
    println!("Hello, World");

    let codes = vec![
        0x60, // PUSH1
        0xF0, // value
        0x60, // PUSH1
        0x01, // value
        0x01, // ADD
        0x00,
    ];

    let mut i = Runner::new(&codes, InMemoryStack::new());

    match i.run() {
        Ok(res) => {
            println!("succeeded with result {:?}", res);
        }
        Err(err) => {
            println!("failed with err {:?}", err);
        }
    }
}
