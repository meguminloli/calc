use std::collections::VecDeque;

use crate::{error::Error, op::BinOp, token::Token};

/// Shunting-yard algorithm for converting infix to postfix
/// Check https://en.wikipedia.org/wiki/Shunting-yard_algorithm for details
/// ## Examples
/// ```
/// use calc::{token::Token, rpn::shunting_yard, op::OPERATORS};
/// use rust_decimal::prelude::*;
/// let infix = vec![Token::Number(Decimal::from_f64(1.0).unwrap()), Token::Operator(*OPERATORS.get(&'+').unwrap()), Token::Number(Decimal::from_f64(2.0).unwrap())];
/// let postfix = shunting_yard(infix).unwrap();
/// assert_eq!(postfix, vec![Token::Number(Decimal::from_f64(1.0).unwrap()), Token::Number(Decimal::from_f64(2.0).unwrap()), Token::Operator(*OPERATORS.get(&'+').unwrap())]);
/// ```
pub fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut operator_stack = VecDeque::new();
    for token in tokens {
        match token {
            // If the token is a number, then add it to the output queue.
            Token::Number(n) => output.push(Token::Number(n)),
            // If the token is a function, then push it onto the operation stack.
            Token::Function(_) => operator_stack.push_back(token),
            // If the token is a left parenthesis, then push it onto the operation stack.
            Token::ParLeft => operator_stack.push_back(token),
            Token::Operator(op1) => {
                while !operator_stack.is_empty() {
                    let token = operator_stack.back().unwrap();
                    if token == &Token::ParLeft {
                        break;
                    } else if let Token::Operator(op2) = token {
                        let op2 = *op2;
                        if op1 < op2 || (op1 == op2 && op1.op != BinOp::Pow) {
                            output.push(operator_stack.pop_back().unwrap());
                        } else {
                            break;
                        }
                    } else if let Token::Function(_) = token {
                        output.push(operator_stack.pop_back().unwrap());
                    } else {
                        return Err(Error::InvalidToken);
                    }
                }
                operator_stack.push_back(token);
            }
            // If the token is a right parenthesis, then
            Token::ParRight => {
                let mut ok = false;
                // While the operator token at the top of the stack is not a left parenthesis,
                // pop operators off the stack onto the output queue.
                while let Some(token) = operator_stack.pop_back() {
                    if token == Token::ParLeft {
                        ok = true;
                        break;
                    } else {
                        output.push(token);
                    }
                }
                // If there is no left parenthesis on the stack, return an error.
                if !ok {
                    return Err(Error::UnbalancedParens);
                }
            }
            // If the token is a comma, discard it
            Token::Comma => (),
        }
    }
    while let Some(token) = operator_stack.pop_back() {
        if token == Token::ParLeft {
            return Err(Error::UnbalancedParens);
        }
        output.push(token);
    }
    Ok(output)
}
