pub mod instruction;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
    // TODO https://blog.subnetzero.io/post/building-language-vm-part-06/
    // Command History
}
