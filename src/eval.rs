use crate::types::{ExprT, ExprTEnum};
use ExprTEnum::*;

// Exercise 1
/*
eval :: ET.ExprT -> Integer
eval (ET.Lit x) = x
eval (ET.Add x y) = (eval x) + (eval y)
eval (ET.Mul x y) = (eval x) * (eval y)
*/

pub fn eval(expr: &ExprT) -> i32 {
    match &**expr {
        Lit(x) => *x,
        Add(x, y) => eval(x) + eval(y),
        Mul(x, y) => eval(x) * eval(y),
    }
}

#[cfg(test)]
mod ex1_tests {
    use std::rc::Rc;
    use super::*;

    #[test]
    fn test_lit() {
        let exprt = Rc::new(Lit(2));
        assert_eq!(eval(&exprt), 2);
    }

    #[test]
    fn test_add() {
        let exprt = Rc::new(Add(Rc::new(Lit(2)), Rc::new(Lit(3))));
        let result = eval(&exprt);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_mul() {
        let exprt = Rc::new(Mul(Rc::new(Lit(2)), Rc::new(Lit(3))));
        let result = eval(&exprt);
        assert_eq!(result, 6);
    }
}
