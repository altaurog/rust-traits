use std::rc::Rc;
use crate::types::*;
use crate::eval::*;
use ExprTEnum::*;

// Exercise 2
/*
evalStr :: String -> Maybe Integer
evalStr str = do
    ast <- parseExp ET.Lit ET.Add ET.Mul str
    return $ eval ast
*/

pub fn eval_str(input: String) -> Option<i32> {
    parse_exp(input).as_ref().map(eval)
}

pub fn parse_exp<T: Expr>(input: String) -> Option<T> {
    match simple_expr::expr(&input) {
        Ok(exprt) => Some(exprt_to_expr(&exprt)),
        Err(_) => None,
    }
}

pub fn exprt_to_expr<T: Expr>(exprt: &ExprT) -> T {
    match &**exprt {
        Lit(i) => T::lit(*i),
        Add(x, y) => {
            let xt: T = exprt_to_expr(x);
            let yt: T = exprt_to_expr(y);
            xt.add(&yt)
        },
        Mul(x, y) => {
            let xt: T = exprt_to_expr(x);
            let yt: T = exprt_to_expr(y);
            xt.mul(&yt)
        },
    }
}

peg::parser!{
    grammar simple_expr() for str {
        pub rule expr() -> ExprT = precedence!{
            x:(@) " "* "+" " "* y:@ { x.add(&y) }
            --
            x:(@) " "* "*" " "* y:@ { x.mul(&y) }
            --
            n:number() { n }
        }

        rule number() -> ExprT
            = n:$(['-' | '+']? ['0'..='9']+) {? n.parse().map(ExprT::lit).or(Err("i32")) }
    }
}

#[cfg(test)]
mod ex2_test {
    use super::*;

    #[test]
    fn test_parse_exp() {
        let expval: ExprT = parse_exp(String::from("2 + 3")).unwrap();
        assert_eq!(expval, Rc::new(Add(Rc::new(Lit(2)), Rc::new(Lit(3)))));
    }

    #[test]
    fn test_eval_str() {
        assert_eq!(eval_str(String::from("2 + 3")), Some(5));
        assert_eq!(eval_str(String::from("2 * 3")), Some(6));
        assert_eq!(eval_str(String::from("hello, world!")), None);
    }
}
