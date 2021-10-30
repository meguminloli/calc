use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::op::Operator;

pub static CONSTANTS: Lazy<HashMap<&'static str, f64>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("e", std::f64::consts::E);
    map.insert("pi", std::f64::consts::PI);
    map.insert("π", std::f64::consts::PI);
    map
});

#[derive(PartialEq, Debug, Clone)]
pub enum Function {
    OneParam(fn(f64) -> f64),
    TwoParam(fn(f64, f64) -> f64),
}

pub static FUNCTIONS: Lazy<HashMap<&str, Function>> = Lazy::new(||{
    let mut map = HashMap::new();
    map.insert("sin", Function::OneParam(f64::sin));
    map.insert("cos", Function::OneParam(f64::cos));
    map.insert("tan", Function::OneParam(f64::tan));
    map.insert("ctan", Function::OneParam(|f: f64| f.cos() / f.sin()));
    map.insert("max", Function::TwoParam(f64::max));
    map.insert("min", Function::TwoParam(f64::min));
    map
});

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(f64),
    ParLeft,
    ParRight,
    Operator(Operator),
    Function(Function),
}
