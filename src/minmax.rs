use std::cmp;
use crate::types::*;

// exercise 4
#[derive(Debug, PartialEq)]
struct MinMax(i32);

impl Expr for MinMax {
    fn lit(val: i32) -> MinMax { MinMax(val) }
    fn add(&self, other: &MinMax) -> MinMax { MinMax(cmp::max(self.0, other.0)) }
    fn mul(&self, other: &MinMax) -> MinMax { MinMax(cmp::min(self.0, other.0)) }
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
