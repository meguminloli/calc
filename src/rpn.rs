use std::collections::VecDeque;

use crate::{error::Error, op::BinOp, token::Token};

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, Error> {
    let mut output = Vec::with_capacity(tokens.len());
    let mut operator_stack = VecDeque::new();
    for token in tokens {
        match token {
            Token::Number(n) => output.push(Token::Number(n)),
            Token::Function(_) => operator_stack.push_back(token),
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
            Token::ParRight => {
                let mut ok = false;
                while let Some(token) = operator_stack.pop_back() {
                    if token == Token::ParLeft {
                        ok = true;
                        break;
                    } else {
                        output.push(token);
                    }
                }
                if !ok {
                    return Err(Error::UnbalancedParens);
                }
            }
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
