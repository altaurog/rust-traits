use crate::types::*;

// exercise 4
/*
instance Expr Integer where
    lit = id
    add = (+)
    mul = (*)
*/

impl Expr for i32 {
    fn lit(val: i32) -> i32 { val }
    fn add(&self, other: &i32) -> i32 { *self + *other }
    fn mul(&self, other: &i32) -> i32 { (*self) * (*other) }
}

#[cfg(test)]
mod ex4_tests_i32 {
    use super::*;

    #[test]
    fn test_i32_lit() {
        let result: i32 = Expr::lit(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_i32_add() {
        let result = Expr::add(&2, &3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_i32_mul() {
        let result = Expr::mul(&2, &3);
        assert_eq!(result, 6);
    }
}
