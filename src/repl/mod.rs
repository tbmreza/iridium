use std::io::{self, Write};

use nom::types::CompleteStr;

use crate::assembler::program_parsers::program;
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

    fn parse_hex(&mut self, input: &str) -> Vec<u8> {
        let split = input.split(" ").collect::<Vec<&str>>();

        split
            .iter()
            .filter_map(|s| u8::from_str_radix(s, 16).ok())
            .collect::<Vec<u8>>()
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
                ".program" => {
                    println!("In VM's program vector:");
                    println!("{:?}", &self.vm.program);
                }
                ".registers" => {
                    println!("In VM's registers:");
                    println!("{:?}", &self.vm.registers);
                }
                ".quit" => {
                    println!("exiting");
                    std::process::exit(0);
                }
                ".history" => {
                    println!("{:?}", &self.command_buffer);
                }
                _ => {
                    // // hex speaking repl
                    // self.parse_hex(buffer)
                    //     .iter()
                    //     .for_each(|byte| self.vm.add_byte(*byte));
                    // self.vm.run_once();

                    if let Ok((_rest, parsed_program)) = program(CompleteStr(buffer)) {
                        let bytecode = parsed_program.to_bytes();
                        bytecode.iter().for_each(|byte| self.vm.add_byte(*byte));
                        self.vm.run_once();
                    } else {
                        println!("Unable to parse input");
                    }
                }
            }
        }
    }
}
