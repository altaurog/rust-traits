use crate::types::*;
use ExprT::*;

// Exercise 3
impl Expr for ExprT {
    fn lit(val: i32) -> ExprT { Lit(val) }

    fn add(&self, other: &ExprT) -> ExprT {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Add(Box::new(my_self), Box::new(the_other))
    }

    fn mul(&self, other: &ExprT) -> ExprT {
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

