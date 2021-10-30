use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::op::Operator;

use rust_decimal::prelude::*;

// TODO: Add more constants
/// List of inbuilt constants
pub static CONSTANTS: Lazy<HashMap<&'static str, Decimal>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("e", Decimal::E);
    map.insert("pi", Decimal::PI);
    map.insert("π", Decimal::PI);
    map
});

/// A function enum for keeping track of the different types of functions
/// with one or two parameters (for now).
#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    OneParam(fn(Decimal) -> Decimal),
    TwoParam(fn(Decimal, Decimal) -> Decimal),
}

// TODO: Add more functions
/// List of inbuilt functions
pub static FUNCTIONS: Lazy<HashMap<&str, Function>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("sin", Function::OneParam(|f| f.sin()));
    map.insert("cos", Function::OneParam(|f| f.cos()));
    map.insert("tan", Function::OneParam(|f| f.tan()));
    map.insert("ctan", Function::OneParam(|f| f.cos() / f.sin()));
    map.insert("max", Function::TwoParam(Decimal::max));
    map.insert("min", Function::TwoParam(Decimal::min));
    map
});

/// A token can be a number, a left parenthesis, a right parenthesis,
/// an operator, a function, a constant, or a comma (only for parsing ease).
#[derive(PartialEq, Debug)]
pub enum Token {
    Number(Decimal),
    ParLeft,
    ParRight,
    Operator(Operator),
    Function(Function),
    Comma,
}
