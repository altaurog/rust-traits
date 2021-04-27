use crate::types::*;

// exercise 4

#[derive(Debug, PartialEq)]
struct Mod7(i32);

impl Expr for Mod7 {
    fn lit(val: i32) -> Mod7 { Mod7(val % 7) }
    fn add(&self, other: &Mod7) -> Mod7 { Mod7((self.0 + other.0) % 7) }
    fn mul(&self, other: &Mod7) -> Mod7 { Mod7((self.0 * other.0) % 7) }
}

#[cfg(test)]
mod ex4_tests_mod7 {
    use super::*;

    #[test]
    fn test_mod7_lit() {
        let result: Mod7 = Expr::lit(2);
        assert_eq!(result, Mod7(2));
        let result: Mod7 = Expr::lit(9);
        assert_eq!(result, Mod7(2));
    }

    #[test]
    fn test_mod7_add() {
        let a: Mod7 = Expr::lit(4);
        let b: Mod7 = Expr::lit(4);
        assert_eq!(a.add(&b), Mod7(1));
    }

    #[test]
    fn test_mod7_mul() {
        let a: Mod7 = Expr::lit(3);
        let b: Mod7 = Expr::lit(6);
        assert_eq!(a.mul(&b), Mod7(4));
    }
}
