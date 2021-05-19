use crate::assembler::instruction_parsers::AssemblerInstruction;
use crate::assembler::label_parsers::label_declaration;
use crate::assembler::operand_parsers::operand;
use crate::assembler::Token;
use nom::alpha1;
use nom::types::CompleteStr;

named!(directive_declaration<CompleteStr, Token>,
  do_parse!(
      tag!(".") >>
      name: alpha1 >>
      (
        Token::Directive{name: &name }
      )
  )
);

named!(directive_combined<CompleteStr, AssemblerInstruction>,
    ws!(
        do_parse!(
            l: opt!(label_declaration) >>
            name: directive_declaration >>
            o1: opt!(operand) >>
            o2: opt!(operand) >>
            o3: opt!(operand) >>
            (
                AssemblerInstruction{
                    opcode: None,
                    directive: Some(name),
                    label: l,
                    operand1: o1,
                    operand2: o2,
                    operand3: o3,
                }
            )
        )
    )
);

named!(
    // Will try to parse out any of the Directive forms
    pub directive<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            directive_combined
        ) >>
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests {
    // #![allow(unused_imports)]

    use super::*;
    // use super::{directive_combined, directive_declaration};
    // use crate:assembler::instruction_parsers::AssemblerInstruction;
    // use assembler::Token;
    // use nom::types::CompleteStr;

    #[test]
    fn test_parser_directive() {
        let result = directive_declaration(CompleteStr(".data"));
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();
        assert_eq!(directive, Token::Directive { name: "data" })
    }

    #[test]
    fn test_string_directive() {
        let result = directive_combined(CompleteStr("test: .asciiz 'Hello'"));
        assert_eq!(result.is_ok(), true);
        let (_, directive) = result.unwrap();

        // Yes, this is the what the result should be
        let correct_instruction = AssemblerInstruction {
            opcode: None,
            label: Some(Token::LabelDecl { name: "test" }),
            directive: Some(Token::Directive { name: "asciiz" }),
            operand1: Some(Token::IrString { name: "Hello" }),
            operand2: None,
            operand3: None,
        };

        assert_eq!(directive, correct_instruction);
    }
}
