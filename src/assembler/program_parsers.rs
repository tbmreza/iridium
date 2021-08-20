use super::instruction_parsers::{instruction, AssemblerInstruction};
use super::SymbolTable;
use nom::types::CompleteStr;
use nom::{do_parse, many1};

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
    pub instructions: Vec<AssemblerInstruction<'a>>,
}

impl<'a> Program<'a> {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let instructions = self.instructions.clone();
        instructions
            .iter()
            .fold(Vec::new(), |mut acc, instruction| {
                acc.append(&mut instruction.to_bytes(symbols));
                acc
            })
    }
}
nom::named!(
    pub program<CompleteStr, Program>,
    do_parse!(
        instructions: many1!(instruction) >>
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
        let symbols = SymbolTable::new();
        let bytecode = program.to_bytes(&symbols);
        assert_eq!(bytecode.len(), 4);
        println!("{:?}", bytecode);
    }
}
