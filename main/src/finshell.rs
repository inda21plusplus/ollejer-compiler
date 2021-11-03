use crate::errors::ErrorType;
use crate::interpeter::Interpeter;
use crate::lexer::Lexer;
use crate::parser::{Node, Parser};
use crate::token::Token;
use std::io::{self, BufRead};

pub fn run(file_name: String, text: String) -> Result<Node, ErrorType> {
    // Get tokens
    let mut lexer: Lexer = Lexer::new(file_name, text);
    let tokens = match lexer.tokenize() {
        Ok(t) => (t),
        Err(e) => return Err(e),
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
        let result = interpeter.visit(root);
        match result {
            Ok(num) => println!("{}", num),
            Err(e) => return Err(e),
        }
    }

    result
}

pub fn shell_loop() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    while let Some(line) = lines.next() {
        // Unclear Clippy message for while
        match run("<stdin>".to_string(), line) {
            Ok(out) => (/*println!("Ast: {}", out)*/),
            Err(e) => match e {
                ErrorType::DisallowedCharError(e) => println!("{}", e.as_string()),
                ErrorType::SyntaxError(e) => println!("{}", e.as_string()),
                ErrorType::RunTimeError(e) => println!("{}", e.as_string()),
                ErrorType::DivisionByZeroError(e) => println!("{}", e.as_string()),
            },
        }
    }
}
