use std::collections::VecDeque;

use rust_decimal::{Decimal, MathematicalOps};

use crate::{error::Error, token::Function, Token};

pub fn evaluate_rpn(queue: Vec<Token>) -> Result<Decimal, Error> {
    let mut stack: VecDeque<Decimal> = VecDeque::with_capacity(queue.len());
    for token in queue {
        if let Token::Operator(op) = token {
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
            stack.push_back(n);
        } else {
            return Err(Error::UnexpectedToken);
        }
    }
    if stack.len() != 1 {
        return Err(Error::UnfinishedExpr);
    }
    Ok(stack.pop_back().unwrap())
}
