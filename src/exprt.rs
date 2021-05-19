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
impl <'a> Expr<'a> for ExprT {
    fn lit(val: i32) -> Self { Lit(val) }

    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Add(Box::new(my_self), Box::new(the_other))
    }

    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Mul(Box::new(my_self), Box::new(the_other))
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
        assert_eq!(result, Add(Box::new(Lit(2)), Box::new(Lit(3))));
    }

    #[test]
    fn test_exprt_mul() {
        let two: ExprT = Expr::lit(2);
        let three: ExprT = Expr::lit(3);
        let result: ExprT = two.mul(&three);
        assert_eq!(result, Mul(Box::new(Lit(2)), Box::new(Lit(3))));
    }
}

