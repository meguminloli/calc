pub mod op;
pub mod token;
pub mod lexer;
pub mod rpn;
pub mod execute;

pub use rpn::shunting_yard;
pub use lexer::parse_str;
pub use token::Token;

#[cfg(test)]
mod test {
    use crate::token::FUNCTIONS;

    #[test]
    fn test_parser1() {
        use super::Token;
        use super::op::OPERATORS;
        let s = "1 + 2 * 3 + (1 + 4)";
        let tokens = super::parse_str(s);
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(2.0),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(3.0),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::ParLeft,
                Token::Number(1.0),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(4.0),
                Token::ParRight,
            ]
        );
    }
    #[test]
    fn test_parser2() {
        use super::Token;
        use super::op::OPERATORS;
        let s = "sin ( max ( 2, 3 ) ÷ 3 × π )";
        let tokens = super::parse_str(s);
        assert_eq!(
            tokens,
            vec![
                Token::Function(FUNCTIONS.get("sin").unwrap().clone()),
                Token::ParLeft,
                Token::Function(FUNCTIONS.get("max").unwrap().clone()),
                Token::ParLeft,
                Token::Number(2.0),
                Token::Number(3.0),
                Token::ParRight,
                Token::Operator(*OPERATORS.get(&'/').unwrap()),
                Token::Number(3.0),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(3.141592653589793),
                Token::ParRight,
            ]
        );
    }
    #[test]
    fn gen_rpn1() {
        use super::Token;
        use super::op::OPERATORS;
        let s = "1 + 2 * 3 + 1 + 4 ^ 2";
        let tokens = super::parse_str(s);
        let rpn = super::shunting_yard(tokens);
        assert_eq!(
            rpn,
            vec![
                Token::Number(1.0),
                Token::Number(2.0),
                Token::Number(3.0),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(1.0),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(4.0),
                Token::Number(2.0),
                Token::Operator(*OPERATORS.get(&'^').unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
            ]
        );
    }
    #[test]
    fn gen_rpn2() {
        use super::Token;
        use super::op::OPERATORS;
        let s = "1.2 - 2 * 3 * 1.2e5 ^ 4 * 2";
        let tokens = super::parse_str(s);
        let rpn = super::shunting_yard(tokens);
        assert_eq!(
            rpn,
            vec![
                Token::Number(1.2),
                Token::Number(2.0),
                Token::Number(3.0),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(1.2e5),
                Token::Number(4.0),
                Token::Operator(*OPERATORS.get(&'^').unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(2.0),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Operator(*OPERATORS.get(&'-').unwrap()),
            ]
        );
    }

    #[test]
    fn test_calculate1() {
        let s = "1 + 2 * 3 + 1 + 4 ^ 2";
        let rpn = super::shunting_yard(super::parse_str(s));
        assert_eq!(crate::execute::execute_rpn(rpn), 24.0);
    }
    #[test]
    fn test_calculate2() {
        let s = "sin ( max ( 2, 3 ) ÷ 3 × π )";
        let rpn = super::shunting_yard(super::parse_str(s));
        assert_eq!(crate::execute::execute_rpn(rpn), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_panic1() {
        let s = "1 + 2 * 3 + 1 + 4 amogus";
        super::parse_str(s);
    }

    #[test]
    #[should_panic]
    fn test_panic2() {
        let s = "1 2 + * 3 + )1 + 4";
        super::shunting_yard(super::parse_str(s));
    }
    #[test]
    #[should_panic]
    fn test_panic3() {
        let s = "1 2 3 + 4";
        super::execute::execute_rpn(super::shunting_yard(super::parse_str(s)));
    }
}
