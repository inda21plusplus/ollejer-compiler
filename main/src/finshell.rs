use crate::lexer;
use std::io::{self, BufRead};

pub fn shell_loop() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    while let Some(line) = lines.next() {
        // Dont understand Clippy message for while
        match lexer::run(line) {
            Ok(tokens) => println!("{:?}", tokens),
            Err(e) => println!("{}", e),
        }
    }
}
