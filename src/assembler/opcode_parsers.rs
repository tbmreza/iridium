use super::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;
use nom::{alpha1, do_parse, tag_no_case};

nom::named!(pub opcode<CompleteStr, Token>,
    do_parse!(
        opcode: alpha1 >>
        ({Token::Op {
            code: Opcode::from(opcode)}
        })
    )
);

nom::named!(pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag_no_case!("load") >>
        (Token::Op {
            code: Opcode::LOAD
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode() {
        let result = opcode_load(CompleteStr("load"));
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(token, Token::Op { code: Opcode::LOAD });

        let result = opcode_load(CompleteStr("LOAD"));
        assert!(result.is_ok());
        let (rest, token) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(token, Token::Op { code: Opcode::LOAD });

        let result = opcode_load(CompleteStr("notload"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_opcode() {
        let result = opcode(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));
        let result = opcode(CompleteStr("aold"));
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::IGL });
    }
}
