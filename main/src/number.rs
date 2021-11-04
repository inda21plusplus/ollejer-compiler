use crate::context::Context;
use crate::errors::RunTimeError;
use num::{pow::pow, Zero};
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
    pos_start: Option<Position>, // Should this be wrapped in Option
    pos_end: Option<Position>,   // Should this be wrapped in Option
    context: Option<Context>,    // Should this be wrapped in Option
}

impl<T> Number<T> {
    pub fn new_no_pos(value: T) -> Self {
        Self {
            value,
            pos_start: None,
            pos_end: None,
            context: None,
        }
    }

    pub fn new(
        value: T,
        pos_start: Option<Position>,
        pos_end: Option<Position>,
        context: Option<Context>,
    ) -> Self {
        Self {
            value,
            pos_start,
            pos_end,
            context,
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

    pub fn set_context(&mut self, context: Context) {}

    pub fn add(&self, other: Number<T>) -> Self
    where
        T: ops::Add<Output = T> + Clone,
    {
        Self::new_no_pos(self.value.clone() + other.value)
    }

    pub fn sub(&self, other: Number<T>) -> Self
    where
        T: ops::Sub<Output = T> + Clone,
    {
        Self::new_no_pos(self.value.clone() - other.value)
    }

    pub fn mult(&self, other: Number<T>) -> Self
    where
        T: ops::Mul<Output = T> + Clone,
    {
        Self::new_no_pos(self.value.clone() * other.value)
    }

    pub fn pow(&self, other: Number<i64>) -> Result<Self, RunTimeError>
    where
        T: ops::Mul<Output = T> + Clone + num::One,
    {
        if other.value.is_negative() {
            return Err(RunTimeError::new(
                other.pos_start.clone(),
                other.pos_end.clone(),
                "Cant raise to Negative power".to_string(),
                other.context.clone().unwrap(), // Blind unwrap!
            ));
        }
        let val = pow::<T>(self.value.clone(), other.value as usize);
        Ok(Self::new_no_pos(val))
    }

    pub fn div(&self, other: Number<T>) -> Result<Self, RunTimeError>
    where
        T: ops::Div<Output = T> + Clone + Zero,
    {
        if Zero::is_zero(&other.value) {
            return Err(RunTimeError::new(
                other.pos_start.clone(),
                other.pos_end.clone(),
                "Division by Zero".to_string(),
                other.context.clone().unwrap(), // Warning! Blind Unwrap
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
