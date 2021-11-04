use crate::errors::{self, DisallowedCharError};
use crate::position::Position;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Int(i64),
    Float(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow, // ¤ <- Symbol
    LParen,
    RParen,
    String(String),
    Invalid,
    EndOfFile,
    Identifier(String),
    Keyword(String),
    Equal,
}

pub enum Keyword {
    Func(),
}

impl Default for TokenType {
    fn default() -> Self {
        Self::Int(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    type_: TokenType, // Also holds values
    position_start: Option<Position>,
    position_end: Option<Position>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            type_: TokenType::default(),
            position_start: None,
            position_end: None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.type_)
    }
}

impl Token {
    pub fn new(
        type_: TokenType,
        position_start: Option<Position>,
        position_end: Option<Position>,
    ) -> Self {
        Self {
            type_,
            position_start,
            position_end,
        }
    }

    pub fn new_no_pos(type_: TokenType) -> Self {
        Self {
            type_,
            position_start: None,
            position_end: None,
        }
    }

    pub fn type_(&self) -> TokenType {
        self.type_.clone()
    }

    pub fn position_start(&self) -> Option<Position> {
        self.position_start.clone()
    }

    pub fn position_end(&self) -> Option<Position> {
        self.position_end.clone()
    }
}
