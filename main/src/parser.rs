use crate::errors::{DisallowedCharError, ErrorType, SyntaxError};
use crate::lexer::Lexer;
use crate::position::Position;
use crate::token::{
    Token,
    TokenType::{self, Divide, Float, Int, LParen, Minus, Multiply, Plus, RParen, String},
};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Binop(Box<Node>, Token, Box<Node>),
    Value(Token),
    Unary(Token, Box<Node>),
}

impl Default for Node {
    fn default() -> Self {
        Self::Value(Token::default())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Binop(left, op, right) => write!(f, "[{}, {}, {}]", left, op, right),
            Node::Value(val) => write!(f, "{}", val),
            Node::Unary(optok, node) => write!(f, "[{}, {}]", optok, node),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    token_index: i64,
    current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Self {
            tokens,
            token_index: -1,
            current_token: Token::default(),
        };
        parser.advance();
        parser
    }
    fn advance(&mut self) {
        //println!("{:?} {:?}", self.token_index, self.current_token);
        self.token_index += 1;
        if self.token_index < self.tokens.len() as i64 {
            self.current_token = self.tokens.get(self.token_index as usize).unwrap().clone();
        }
    }
    pub fn parse(&mut self) -> Result<Node, ErrorType> {
        let result = self.expression();
        if result.is_ok() && self.current_token.type_() != TokenType::EndOfFile {
            return Err(ErrorType::SyntaxError(SyntaxError::new(
                self.current_token.position_start(),
                self.current_token.position_end(),
                "Expected '+', '-', '*', or '/'".to_string(),
            )));
        };
        result
    }

    fn factor(&mut self) -> Result<Node, ErrorType> {
        //println!("{:?}", self.tokens);
        //println!("{:?} {:?}", self.token_index, self.current_token);
        let token = self.current_token.clone();
        match token.type_() {
            Int(_) | Float(_) => {
                self.advance();
                Ok(Node::Value(token))
            }

            Plus | Minus => {
                self.advance();
                let result = self.factor();
                match result {
                    Ok(factor) => Ok(Node::Unary(token, Box::new(factor))),
                    Err(e) => Err(e),
                }
            }

            LParen => {
                self.advance();
                let result = self.expression();
                match result {
                    Err(e) => Err(e),
                    Ok(node) => {
                        if self.current_token.type_() == TokenType::RParen {
                            self.advance();
                            Ok(node)
                        } else {
                            Err(ErrorType::SyntaxError(SyntaxError::new(
                                token.position_start(),
                                token.position_end(),
                                "Expected ')'".to_string(),
                            )))
                        }
                    }
                }
            }

            _ => Err(ErrorType::SyntaxError(SyntaxError::new(
                token.position_start(),
                token.position_end(),
                "Expected Int or Float".to_string(),
            ))),
        }
    }

    fn term(&mut self) -> Result<Node, ErrorType> {
        let valid_operations = vec![Multiply, Divide];
        self.binary_operation(Self::factor, valid_operations)
    }

    fn expression(&mut self) -> Result<Node, ErrorType> {
        let valid_operations = vec![Plus, Minus];
        self.binary_operation(Self::term, valid_operations)
    }

    fn binary_operation(
        &mut self,
        function: fn(&mut Parser) -> Result<Node, ErrorType>,
        operation_tokens: Vec<TokenType>,
    ) -> Result<Node, ErrorType> {
        let result = function(self);
        let mut left = match result {
            Ok(node) => node,
            Err(e) => return Err(e),
        };

        while operation_tokens.contains(&self.current_token.type_()) {
            let current = self.current_token.clone();
            self.advance();
            let result = function(self);
            let right = match result {
                Ok(node) => node,
                Err(e) => return Err(e),
            };
            left = Node::Binop(Box::new(left), current, Box::new(right));
        }
        Ok(left)
    }
}

fn print_ast(root: Node) {
    match root {
        Node::Binop(left, _op, right) => {
            print_ast(*left);
            print_ast(*right)
        }
        Node::Value(_val) => {}
        Node::Unary(_op, node) => {
            print_ast(*node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_tokens_from_str(s: &str) -> Vec<Token> {
        let mut lexer = Lexer::new("test".to_string(), s.to_string());
        let tokens = match lexer.tokenize() {
            Ok(ts) => ts,
            Err(e) => panic!("Lexer failed to exctract tokens due to: {:?}", e),
        };

        return tokens;
    }

    fn get_ast_from_string(s: &str) -> Node {
        let tokens = get_tokens_from_str(s);
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(root) => root,
            Err(e) => panic!("Parser failed to parse due to: {:?}", e),
        }
    }

    #[test]
    fn test_parser() {
        let ast = get_ast_from_string("3*4+5/5");
        println!("AST: {:?}", ast);
    }

    #[test]
    fn test_unary_op() {
        let ast1 = Node::Unary(
            Token::new_no_pos(Minus),
            Box::new(Node::Value(Token::new_no_pos(Int(3)))),
        );
        let ast2 = get_ast_from_string("-3");
        let ast1_rep = format!("{}", ast1);
        let ast2_rep = format!("{}", ast2);
        assert_eq!(ast1_rep, ast2_rep);
    }

    #[test]
    fn test_ast_print() {
        let ast = get_ast_from_string("3/1+2*4");
        print_ast(ast);
    }
}
