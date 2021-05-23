use std::str;

use nom::types::CompleteStr;

use crate::instruction::Opcode;

use self::program_parsers::Program;
mod directive_parsers;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;

#[derive(Debug, Default)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbol_table: SymbolTable,
}

#[derive(Debug)]
pub enum AssemblerPhase {
    First,
    Second,
}

impl Default for AssemblerPhase {
    fn default() -> Self {
        AssemblerPhase::First
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

impl Default for SymbolType {
    fn default() -> Self {
        SymbolType::Label
    }
}

#[derive(Debug, Default)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: &str, symbol_type: SymbolType, offset: u32) -> Self {
        Symbol {
            name: name.to_string(),
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler::default()
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program_parsers::program(CompleteStr(raw)) {
            Ok((_rest, p)) => {
                self.phase1_extract_labels(&p);
                self.phase = AssemblerPhase::Second;
                Some(self.phase2_process(&p))
            }
            Err(e) => {
                println!("Error assembling the code: {:?}", e);
                None
            }
        }
    }

    fn phase1_extract_labels(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if let Some(label_name) = i.label_name() {
                let s = Symbol::new(label_name, SymbolType::Label, c);
                self.symbol_table.symbols.push(s);
            }
            c += 4;
        }
    }

    fn phase2_process(&mut self, p: &Program) -> Vec<u8> {
        let mut assembled = Vec::new();
        for i in &p.instructions {
            let mut instruction = i.to_bytes();
            assembled.append(&mut instruction);
        }
        assembled
    }
}

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
