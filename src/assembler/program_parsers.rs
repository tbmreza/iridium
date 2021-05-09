use crate::assembler::instruction_parsers::{instruction_one, AssemblerInstruction};
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let instructions = self.instructions.clone();
        instructions
            .iter()
            .fold(Vec::new(), |mut acc, instruction| {
                acc.append(&mut instruction.to_bytes());
                acc
            })
    }
}
named!(
    pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction_one) >>
        (Program {
            instructions,
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let result = program(CompleteStr("load $0 #100\n"));
        let (rest, p) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(1, p.instructions.len());
        println!("{:?}", p.instructions);
    }

    #[test]
    fn test_program_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert!(result.is_ok());
        let (_, program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
