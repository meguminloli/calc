use std::collections::VecDeque;

use crate::{token::Function, Token};

pub fn execute_rpn(queue: Vec<Token>) -> f64 {
    let mut stack: VecDeque<f64> = VecDeque::with_capacity(queue.len());
    for token in queue {
        if let Token::Operator(op) = token {
            let first = stack.pop_back().unwrap();
            let second = stack.pop_back().unwrap();
            use crate::op::BinOp;
            let result = match op.op {
                BinOp::Add => first + second,
                BinOp::Sub => first - second,
                BinOp::Div => first / second,
                BinOp::Mul => first * second,
                BinOp::Pow => first.powf(second),
            };
            stack.push_back(result);
        } else if let Token::Function(f) = token {
            match f {
                Function::OneParam(f) => {
                    let arg = stack.pop_back().unwrap();
                    let a = f(arg);
                    stack.push_back(a);
                }
                Function::TwoParam(f) => {
                    let first = stack.pop_back().unwrap();
                    let second = stack.pop_back().unwrap();
                    let a = f(first, second);
                    stack.push_back(a);
                }
            }
        } else if let Token::Number(n) = token {
            stack.push_back(n);
        } else {
            panic!("Unexpected token");
        }
        println!("{:?}", stack);
    }
    if stack.len() != 1 {
        panic!("Wrong expression");
    }
    stack.pop_back().unwrap()
}
