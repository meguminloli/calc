use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Copy, Clone, Debug)]
pub struct Operator {
    pub(crate) op: BinOp,
    precedence: u8,
}

impl Operator {
    const fn new(op: BinOp, precedence: u8) -> Self {
        Self { op, precedence }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.precedence == other.precedence
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.precedence.cmp(&other.precedence))
    }
}

pub static OPERATORS: Lazy<HashMap<char, Operator>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert('+', Operator::new(BinOp::Add, 2));
    map.insert('-', Operator::new(BinOp::Sub, 2));
    map.insert('*', Operator::new(BinOp::Mul, 3));
    map.insert('/', Operator::new(BinOp::Div, 3));
    map.insert('^', Operator::new(BinOp::Pow, 4));
    // Unicode stuff
    map.insert('×', Operator::new(BinOp::Mul, 3));
    map.insert('÷', Operator::new(BinOp::Div, 3));
    map
});
