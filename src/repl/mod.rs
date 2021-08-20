use super::assembler::program_parsers::program;
use super::assembler::Assembler;
use super::vm::VM;
use nom::types::CompleteStr;
use std::io::{self, Read, Write};

#[derive(Default)]
pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
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
                ".load_file" => {
                    print!("Enter path to source file:");

                    let mut buf = String::new();
                    let filepath = {
                        stdin
                            .read_line(&mut buf)
                            .expect("unable to read line from user");
                        let buf = buf.trim();
                        std::path::Path::new(buf)
                    };

                    let mut source = String::new();
                    let mut handle = std::fs::File::open(filepath).expect("file not found");
                    handle
                        .read_to_string(&mut source)
                        .expect("error reading from file");

                    match program(CompleteStr(&source)) {
                        Ok((_rest, program)) => {
                            self.vm
                                .program
                                .append(&mut program.to_bytes(&self.asm.symbols));
                        }
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                        }
                    }
                }
                ".clear_program" => {
                    println!("Clearing the following program:");
                    println!("{:?}", &self.vm.program);
                    self.vm.program.clear();
                }
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
                        let bytecode = parsed_program.to_bytes(&self.asm.symbols);
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
