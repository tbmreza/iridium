use crate::assembler::Token;
use nom::types::CompleteStr;
use nom::{alphanumeric, multispace};

named!(
    // Looks for a user-defined label, such as `label1:`
    pub label_declaration<CompleteStr, Token>,
    ws!(
        do_parse!(
            name: alphanumeric >>
            tag!(":") >>
            opt!(multispace) >>
            (
                Token::LabelDecl{name: &name}
            )
        )
    )
);

named!(
    // Looks for a user-defined label, such as `label1:`
    pub label_usage<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("@") >>
            name: alphanumeric >>
            opt!(multispace) >>
            (
                Token::LabelUsage{name: &name}
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_label_declaration() {
        let result = label_declaration(CompleteStr("test:"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelDecl { name: "test" });
        let result = label_declaration(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_parse_label_usage() {
        let result = label_usage(CompleteStr("@test"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::LabelUsage { name: "test" });
        let result = label_usage(CompleteStr("test"));
        assert_eq!(result.is_ok(), false);
    }

}
