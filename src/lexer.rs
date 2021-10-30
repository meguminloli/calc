use crate::{
    op::OPERATORS,
    token::{Token, CONSTANTS, FUNCTIONS},
};

const RESTRICTED_CHARS: &[char] = &['(', ')', '+', '-', '*', '×', '/', '÷', '^', ' ', '\n', '\t'];

pub fn parse_str(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_whitespace() || c == ',' {
            continue;
        } else if c.is_numeric() {
            // TODO: check the validity of number
            // Check if it contains multiple `e` or `e` and `.` at the same time etc.
            let mut string = String::new();
            string.push(c);
            while let Some(c) = iter.peek() {
                if c.is_numeric() || c == &'e' || c == &'.' {
                    string.push(*c);
                    iter.next();
                } else {
                    break;
                }
            }
            let num = string.parse::<f64>().unwrap();
            tokens.push(Token::Number(num));
        } else if c == '(' {
            tokens.push(Token::ParLeft);
        } else if c == ')' {
            tokens.push(Token::ParRight);
        } else if let Some(op) = OPERATORS.get(&c) {
            tokens.push(Token::Operator(*op));
        } else {
            let mut string = String::new();
            string.push(c);
            while let Some(c) = iter.peek() {
                if !(c.is_numeric() || c == &'e' || c == &'.') && !RESTRICTED_CHARS.contains(c) {
                    string.push(*c);
                    iter.next();
                } else {
                    break;
                }
            }
            if let Some(f) = CONSTANTS.get(string.as_str()) {
                tokens.push(Token::Number(*f));
            } else if let Some(fun) = FUNCTIONS.get(&string.as_str()) {
                tokens.push(Token::Function(fun.clone()));
            } else {
                panic!("Unknown token: {}", string);
            }
        }
    }
    tokens
}
