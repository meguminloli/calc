use crate::error::Error;
use std::{collections::HashMap, iter::Peekable, str::Chars};

use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{
    op::OPERATORS,
    token::{Token, CONSTANTS, FUNCTIONS},
};

const RESTRICTED_CHARS: &[char] = &['(', ')', '+', '-', '*', '×', '/', '÷', '^', ','];

fn parse_number(c: char, iter: &mut Peekable<Chars>) -> Result<Decimal, Error> {
    let mut num = String::new();
    num.push(c);
    while let Some(c) = iter.peek() {
        let c = *c;
        if c.is_numeric() || c == '.' {
            num.push(c);
            iter.next();
        } else if c == 'e' {
            iter.next();
            if matches!(iter.peek(), Some(&'+') | Some(&'-'))
                || matches!(iter.peek(), Some(x) if (&'0'..=&'9').contains(&x))
            {
                num.push(c);
                num.push(iter.next().unwrap());
            } else {
                return Err(Error::NumberParseError);
            }
        } else {
            break;
        }
    }
    let num = num.parse::<f64>()?;
    Ok(Decimal::from_f64(num).unwrap())
}

pub fn parse_str(
    s: &str,
    variables: Option<&HashMap<String, Decimal>>,
) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = s.chars().peekable();
    while let Some(c) = iter.next() {
        if c.is_whitespace() {
            continue;
        } else if c == ',' {
            tokens.push(Token::Comma);
        } else if c.is_numeric()
            || matches!(c, '+' | '-')
                && matches!(
                    tokens.last(),
                    Some(Token::Comma) | None | Some(Token::Operator(_)) | Some(Token::ParLeft)
                )
        {
            let num = parse_number(c, &mut iter)?;
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
                if !(c.is_numeric()
                    || c == &'e'
                    || c == &'.'
                    || RESTRICTED_CHARS.contains(c)
                    || c.is_whitespace())
                {
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
            } else if let Some(variables) = variables {
                if let Some(v) = variables.get(&string) {
                    tokens.push(Token::Number(*v));
                } else {
                    return Err(Error::InvalidToken);
                }
            } else {
                return Err(Error::InvalidToken);
            }
        }
    }
    Ok(tokens)
}
