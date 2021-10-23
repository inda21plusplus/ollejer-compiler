use crate::errors::{self, DisallowedCharError, Error};

#[derive(Debug)]
pub enum Token {
    Int(i64),
    Float(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    String(String),
}
