mod directive_parsers;
mod instruction_parsers;
mod label_parsers;
mod opcode_parsers;
mod operand_parsers;
pub mod program_parsers;
mod register_parsers;

use self::program_parsers::Program;
use super::instruction::Opcode;
use nom::types::CompleteStr;
use std::str;

#[derive(Debug, Default)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
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

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable::default()
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
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
                self.symbols.symbols.push(s);
            }
            c += 4;
        }
    }

    fn phase2_process(&mut self, p: &Program) -> Vec<u8> {
        let mut assembled = Vec::new();
        for i in &p.instructions {
            let mut instruction = i.to_bytes(&self.symbols);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VM;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test", SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert_eq!(v.is_some(), false);
    }

    #[test]
    // #[ignore]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string =
            "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njmpe @test\nhlt";
        let program = asm.assemble(test_string).unwrap();
        let mut vm = VM::new();
        assert_eq!(program.len(), 21);
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 21);
    }
}
