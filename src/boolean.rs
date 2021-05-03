use crate::types::*;

// exercise 4
/*
instance Expr Bool where
    lit = (> 0)
    add = (||)
    mul = (&&)
*/

impl Expr for bool {
    fn lit(val: i32) -> bool { val > 0 }
    fn add(&self, other: &bool) -> bool { (*self) || (*other) }
    fn mul(&self, other: &bool) -> bool { (*self) && (*other) }
}

#[cfg(test)]
mod ex4_tests_bool {
    use super::*;

    #[test]
    fn test_bool_lit() {
        let result: bool = Expr::lit(1);
        assert_eq!(result, true);
        let result: bool = Expr::lit(0);
        assert_eq!(result, false);
    }

    #[test]
    fn test_bool_add() {
        assert_eq!(Expr::add(&true, &true), true);
        assert_eq!(Expr::add(&true, &false), true);
        assert_eq!(Expr::add(&false, &true), true);
        assert_eq!(Expr::add(&false, &false), false);
    }

    #[test]
    fn test_bool_mul() {
        assert_eq!(Expr::mul(&true, &true), true);
        assert_eq!(Expr::mul(&true, &false), false);
        assert_eq!(Expr::mul(&false, &true), false);
        assert_eq!(Expr::mul(&false, &false), false);
    }
}
