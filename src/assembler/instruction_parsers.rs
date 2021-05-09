use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AssemblerInstruction {
    label: Option<&'static str>,
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    fn extract_operand_bytes(t: Option<Token>, results: &mut Vec<u8>) {
        match t {
            None | Some(Token::Op { .. }) => {
                println!("Non-operand in operand field");
                std::process::exit(1);
            }
            Some(Token::Register { reg_num }) => {
                results.push(reg_num);
            }
            Some(Token::IntegerOperand { value }) => {
                let (byte1, byte2) = {
                    let converted = value as u16;
                    (converted, converted >> 8)
                };
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = Vec::new();
        match &self.opcode {
            Token::Op { code } => results.push(*code as u8),
            _ => {
                println!("Non-opcode in opcode field");
                std::process::exit(1);
            }
        }

        let operands = vec![&self.operand1, &self.operand2, &self.operand3];
        // let mut operands = vec![self.operand1, self.operand2, self.operand3];
        operands
            .iter()
            .filter(|operand| operand.is_some()) 
            .map(|operand| **operand)
            .for_each(|t| {
                AssemblerInstruction::extract_operand_bytes(t, &mut results);
            });

        results
    }
}

named!(
    pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        // opcode: opcode_load >>
        opcode: opcode >>
        r: register >>
        i: integer_operand >>
        (AssemblerInstruction {
            label: None,
            opcode,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        let result = instruction_one(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result.unwrap(),
            (
                CompleteStr(""),
                AssemblerInstruction {
                    label: None,
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None,
                }
            )
        );
    }
}
