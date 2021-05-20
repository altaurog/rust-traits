use std::rc::Rc;
use crate::types::*;
use ExprTEnum::*;

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
    fn lit(val: i32) -> ExprT { Rc::new(Lit(val)) }

    fn add(&self, other: &ExprT) -> ExprT {
        Rc::new(Add(self.clone(), other.clone()))
    }

    fn mul(&self, other: &ExprT) -> ExprT {
        Rc::new(Mul(self.clone(), other.clone()))
    }
}

#[cfg(test)]
mod ex3_tests_exprt {
    use super::*;

    #[test]
    fn test_exprt_lit() {
        let result: ExprT = Expr::lit(2);
        assert_eq!(result, Rc::new(Lit(2)));
    }

    #[test]
    fn test_exprt_add() {
        let two: ExprT = Expr::lit(2);
        let three: ExprT = Expr::lit(3);
        let result: ExprT = two.add(&three);
        assert_eq!(result, Rc::new(Add(Rc::new(Lit(2)), Rc::new(Lit(3)))));
    }

    #[test]
    fn test_exprt_mul() {
        let two: ExprT = Expr::lit(2);
        let three: ExprT = Expr::lit(3);
        let result: ExprT = two.mul(&three);
        assert_eq!(result, Rc::new(Mul(Rc::new(Lit(2)), Rc::new(Lit(3)))));
    }
}

