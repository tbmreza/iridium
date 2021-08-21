extern crate clap;
extern crate nom;

#[allow(unused_imports)]
use clap::{load_yaml, App, Arg, SubCommand};
use iridium::{assembler, repl, vm};

fn main() {
    let _cli_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(_cli_config).get_matches();

    match matches.value_of("INPUT_FILE") {
        Some(filename) => {
            let mut asm = assembler::Assembler::new();
            let program = read_file(filename);
            let program = asm.assemble(&program);

            // match program {
            //     Some(p) => {
            //         let mut vm = vm::VM::new();
            //         vm.add_bytes(p);
            //         vm.run();
            //         std::process::exit(0);
            //     }
            //     None => {}
            // }
            if let Some(p) = program {
                let mut vm = vm::VM::new();
                vm.add_bytes(p);
                vm.run();
                std::process::exit(0);
            }
        }
        None => {
            start_repl();
        }
    }
}

fn read_file(tmp: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let filename = Path::new(tmp);
    match File::open(Path::new(&filename)) {
        Ok(mut f) => {
            let mut contents = String::new();
            match f.read_to_string(&mut contents) {
                Ok(_) => {
                    contents
                }
                Err(e) => {
                    println!("There was an error reading file: {:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("File not found: {:?}", e);
            std::process::exit(1)
        }
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

// TODO https://blog.subnetzero.io/post/building-language-vm-interlude-02/
// pad until 32 bits?
