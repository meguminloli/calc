use std::collections::VecDeque;

use rust_decimal::{Decimal, MathematicalOps};

use crate::{error::Error, token::Function, Token};

/// This function evalutes a rpn expression
/// and returns a result.
/// ## Example
/// ```rust
/// use calc::{execute::evaluate_rpn, token::Token::*, op::OPERATORS};
/// use rust_decimal::prelude::*;
/// 
/// let tokens = vec![Number(Decimal::from_f64(1.0).unwrap()), Number(Decimal::from_f64(2.0).unwrap()), Operator(*OPERATORS.get(&'+').unwrap())];
/// let result = evaluate_rpn(tokens).unwrap();
/// assert_eq!(result, Decimal::from_f64(3.0).unwrap());
/// ```
pub fn evaluate_rpn(queue: Vec<Token>) -> Result<Decimal, Error> {
    let mut stack: VecDeque<Decimal> = VecDeque::with_capacity(queue.len());
    for token in queue {
        if let Token::Operator(op) = token {
            // If there are less than 2 elements, return an error
            if stack.len() < 2 {
                return Err(Error::NotEnoughArgs);
            }
            let first = stack.pop_back().unwrap();
            let second = stack.pop_back().unwrap();
            use crate::op::BinOp;
            let result = match op.op {
                BinOp::Add => first + second,
                BinOp::Sub => second - first,
                BinOp::Div => second / first,
                BinOp::Mul => first * second,
                BinOp::Pow => second.powd(first),
            };
            stack.push_back(result);
        } else if let Token::Function(f) = token {
            // Apply the function to elements from the stack
            match f {
                Function::OneParam(f) => {
                    if stack.is_empty() {
                        return Err(Error::NotEnoughArgs);
                    }
                    let arg = stack.pop_back().unwrap();
                    let a = f(arg);
                    stack.push_back(a);
                }
                Function::TwoParam(f) => {
                    if stack.len() < 2 {
                        return Err(Error::NotEnoughArgs);
                    }
                    let first = stack.pop_back().unwrap();
                    let second = stack.pop_back().unwrap();
                    let a = f(first, second);
                    stack.push_back(a);
                }
            }
        } else if let Token::Number(n) = token {
            // If the token is a number, push it to the stack
            stack.push_back(n);
        } else {
            // The input should contain only Token::Number, Token::Operator and Token::Function
            return Err(Error::UnexpectedToken);
        }
    }
    // If there is not a single elemnt on the stack, then return an error
    if stack.len() != 1 {
        return Err(Error::UnfinishedExpr);
    }
    Ok(stack.pop_back().unwrap())
}
