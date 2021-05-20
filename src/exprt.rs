use std::rc::Rc;
use crate::types::*;
use ExprT::*;

// Exercise 3
/*
class Expr a where
    lit :: Integer -> a
    add :: a -> a -> a
    mul :: a -> a -> a

instance Expr ET.ExprT where
    lit = ET.Lit
    add = ET.Add
    mul = ET.Mul
*/
impl Expr for ExprT {
    fn lit(val: i32) -> ExprT { Lit(val) }

    fn add(&self, other: &ExprT) -> ExprT {
        Add(Rc::new(*self), Rc::new(*other))
    }

    fn mul(&self, other: &ExprT) -> ExprT {
        Mul(Rc::new(*self), Rc::new(*other))
    }
}

#[cfg(test)]
mod ex3_tests_exprt {
    use super::*;

    #[test]
    fn test_exprt_lit() {
        let result: ExprT = Expr::lit(2);
        assert_eq!(result, Lit(2));
    }

    #[test]
    fn test_exprt_add() {
        let two: ExprT = Expr::lit(2);
        let three: ExprT = Expr::lit(3);
        let result: ExprT = two.add(&three);
        assert_eq!(result, Add(Rc::new(Lit(2)), Rc::new(Lit(3))));
    }

    #[test]
    fn test_exprt_mul() {
        let two: ExprT = Expr::lit(2);
        let three: ExprT = Expr::lit(3);
        let result: ExprT = two.mul(&three);
        assert_eq!(result, Mul(Rc::new(Lit(2)), Rc::new(Lit(3))));
    }
}

