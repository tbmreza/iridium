use crate::assembler::label_parsers::label_declaration;
use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::{integer_operand, operand};
use crate::assembler::register_parsers::register;
use crate::assembler::Token;
use nom::types::CompleteStr;
#[derive(Debug, PartialEq, Clone, Copy)]
// #[derive(Debug, PartialEq, Clone)]
pub struct AssemblerInstruction<'a> {
    // label: Option<&'static str>,
    pub label: Option<Token<'a>>,
    pub directive: Option<Token<'a>>,
    pub opcode: Option<Token<'a>>,
    pub operand1: Option<Token<'a>>,
    pub operand2: Option<Token<'a>>,
    pub operand3: Option<Token<'a>>,
}

impl<'a> AssemblerInstruction<'a> {
    fn extract_operand_bytes(t: Token, results: &mut Vec<u8>) {
        match t {
            Token::Op { .. } => {
                println!("Non-operand in operand field");
                std::process::exit(1);
            }
            Token::Register { reg_num } => {
                results.push(reg_num);
            }
            Token::IntOperand { value } => {
                let (byte1, byte2) = {
                    let converted = value as u16;
                    (converted, converted >> 8)
                };
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                unimplemented!();
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = Vec::new();
        match &self.opcode {
            Some(Token::Op { code }) => results.push(*code as u8),
            _ => {
                println!("Non-opcode in opcode field");
                std::process::exit(1);
            }
        }

        let operands = vec![&self.operand1, &self.operand2, &self.operand3];

        for operand in operands {
            if let Some(t) = operand {
                AssemblerInstruction::extract_operand_bytes(*t, &mut results);
            }
        }

        results
    }
}

named!(
    // Zero args: hlt
    instruction_0<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        opt!(nom::multispace) >>
        (AssemblerInstruction {
            label: None,
            directive: None,
            opcode: Some(opcode),
            operand1: None,
            operand2: None,
            operand3: None,
        })
    )
);

named!(
    // Two args: load $0 #100
    instruction_2<CompleteStr, AssemblerInstruction>,
    do_parse!(
        // opcode: opcode_load >>
        opcode: opcode >>
        r: register >>
        i: integer_operand >>
        (AssemblerInstruction {
            label: None,
            directive: None,
            opcode: Some(opcode),
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        })
    )
);

named!(
    // Three args: add $0 $1 $2
    instruction_3<CompleteStr, AssemblerInstruction>,
    do_parse!(
        opcode: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        (AssemblerInstruction {
            label: None,
            directive: None,
            opcode: Some(opcode),
            operand1: Some(r1),
            operand2: Some(r2),
            operand3: Some(r3),
        })
    )
);

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

named!(
    pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            // instruction_3 | instruction_2 | instruction_0
            instruction_combined
        ) >>
        ( ins )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_2() {
        let result = instruction_2(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result.unwrap(),
            (
                CompleteStr(""),
                AssemblerInstruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntOperand { value: 100 }),
                    operand3: None,
                }
            )
        );
    }

    #[test]
    fn test_parse_instruction_0() {
        let result = instruction_0(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::HLT }),
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }
}
