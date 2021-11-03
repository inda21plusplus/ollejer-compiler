use crate::errors::DisallowedCharError;
use crate::errors::ErrorType;
use crate::position::Position;
use crate::token::{
    Token,
    TokenType::{self, Divide, EndOfFile, Float, Int, LParen, Minus, Multiply, Plus, RParen},
};

#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(file_name: String, text: String) -> Self {
        let mut lexer = Self {
            text: text.clone(),
            pos: Position::new(-1, 0, -1, file_name, text),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char);
        if self.pos.index() < self.text.len() as i64 {
            self.current_char = Some(self.text.chars().nth(self.pos.index() as usize).unwrap());
        } else {
            self.current_char = None;
        }
    }

    // TODO CHECK IF ERROR IN RESULT RETURN SHOULD BE ERROR TRAIT / ILLIGALCHAR STRUCT
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ErrorType> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(current) = self.current_char {
            match current {
                ' ' | '\t' => (),
                '+' => tokens.push(Token::new(Plus, Some(self.pos.clone()), None)),
                '-' => tokens.push(Token::new(Minus, Some(self.pos.clone()), None)),
                '*' => tokens.push(Token::new(Multiply, Some(self.pos.clone()), None)),
                '/' => tokens.push(Token::new(Divide, Some(self.pos.clone()), None)),
                '(' => tokens.push(Token::new(LParen, Some(self.pos.clone()), None)),
                ')' => tokens.push(Token::new(RParen, Some(self.pos.clone()), None)),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    tokens.push(self.construct_number());
                    continue;
                }
                _ => {
                    let pos_start = self.pos.clone();
                    self.advance();
                    let error = ErrorType::DisallowedCharError(DisallowedCharError::new(
                        Some(pos_start),
                        Some(self.pos.clone()),
                        current.to_string(),
                    ));
                    return Err(error);
                }
            }

            self.advance()
        }

        tokens.push(Token::new(EndOfFile, Some(self.pos.clone()), None));
        Ok(tokens)
    }

    // TODO MAKE THIS RETURN A RESULT INSTEAD OF PANIC
    fn construct_number(&mut self) -> Token {
        let mut number_string = String::new();
        let mut dot_count = 0;
        let pos_start = self.pos.clone();

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
            return Token::new(
                Int(number_string.parse::<i64>().unwrap()),
                Some(pos_start),
                Some(self.pos.clone()),
            );
        }
        if dot_count == 1 {
            return Token::new(
                Float(number_string.parse::<f64>().unwrap()),
                Some(pos_start),
                Some(self.pos.clone()),
            );
        }
        Token::new(Divide, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_token_types_from_str(text: &str) -> Vec<TokenType> {
        let file_name = "(stdin)".to_string();
        //let text = "2 + 2".to_string();
        let mut lexer: Lexer = Lexer::new(file_name, text.to_string());
        let tokens = match lexer.tokenize() {
            Ok(t) => (t),
            Err(e) => panic!("{:?}", e),
        };
        tokens.iter().map(|t| t.type_()).collect()
    }
    #[test]
    fn unit_test_lexer() {
        let given_token_types = get_token_types_from_str("2+ 8.0*8/ (3)");
        let expected_token_types = vec![
            Int(2),
            Plus,
            Float(8.0),
            Multiply,
            Int(8),
            Divide,
            LParen,
            Int(3),
            RParen,
        ];
        //println!("{:?}", given_tokens);
        //println!("{:?}", expected_tokens);
        assert_eq!(given_token_types, expected_token_types)
    }

    #[test]
    fn test_int() {
        assert_eq!(
            vec![TokenType::Int(1), TokenType::Plus, TokenType::Int(3)],
            get_token_types_from_str("1+3"),
        )
    }

    #[test]
    fn test_float() {
        let valid = vec![
            TokenType::Float(2.8),
            TokenType::Plus,
            TokenType::Float(3.0),
        ];
        let given = get_token_types_from_str("2.8+3.0");
        //println!("{:?}", valid);
        //println!("{:?}", given);
        assert_eq!(valid, given)
    }

    #[test]
    fn test_ignore_space() {
        assert_eq!(
            get_token_types_from_str("1+3.0"),
            get_token_types_from_str("1 + 3.0")
        );
        assert_eq!(
            get_token_types_from_str("1+ 3.0"),
            get_token_types_from_str("1 +3.0")
        );
    }
}
