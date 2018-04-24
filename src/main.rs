mod interpreter;
mod lexer;
mod token;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("7 + 3 * (10 / (12 / (3 + 1) - 1))");
    let result = interpreter.expr();

    println!("{}", result);
}
