use crate::types::*;

// exercise 4
/*
instance Expr Integer where
    lit = id
    add = (+)
    mul = (*)
*/

impl <'a> Expr<'a> for i32 {
    fn lit(val: i32) -> Self { val }
    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self { *self + *other }
    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self { (*self) * (*other) }
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
