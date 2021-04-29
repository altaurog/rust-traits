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

pub fn exprt_to_expr<T: Expr>(exprt: ExprT) -> T {
    match exprt {
        ExprT::Lit(i) => T::lit(i),
        ExprT::Add(box_x, box_y) => {
            let xt: T = exprt_to_expr(*box_x);
            let yt: T = exprt_to_expr(*box_y);
            xt.add(&yt)
        },
        ExprT::Mul(box_x, box_y) => {
            let xt: T = exprt_to_expr(*box_x);
            let yt: T = exprt_to_expr(*box_y);
            xt.mul(&yt)
        },
    }
}

pub fn parse_exp<T: Expr>(input: &str) -> Option<T> {
    match simple_expr::expr(&input) {
        Ok(exprt) => Some(exprt_to_expr(exprt)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod ex2_test {
    use super::*;

    #[test]
    fn test_parse_exp() {
        let expval: ExprT = parse_exp("2 + 3").unwrap();
        assert_eq!(expval, ExprT::Add(Box::new(ExprT::Lit(2)), Box::new(ExprT::Lit(3))));
    }
}
