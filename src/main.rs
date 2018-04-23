mod interpreter;
mod token;

use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("4 - 7".into());
    let result = interpreter.expr();

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_single_digit_numbers() {
        let mut interpreter = Interpreter::new("4 + 7".into());
        let result = interpreter.expr();

        assert_eq!(result, 11);
    }

    #[test]
    fn subtract_two_single_digit_numbers() {
        let mut interpreter = Interpreter::new("4 - 7".into());
        let result = interpreter.expr();

        assert_eq!(result, -3);
    }

    #[test]
    fn add_multiple_digit_numbers() {
        let mut interpreter = Interpreter::new("101 + 99".into());
        let result = interpreter.expr();

        assert_eq!(result, 200);
    }

    #[test]
    fn subtract_multiple_digit_numbers() {
        let mut interpreter = Interpreter::new("1234 - 134".into());
        let result = interpreter.expr();

        assert_eq!(result, 1100);
    }

}
