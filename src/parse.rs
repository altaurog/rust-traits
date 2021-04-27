use crate::types::*;

// Exercise 2
peg::parser!{
    grammar simple_expr() for str {
        pub rule expr() -> ExprT = precedence!{
            x:(@) " "* "+" " "* y:@ { ExprT::Add(Box::new(x), Box::new(y)) }
            --
            x:(@) " "* "*" " "* y:@ { ExprT::Mul(Box::new(x), Box::new(y)) }
            --
            n:number() { n }
        }

        rule number() -> ExprT
            = n:$(['-' | '+']? ['0'..='9']+) {? n.parse().map(ExprT::Lit).or(Err("i32")) }
    }
}

pub fn exprt_to_expr<T, F, G, H>(lit: F, add: G, mul: H, exprt: &ExprT) -> T where
    T: Expr,
    F: Fn(i32) -> T,
    G: Fn(&T, &T) -> T,
    H: Fn(&T, &T) -> T {

    match *exprt {
        ExprT::Lit(i) => lit(i),
        ExprT::Add(box_x, box_y) => {
            let xt = exprt_to_expr(lit, add, mul, &box_x);
            let yt = exprt_to_expr(lit, add, mul, &box_y);
            add(&xt, &yt)
        },
        ExprT::Mul(box_x, box_y) => {
            let xt = exprt_to_expr(lit, add, mul, &box_x);
            let yt = exprt_to_expr(lit, add, mul, &box_y);
            mul(&xt, &yt)
        },
    }
}

pub fn parseExp<T, F, G, H>(lit: F, add: G, mul: H, input: &str) -> Option<T> where
    T: Expr,
    F: Fn(i32) -> T,
    G: Fn(&T, &T) -> T,
    H: Fn(&T, &T) -> T {

    match simple_expr::expr(&input) {
        Ok(exprt) => Some(exprt_to_expr(lit, add, mul, &exprt)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod ex2_test {
    use super::*;

    #[test]
    fn test_parseExp() {
        let expval = parseExp(ExprT::Lit, ExprT::Add, ExprT::Mul, "2 + 3");
        assert_eq!(expval, Some(ExprT::Add(Box::new(ExprT::Lit(2)), Box::new(ExprT::Lit(3)))));
    }
}
