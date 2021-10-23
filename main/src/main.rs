mod errors;
mod finshell;
mod lexer;
mod token;
fn main() {
    println!("Starting Shell");
    finshell::shell_loop();
}
