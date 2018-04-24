mod interpreter;
mod lexer;
mod token;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("4 + 7".into());
    let result = interpreter.expr();

    println!("{}", result);
}
