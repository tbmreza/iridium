use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GTE,
    LTE,
    LT,
    GT,
    JMPE,
    ALOC,
    INC,
    IGL,
}

enum V<'a> {
    Word(&'a str),
    Int(u8),
}

fn match_opcode(v: V) -> Opcode {
    match v {
        V::Int(0) | V::Word("load") => Opcode::LOAD,
        V::Int(1) | V::Word("add") => Opcode::ADD,
        V::Int(2) | V::Word("sub") => Opcode::SUB,
        V::Int(3) | V::Word("mul") => Opcode::MUL,
        V::Int(4) | V::Word("div") => Opcode::DIV,
        V::Int(5) | V::Word("hlt") => Opcode::HLT,
        V::Int(6) | V::Word("jmp") => Opcode::JMP,
        V::Int(7) | V::Word("jmpf") => Opcode::JMPF,
        V::Int(8) | V::Word("jmpb") => Opcode::JMPB,
        V::Int(9) | V::Word("eq") => Opcode::EQ,
        V::Int(10) | V::Word("neq") => Opcode::NEQ,
        V::Int(11) | V::Word("gte") => Opcode::GTE,
        V::Int(12) | V::Word("lte") => Opcode::LTE,
        V::Int(13) | V::Word("lt") => Opcode::LT,
        V::Int(14) | V::Word("gt") => Opcode::GT,
        V::Int(15) | V::Word("jmpe") => Opcode::JMPE,
        V::Int(17) | V::Word("aloc") => Opcode::ALOC,
        V::Int(18) | V::Word("inc") => Opcode::INC,
        _ => Opcode::IGL,
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        match_opcode(V::Word(v.0))
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match_opcode(V::Int(v))
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
