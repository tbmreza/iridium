use super::label_parsers::label_declaration;
use super::opcode_parsers::*;
use super::operand_parsers::{integer_operand, operand};
use super::register_parsers::register;
use super::SymbolTable;
use super::Token;
use byteorder::{LittleEndian, WriteBytesExt};
use nom::types::CompleteStr;
use nom::{alt, do_parse, opt};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AssemblerInstruction<'a> {
    pub label: Option<Token<'a>>,
    pub directive: Option<Token<'a>>,
    pub opcode: Option<Token<'a>>,
    pub operand1: Option<Token<'a>>,
    pub operand2: Option<Token<'a>>,
    pub operand3: Option<Token<'a>>,
}

impl<'a> AssemblerInstruction<'a> {
    pub fn label_name(&self) -> Option<&'a str> {
        // let instruction = self.clone();
        if let AssemblerInstruction {
            label: Some(Token::LabelDecl { name }),
            ..
                // } = instruction
    } = self.to_owned()
    {
        Some(name)
    } else {
        None
    }
    }
    fn extract_operand(t: Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
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
            Token::LabelUsage { name } => {
                if let Some(value) = symbols.symbol_value(name) {
                    let mut wtr = vec![];
                    wtr.write_u32::<LittleEndian>(value).unwrap();
                    results.push(wtr[1]);
                    results.push(wtr[0]);
                } else {
                    panic!("No value found for {:?}", name);
                }
            }
            _ => {
                unimplemented!();
            }
        }
    }

    // pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
    pub fn as_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut results = Vec::new();
        match &self.opcode {
            Some(Token::Op { code }) => results.push(*code as u8),
            _ => {
                println!("Non-opcode in opcode field");
                std::process::exit(1);
            }
        }

        let operands = vec![&self.operand1, &self.operand2, &self.operand3];

        operands.into_iter().flatten().for_each(|t| {
            AssemblerInstruction::extract_operand(*t, &mut results, symbols);
        });

        results
    }
}

nom::named!(
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

nom::named!(
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

nom::named!(
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

nom::named!(instruction_combined<CompleteStr, AssemblerInstruction>,
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

nom::named!(
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
