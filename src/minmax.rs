use std::cmp;
use crate::types::*;

// exercise 4
/*
newtype MinMax = MinMax Integer deriving (Eq, Show)

instance Expr MinMax where
    lit = MinMax
    add (MinMax a) (MinMax b) = lit $ max a b
    mul (MinMax a) (MinMax b) = lit $ min a b
*/

#[derive(Debug, PartialEq)]
struct MinMax(i32);

impl <'a> Expr<'a> for MinMax {
    fn lit(val: i32) -> Self { MinMax(val) }

    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self {
        MinMax(cmp::max(self.0, other.0))
    }

    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self {
        MinMax(cmp::min(self.0, other.0))
    }
}

#[cfg(test)]
mod ex4_tests_minmax {
    use super::*;

    #[test]
    fn test_minmax_lit() {
        let result: MinMax = Expr::lit(2);
        assert_eq!(result, MinMax(2));
    }

    #[test]
    fn test_minmax_add() {
        let a: MinMax = Expr::lit(2);
        let b: MinMax = Expr::lit(3);
        assert_eq!(a.add(&b), MinMax(3));
    }

    #[test]
    fn test_minmax_mul() {
        let a: MinMax = Expr::lit(2);
        let b: MinMax = Expr::lit(3);
        assert_eq!(a.mul(&b), MinMax(2));
    }
}
