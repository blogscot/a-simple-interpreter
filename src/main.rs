mod interpreter;
mod lexer;
mod token;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("3 * 5".into());
    let result = interpreter.expr();

    println!("{}", result);
}
