use crate::context::Context;
use crate::errors::{ErrorType, RunTimeError};
use crate::lexer::Lexer;
use crate::number::{
    Number,
    NumberType::{self, Float, Integer},
};
use crate::parser::Node::{self, Binop, Unary, Value, VarAccessNode, VarAssignNode};
use crate::parser::Parser;
use crate::position::Position;
use crate::token::{Token, TokenType};

use std::ops;
use std::{clone, collections, fmt};

pub struct Interpeter;

impl Interpeter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn visit(&self, node: Node, context: Context) -> Result<NumberType, ErrorType> {
        match node.clone() {
            Binop(left, op, right) => self.visit_binop_node(node, *left, op, *right, context),
            Value(val) => self.visit_value_node(val, context),
            Unary(op, child) => self.visit_unary_node(op, *child, context),
            VarAssignNode(name, node) => self.visit_varass_node(name, *node, context),
            VarAccessNode(tok) => self.visit_varacc_node(tok, context),
        }
    }

    fn visit_varacc_node(
        &self,
        name_tok: Token,
        context: Context,
    ) -> Result<NumberType, ErrorType> {
        let variable_name = match name_tok.type_() {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(ErrorType::RunTimeError(RunTimeError::new(
                    name_tok.position_start(),
                    name_tok.position_end(),
                    "Invalid Variable name".to_string(),
                    context,
                )))
            }
        };

        let symbol_map = match context.symbol_map() {
            Some(symbols) => symbols,
            None => {
                return Err(ErrorType::RunTimeError(RunTimeError::new(
                    name_tok.position_start(),
                    name_tok.position_end(),
                    "No Symbol Table!".to_string(),
                    context,
                )))
            }
        };

        match symbol_map.get(variable_name.clone()) {
            None => {
                return Err(ErrorType::RunTimeError(RunTimeError::new(
                    name_tok.position_start(),
                    name_tok.position_end(),
                    format!("{} is not defined", variable_name,),
                    context,
                )))
            }
            Some(value) => Ok(value.clone()),
        }
    }

    fn visit_varass_node(
        &self,
        token: Token,
        node: Node,
        context: Context,
    ) -> Result<NumberType, ErrorType> {
        let variable_name = match token.type_() {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(ErrorType::RunTimeError(RunTimeError::new(
                    token.position_start(),
                    token.position_end(),
                    "Invalid Variable namee".to_string(),
                    context,
                )))
            }
        };
        let value = match self.visit(node, context.clone()) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        let mut symbol_map = match context.symbol_map() {
            Some(symbols) => symbols,
            None => {
                return Err(ErrorType::RunTimeError(RunTimeError::new(
                    token.position_start(),
                    token.position_end(),
                    "No Symbol Table!".to_string(),
                    context,
                )))
            }
        };
        let _u = symbol_map.set(variable_name, value.clone());
        return Ok(value);
    }

    fn visit_binop_node(
        &self,
        node: Node,
        left: Node,
        optok: Token,
        right: Node,
        context: Context,
    ) -> Result<NumberType, ErrorType> {
        let result_left = self.visit(left, context.clone());
        let result_right = self.visit(right, context.clone());

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
                    context,
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
                    context,
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
                    context,
                ))),
            },
            TokenType::Divide => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => match num1.div(num2) {
                    Ok(num) => Ok(Integer(num)),
                    Err(e) => return Err(ErrorType::RunTimeError(e)),
                },
                (Float(num1), Float(num2)) => match num1.div(num2) {
                    Ok(num) => Ok(Float(num)),
                    Err(e) => return Err(ErrorType::RunTimeError(e)),
                },
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!("Cant Divide {} with {} due to different types", left, right),
                    context,
                ))),
            },

            // TODO: Add fraction powers and whole powers to floats
            TokenType::Pow => match (left.clone(), right.clone()) {
                (Integer(num1), Integer(num2)) => match num1.pow(num2) {
                    Ok(num) => Ok(Integer(num)),
                    Err(e) => return Err(ErrorType::RunTimeError(e)),
                },
                (Float(num1), Integer(num2)) => match num1.pow(num2) {
                    Ok(num) => Ok(Float(num)),
                    Err(e) => return Err(ErrorType::RunTimeError(e)),
                },
                _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                    optok.position_start(),
                    optok.position_end(),
                    format!("Cant raise {} to {}", left, right),
                    context,
                ))),
            },
            _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                optok.position_start(),
                optok.position_end(),
                format!("Invalid operator token '{}'", optok),
                context,
            ))),
        }
    }

    fn check_if_same_numbertype(&self, first: NumberType, other: NumberType) -> bool {
        matches!(
            (first, other),
            (Integer(_), Integer(_)) | (Float(_), Float(_))
        )
    }

    fn visit_value_node(&self, token: Token, context: Context) -> Result<NumberType, ErrorType> {
        match token.type_() {
            TokenType::Int(val) => Ok(Integer(Number::<i64>::new(
                val,
                token.position_start(),
                token.position_end(),
                Some(context),
            ))),
            TokenType::Float(val) => Ok(Float(Number::<f64>::new(
                val,
                token.position_start(),
                token.position_end(),
                Some(context),
            ))),
            _ => Err(ErrorType::RunTimeError(RunTimeError::new(
                token.position_start(),
                token.position_end(),
                format!(
                    "Non Value Token {:?} found inside visit value function",
                    token.type_()
                ),
                context,
            ))),
        }
    }

    fn visit_unary_node(
        &self,
        optok: Token,
        node: Node,
        context: Context,
    ) -> Result<NumberType, ErrorType> {
        let result = self.visit(node, context.clone());

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
                context,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpeter() {
        let text = "4*(3-2)/(4-2)".to_string();
        let text_value = 2;
        let mut lexer: Lexer = Lexer::new("finshell £".to_string(), text);
        let mut tokens: Vec<Token> = vec![];

        match lexer.tokenize() {
            Ok(t) => tokens = t,
            Err(e) => panic!("{:?}", e),
        };

        // Get Abstract Syntax Tree
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        let mut node: Option<Node> = None;
        if let Ok(root) = result.clone() {
            node = Some(root);
        }

        if let Some(root) = node {
            let interpeter = Interpeter::new();
            let result = interpeter.visit(root, Context::init("Test Program"));
            let number = match result {
                Ok(num) => println!("{}", num),
                Err(e) => panic!("{:?}", e),
            };
        }
    }
}
