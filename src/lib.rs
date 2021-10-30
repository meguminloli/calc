pub mod error;
pub mod execute;
pub mod op;
pub mod parse;
pub mod rpn;
pub mod token;

pub use error::Error;
pub use execute::evaluate_rpn;
pub use parse::parse_str;
pub use rpn::shunting_yard;
use token::Token;

#[cfg(test)]
mod test {
    use rust_decimal::{prelude::FromPrimitive, Decimal};

    use crate::token::FUNCTIONS;

    #[test]
    fn test_parser1() {
        use super::op::OPERATORS;
        use super::Token;
        let s = "1 + 2 * 3 + (1 + 4)";
        let tokens = super::parse_str(s, None).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(Decimal::from_f64(1.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(Decimal::from_f64(3.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::ParLeft,
                Token::Number(Decimal::from_f64(1.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(Decimal::from_f64(4.0).unwrap()),
                Token::ParRight,
            ]
        );
    }
    #[test]
    fn test_parser2() {
        use super::op::OPERATORS;
        use super::Token;
        let s = "sin ( max ( 2, 3 ) ÷ 3 × π )";
        let tokens = super::parse_str(s, None).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Function(FUNCTIONS.get("sin").unwrap().clone()),
                Token::ParLeft,
                Token::Function(FUNCTIONS.get("max").unwrap().clone()),
                Token::ParLeft,
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Comma,
                Token::Number(Decimal::from_f64(3.0).unwrap()),
                Token::ParRight,
                Token::Operator(*OPERATORS.get(&'/').unwrap()),
                Token::Number(Decimal::from_f64(3.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(Decimal::PI),
                Token::ParRight,
            ]
        );
    }
    #[test]
    fn test_variables() {
        use super::op::OPERATORS;
        use super::Token;
        use std::collections::HashMap;
        let s = "x + 3";
        let tokens = super::parse_str(
            s,
            Some(&HashMap::from([(
                "x".to_string(),
                Decimal::from_f64(1.0).unwrap(),
            )])),
        )
        .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(Decimal::from_f64(1.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(Decimal::from_f64(3.0).unwrap()),
            ]
        );
    }
    #[test]
    fn gen_rpn1() {
        use super::op::OPERATORS;
        use super::Token;
        let s = "1 + 2 * 3 + 1 + 4 ^ 2";
        let tokens = super::parse_str(s, None).unwrap();
        let rpn = super::shunting_yard(tokens).unwrap();
        assert_eq!(
            rpn,
            vec![
                Token::Number(Decimal::from_f64(1.0).unwrap()),
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Number(Decimal::from_f64(3.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(Decimal::from_f64(1.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
                Token::Number(Decimal::from_f64(4.0).unwrap()),
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'^').unwrap()),
                Token::Operator(*OPERATORS.get(&'+').unwrap()),
            ]
        );
    }
    #[test]
    fn gen_rpn2() {
        use super::op::OPERATORS;
        use super::Token;
        let s = "1.2 - 2 * 3 * 1.2e5 ^ 4 * 2";
        let tokens = super::parse_str(s, None).unwrap();
        let rpn = super::shunting_yard(tokens).unwrap();
        assert_eq!(
            rpn,
            vec![
                Token::Number(Decimal::from_f64(1.2).unwrap()),
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Number(Decimal::from_f64(3.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(Decimal::from_f64(1.2e5).unwrap()),
                Token::Number(Decimal::from_f64(4.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'^').unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Number(Decimal::from_f64(2.0).unwrap()),
                Token::Operator(*OPERATORS.get(&'*').unwrap()),
                Token::Operator(*OPERATORS.get(&'-').unwrap()),
            ]
        );
    }

    #[test]
    fn test_calculate1() {
        let s = "1 + 2 * 3 + 1 + 4 ^ 2";
        let rpn = super::shunting_yard(super::parse_str(s, None).unwrap()).unwrap();
        assert_eq!(
            crate::execute::evaluate_rpn(rpn).unwrap(),
            Decimal::from_f64(24.0).unwrap()
        );
    }
    #[test]
    fn test_calculate2() {
        let s = "sin ( max ( 2, 3 ) ÷ 3 × π )";
        let rpn = super::shunting_yard(super::parse_str(s, None).unwrap()).unwrap();
        assert_eq!(
            crate::execute::evaluate_rpn(rpn).unwrap(),
            Decimal::from_f64(0.0).unwrap()
        );
    }
    #[test]
    #[should_panic]
    fn test_panic1() {
        let s = "1 + 2 * 3 + 1 + 4 amogus";
        super::parse_str(s, None).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_panic2() {
        let s = "1 2 + * 3 + )1 + 4";
        super::shunting_yard(super::parse_str(s, None).unwrap()).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_panic3() {
        let s = "1 2 3 + 4";
        super::execute::evaluate_rpn(
            super::shunting_yard(super::parse_str(s, None).unwrap()).unwrap(),
        )
        .unwrap();
    }
    #[test]
    #[should_panic]
    fn test_panic4() {
        let s = "1 2 3 + 4";
        super::execute::evaluate_rpn(
            super::shunting_yard(super::parse_str(s, None).unwrap()).unwrap(),
        )
        .unwrap();
    }
}
