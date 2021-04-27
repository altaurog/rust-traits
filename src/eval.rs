use crate::types::ExprT;
use ExprT::*;

// Exercise 1
pub fn eval(expr: ExprT) -> i32 {
    match expr {
        Lit(x) => x,
        Add(x, y) => eval(*x) + eval(*y),
        Mul(x, y) => eval(*x) * eval(*y),
    }
}

#[cfg(test)]
mod ex1_tests {
    use super::*;

    #[test]
    fn test_lit() {
        assert_eq!(eval(Lit(2)), 2);
    }

    #[test]
    fn test_add() {
        let result = eval(Add(Box::new(Lit(2)), Box::new(Lit(3))));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_mul() {
        let result = eval(Mul(Box::new(Lit(2)), Box::new(Lit(3))));
        assert_eq!(result, 6);
    }
}
