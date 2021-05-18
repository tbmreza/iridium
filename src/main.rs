#[macro_use]
extern crate nom;
pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
 
// TODO https://blog.subnetzero.io/post/building-language-vm-interlude-02/
// pad until 32 bits?
// INC, DEC (increment/decrement value in register)