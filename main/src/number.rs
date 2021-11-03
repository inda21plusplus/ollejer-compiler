use crate::errors::DivisionByZeroError;
use num::Zero;
use std::{cmp, fmt, ops};

use crate::position::Position;
#[derive(Debug, Clone, PartialEq)]
pub enum NumberType {
    Integer(Number<i64>),
    Float(Number<f64>),
}

impl fmt::Display for NumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberType::Integer(num) => write!(f, "{}", num.value()),
            NumberType::Float(num) => write!(f, "{}", num.value()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number<T> {
    value: T,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
}

impl<T> Number<T> {
    pub fn new_no_pos(value: T) -> Self {
        Self {
            value,
            pos_start: None,
            pos_end: None,
        }
    }

    pub fn new(value: T, pos_start: Option<Position>, pos_end: Option<Position>) -> Self {
        Self {
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.value.clone()
    }
    pub fn set_pos(&mut self, pos_start: Option<Position>, pos_end: Option<Position>) {
        self.pos_start = pos_start;
        self.pos_end = pos_end;
    }

    pub fn add(&self, other: Number<T>) -> Self
    where
        T: ops::Add<Output = T>,
        T: Clone,
    {
        Self::new_no_pos(self.value.clone() + other.value)
    }

    pub fn sub(&self, other: Number<T>) -> Self
    where
        T: ops::Sub<Output = T>,
        T: Clone,
    {
        Self::new_no_pos(self.value.clone() - other.value)
    }

    pub fn mult(&self, other: Number<T>) -> Self
    where
        T: ops::Mul<Output = T>,
        T: Clone,
    {
        Self::new_no_pos(self.value.clone() * other.value)
    }

    pub fn div(&self, other: Number<T>) -> Result<Self, DivisionByZeroError>
    where
        T: ops::Div<Output = T> + Clone + Zero,
    {
        if Zero::is_zero(&other.value) {
            return Err(DivisionByZeroError::new(
                self.pos_start.clone(),
                self.pos_end.clone(),
                "Cant divide by 0".to_string(),
            ));
        }
        Ok(Self::new_no_pos(self.value.clone() / other.value))
    }
}

impl<T> fmt::Display for Number<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
