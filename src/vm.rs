use crate::instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    program: Vec<u8>,
    pc: usize,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    // pub fn get_test_vm() -> VM {
    //     let mut test_vm = VM::new();
    //     test_vm.registers[0] = 5;
    //     test_vm.registers[1] = 10;
    //     test_vm
    // }
    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                // TODO liat di gitlab function ini.
                let i = self.next_8_bits() as usize;
                let number = i32::from(self.next_16_bits());
                // let number = self.next_16_bits() as u16;

                self.registers[i] = number/* as i32*/;
            }
            Opcode::ADD => {
                let (r1, r2, r3) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[r3 as usize] = r1 + r2;
            }
            Opcode::DIV => {
                let (r1, r2, r3) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[r3 as usize] = r1 / r2;
                self.remainder = (r1 % r2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            _ => {
                println!("Unrecognized opcode found. Terminating!");
                return true;
            }
        }

        false
    }

    // pub fn run(&mut self) {
    //     loop {
    //         if self.pc >= self.program.len() {
    //             break;
    //         }
    //         match self.decode_opcode() {
    //             Opcode::LOAD => {
    //                 let i = self.next_8_bits() as usize;
    //                 let number = self.next_16_bits();

    //                 println!("i: {:?}. result: {:?}", i, number);
    //                 self.registers[i] = number as i32;
    //                 continue;
    //             }
    //             Opcode::HLT => {
    //                 println!("HLT encountered");
    //                 return;
    //             }
    //             _ => {
    //                 println!("Unrecognized opcode found. Terminating!");
    //                 return;
    //             }
    //         }
    //     }
    // }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    // fn program_byte_at(&self, index: usize) -> Option<u16> {
    //     if let Some(b) = self.program.get(index) {
    //         Some(*b as u16)
    //     } else {
    //         None
    //     }
    // }

    fn next_16_bits(&mut self) -> u16 {
        // None in this bitwise operation really is zero.
        let result = ((u16::from(self.program[self.pc])) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        result
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[1], 0);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}
