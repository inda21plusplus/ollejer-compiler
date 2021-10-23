use crate::errors::{DisallowedCharError, Error};
use crate::token::Token;

#[derive(Debug)]
struct Lexer {
    text: String,
    pos: i64,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let mut lexer = Self {
            text,
            pos: -1,
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.pos += 1;
        if self.pos < self.text.len() as i64 {
            self.current_char = Some(self.text.chars().nth(self.pos as usize).unwrap());
        } else {
            self.current_char = None;
        }
    }

    // TODO CHECK IF ERROR IN RESULT RETURN SHOULD BE ERROR TRAIT / ILLIGALCHAR STRUCT
    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(current) = self.current_char {
            match current {
                ' ' | '\t' => self.advance(),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    tokens.push(self.construct_number())
                }
                _ => {
                    let error =
                        DisallowedCharError::new("InvalidChar".to_string(), current.to_string());
                    return Err(error.as_string());
                }
            }

            self.advance()
        }

        Ok(tokens)
    }

    // TODO MAKE THIS RETURN A RESULT INSTEAD OF PANIC
    fn construct_number(&mut self) -> Token {
        let mut number_string = String::new();
        let mut dot_count = 0;

        while let Some(current) = self.current_char {
            if !current.is_digit(10) && current != '.' {
                break;
            }
            if current == '.' {
                if dot_count >= 1 {
                    panic!("To many dots in number!");
                }
                dot_count += 1;
            }
            number_string.push(current);

            self.advance()
        }

        if dot_count == 0 {
            return Token::Int(number_string.parse::<i64>().unwrap());
        }
        if dot_count == 1 {
            return Token::Float(number_string.parse::<f64>().unwrap());
        }

        Token::Divide
    }
}

pub fn run(text: String) -> Result<Vec<Token>, String> {
    let mut lexer: Lexer = Lexer::new(text);
    lexer.tokenize()
}
