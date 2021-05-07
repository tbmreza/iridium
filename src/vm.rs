use crate::instruction::Opcode;

#[derive(Debug, Default)]
pub struct VM {
    pub registers: [i32; 32],
    pub program: Vec<u8>,
    pc: usize,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    /// New VM with registers preset with values.
    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        // test_vm.float_registers[0] = 5.0;
        // test_vm.float_registers[1] = 10.0;
        test_vm
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }
    // pub fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
    //     let mut prepension = vec![];
    //     for byte in &PIE_HEADER_PREFIX {
    //         prepension.push(byte.clone());
    //     }

    //     // The 4 is added here to allow for the 4 bytes that tell the VM where the executable code starts
    //     while prepension.len() < PIE_HEADER_LENGTH + 4 {
    //         prepension.push(0);
    //     }

    //     prepension.append(&mut b);
    //     prepension
    // }

    pub fn run(&mut self) {
        let mut executing = Some(());

        while executing.is_some() {
            executing = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> Option<()> {
        if self.pc >= self.program.len() {
            return None;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let i = self.next_8_bits() as usize;
                let number = i32::from(self.next_16_bits());

                self.registers[i] = number;
            }
            Opcode::ADD => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[self.next_8_bits() as usize] = r1 + r2;
            }
            Opcode::SUB => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[self.next_8_bits() as usize] = r1 - r2;
            }
            Opcode::MUL => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[self.next_8_bits() as usize] = r1 * r2;
            }
            Opcode::DIV => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );

                self.registers[self.next_8_bits() as usize] = r1 / r2;
                self.remainder = (r1 % r2) as u32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return None;
            }
            Opcode::JMP => {
                let r1 = self.registers[self.next_8_bits() as usize];
                self.pc = r1 as usize;
            }
            Opcode::JMPF => {
                let r1 = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += r1;
            }
            Opcode::JMPB => {
                let r1 = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= r1;
            }
            Opcode::EQ => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 == r2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 != r2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 >= r2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 <= r2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 < r2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let (r1, r2) = (
                    self.registers[self.next_8_bits() as usize],
                    self.registers[self.next_8_bits() as usize],
                );
                self.equal_flag = r1 > r2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                if self.equal_flag {
                    let r1 = self.registers[self.next_8_bits() as usize];
                    self.pc = r1 as usize;
                }
            }
            _ => {
                println!("Unrecognized opcode found. Terminating!");
                return None;
            }
        }

        Some(())
    }

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
        let result =
            ((u16::from(self.program[self.pc])) << 8) | u16::from(self.program[self.pc + 1]);
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
    fn test_opcode_eq() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        // test_vm.registers[0] = 10;
        test_vm.registers[1] = 23;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);

        // test_vm.registers[0] = 10;
        test_vm.registers[1] = 23;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }
    #[test]
    fn test_opcode_gte() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.registers[1] = 23;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
    #[test]
    fn test_opcode_lte() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 11;
        test_vm.registers[1] = 23;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }
    #[test]
    fn test_opcode_lt() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 7;
        // test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }
    #[test]
    fn test_opcode_gt() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0];

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[0] = 17;
        // test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
    #[test]
    fn test_opcode_jmpe() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 42, 0, 0, 0, 99, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }
    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 4;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 5, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }
    #[test]
    fn test_opcode_add() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 15);
    }
    #[test]
    fn test_opcode_sub() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![2, 0, 1, 2];
        test_vm.registers[0] = 50;
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 45);
    }
    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.registers[0] = 50;
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 250);
    }
    #[test]
    fn test_opcode_div() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![4, 0, 1, 2];
        test_vm.registers[0] = 50;
        test_vm.registers[1] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 10);
    }

    #[test]
    fn test_opcode_load() {
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

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[31], 0);
        assert_eq!(test_vm.program, vec![]);
        assert_eq!(test_vm.pc, 0);
        assert_eq!(test_vm.remainder, 0);
        assert_eq!(test_vm.equal_flag, false);
    }
}
