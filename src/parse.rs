use crate::error::Error;
use std::{collections::HashMap, iter::Peekable, str::Chars};

use rust_decimal::{prelude::FromPrimitive, Decimal};

use crate::{
    op::OPERATORS,
    token::{Token, CONSTANTS, FUNCTIONS},
};

const RESTRICTED_CHARS: &[char] = &['(', ')', '+', '-', '*', '×', '/', '÷', '^', ','];

// Parse a number from an iterator of chars
// Kinda unreadable, but it works
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

/// Parses a string and generates a vector of tokens
/// It can take a map of variables to replace them in the expression
/// ## Example
/// ```rust
/// use calc::{parse::parse_str, op::OPERATORS, token::Token};
/// use rust_decimal::prelude::*;
/// let tokens = parse_str("2 + 3", None).unwrap();
/// assert_eq!(
///     tokens,
///     vec![Token::Number(Decimal::from_f64(2.0).unwrap()), Token::Operator(*OPERATORS.get(&'+').unwrap()), Token::Number(Decimal::from_f64(3.0).unwrap())]
/// )
/// ```
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
            // This is helpful for parsing numbers
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
            // Check if the name is a constant, a function or is in the variable map
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
