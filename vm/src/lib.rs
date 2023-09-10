pub mod machine;

pub trait EVM {
    fn run(&self) -> ();
}