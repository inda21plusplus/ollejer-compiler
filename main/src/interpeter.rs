use crate::errors::{ErrorType, RunTimeError};
use crate::number::{
    Number,
    NumberType::{self, Float, Integer},
};
use crate::{
    parser::Node::{self, Binop, Unary, Value},
    position::Position,
    token::{Token, TokenType},
};

use std::fmt;
use std::ops;

pub struct Interpeter;

impl Interpeter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visit(&self, node: Node) -> Result<NumberType, ErrorType> {
        match node.clone() {
            Binop(left, op, right) => self.visit_binop_node(node, *left, op, *right),
            Value(val) => self.visit_value_node(val),
            Unary(op, child) => self.visit_unary_node(op, *child),
        }
    }

    fn visit_binop_node(
        &self,
        node: Node,
        left: Node,
        optok: Token,
        right: Node,
    ) -> Result<NumberType, ErrorType> {
        let result_left = self.visit(left);
        let result_right = self.visit(right);

        let left = match result_left {
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        let right = match result_right {
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        match optok.type_() {
            TokenType::Plus => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => Ok(Integer(num1.add(num2))),
                (Float(num1), Float(num2)) => Ok(Float(num1.add(num2))),
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!("Cant add {} with {} due to different types", left, right),
                ))),
            },
            TokenType::Minus => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => Ok(Integer(num1.sub(num2))),
                (Float(num1), Float(num2)) => Ok(Float(num1.sub(num2))),
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!(
                        "Cant subtract {} from {} due to different types",
                        left, right
                    ),
                ))),
            },
            TokenType::Multiply => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => Ok(Integer(num1.mult(num2))),
                (Float(num1), Float(num2)) => Ok(Float(num1.mult(num2))),
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!(
                        "Cant Multiply {} with {} due to different types",
                        left, right
                    ),
                ))),
            },
            TokenType::Divide => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => match num1.div(num2) {
                    Ok(num) => Ok(Integer(num)),
                    Err(e) => return Err(ErrorType::DivisionByZeroError(e)),
                },
                (Float(num1), Float(num2)) => match num1.div(num2) {
                    Ok(num) => Ok(Float(num)),
                    Err(e) => return Err(ErrorType::DivisionByZeroError(e)),
                },
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!("Cant Divide {} with {} due to different types", left, right),
                ))),
            },
            _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                optok.position_start(),
                optok.position_end(),
                format!("Invalid operator token '{}'", optok),
            ))),
        }
    }

    fn check_if_same_numbertype(&self, first: NumberType, other: NumberType) -> bool {
        matches!(
            (first, other),
            (Integer(_), Integer(_)) | (Float(_), Float(_))
        )
    }

    fn visit_value_node(&self, token: Token) -> Result<NumberType, ErrorType> {
        match token.type_() {
            TokenType::Int(val) => Ok(Integer(Number::<i64>::new(
                val,
                token.position_start(),
                token.position_end(),
            ))),
            TokenType::Float(val) => Ok(Float(Number::<f64>::new(
                val,
                token.position_start(),
                token.position_end(),
            ))),
            _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                token.position_start(),
                token.position_end(),
                format!(
                    "Non Value Token {:?} found inside visit value function",
                    token.type_()
                ),
            ))),
        }
    }

    fn visit_unary_node(&self, optok: Token, node: Node) -> Result<NumberType, ErrorType> {
        let result = self.visit(node);

        let number = match result {
            Ok(num) => num,
            Err(e) => return Err(e),
        };

        match optok.type_() {
            TokenType::Minus => match number {
                Integer(num) => Ok(Integer(num.mult(Number::new_no_pos(-1)))),
                Float(num) => Ok(Float(num.mult(Number::new_no_pos(-1.0)))),
            },
            TokenType::Plus => Ok(number),
            _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                optok.position_start(),
                optok.position_end(),
                format!("Invalid operator token '{}'", optok),
            ))),
        }
    }
}
