use std::io::{self, Write};

use crate::vm::VM;

#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL::default()
    }
    pub fn run(&mut self) {
        println!("<welcome_message>");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            match buffer.trim() {
                ".quit" => {
                    println!("exiting");
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid input")
                }
            }
        }
    }
}
