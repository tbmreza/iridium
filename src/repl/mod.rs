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

            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("exiting");
                    std::process::exit(0);
                }
                ".history" => {
                    &self
                        .command_buffer
                        .iter()
                        .for_each(|command| println!("{}", command));
                }
                _ => {
                    println!("Invalid input")
                }
            }
        }
    }
}
