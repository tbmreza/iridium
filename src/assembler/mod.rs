use crate::instruction::Opcode;
mod instruction_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
