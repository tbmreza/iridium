use super::Token;
use nom::types::CompleteStr;
use nom::{digit, tag, ws};

nom::named!(
    pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg_num: digit >>
            (Token::Register {
                reg_num: reg_num.parse::<u8>().unwrap()
            })
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_register() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("$19"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$a"));
        assert_eq!(result.is_ok(), false);
        let result = register(CompleteStr("$"));
        assert_eq!(result.is_ok(), false);
    }
}
