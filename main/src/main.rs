mod errors;
mod finshell;
mod interpeter;
mod lexer;
mod number;
mod parser;
mod position;
mod token;

fn main() {
    println!("Starting Shell");
    finshell::shell_loop();
}
