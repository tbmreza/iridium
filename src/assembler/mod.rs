use crate::instruction::Opcode;
mod instruction_parsers;
mod label_parsers;
mod directive_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;

#[derive(Debug, PartialEq, Clone, Copy)]
// #[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntOperand { value: i32 },
    LabelDecl { name: &'a str },
    LabelUsage { name: &'a str },
    Directive { name: &'a str },
    IrString { name: &'a str },
}
