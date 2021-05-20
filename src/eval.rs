use std::rc::Rc;
use crate::types::ExprT;
use ExprT::*;

// Exercise 1
/*
eval :: ET.ExprT -> Integer
eval (ET.Lit x) = x
eval (ET.Add x y) = (eval x) + (eval y)
eval (ET.Mul x y) = (eval x) * (eval y)
*/

pub fn eval(expr: Rc<ExprT>) -> i32 {
    match &*expr {
        Lit(x) => x,
        Add(x, y) => eval(x) + eval(y),
        Mul(x, y) => eval(x) * eval(y),
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
        let result = eval(Add(Rc::new(Lit(2)), Rc::new(Lit(3))));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_mul() {
        let result = eval(Mul(Rc::new(Lit(2)), Rc::new(Lit(3))));
        assert_eq!(result, 6);
    }
}
