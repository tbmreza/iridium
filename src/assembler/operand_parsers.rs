use super::label_parsers::label_usage;
use super::register_parsers::register;
use super::Token;
use nom::types::CompleteStr;
use nom::{alt, digit, do_parse, tag, take_until, ws};

nom::named!(pub irstring<CompleteStr, Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        (Token::IrString {
            name: &content
        })
    )
);

nom::named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register |
        irstring
    )
);

nom::named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (Token::IntOperand {
                value: reg_num.parse::<i32>().unwrap()
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#10"));
        assert!(result.is_ok());

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntOperand { value: 10 });

        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }
}
