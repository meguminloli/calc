use std::collections::HashMap;

use once_cell::sync::Lazy;

/// BinOp enum
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// A binary operator struct
/// Operators are used to represent binary operations
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

// An operator is less than another if it has a lower precedence
// than the other operator.
// An operator is greater than another if it has a higher precedence
// than the other operator.
// An operator is equal to another if it has the same precedence
// as the other operator.
impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.precedence.cmp(&other.precedence))
    }
}

// Add the operators to the map
// Add a few unicode characters to flex on cniles
/// Map that contains all binary operators
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
