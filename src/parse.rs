use crate::types::*;
use crate::eval::*;

// Exercise 2
/*
evalStr :: String -> Maybe Integer
evalStr str = do
    ast <- parseExp ET.Lit ET.Add ET.Mul str
    return $ eval ast
*/

pub fn eval_str(input: String) -> Option<i32> {
    parse_exp(input).map(eval)
}

pub fn parse_exp<T: Expr>(input: String) -> Option<T> {
    match simple_expr::expr(&input) {
        Ok(exprt) => Some(exprt_to_expr(exprt)),
        Err(_) => None,
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

peg::parser!{
    grammar simple_expr() for str {
        pub rule expr() -> ExprT = precedence!{
            x:(@) _ "+" _ y:@ { ExprT::Add(Box::new(x), Box::new(y)) }
            --
            x:(@) _ "*" _ y:@ { ExprT::Mul(Box::new(x), Box::new(y)) }
            --
            n:number() { n }
            "(" _ e:expr() _ ")" { e }
        }

        rule number() -> ExprT
            = n:$(['-' | '+']? ['0'..='9']+) {? n.parse().map(ExprT::Lit).or(Err("i32")) }

        rule _ = quiet!{[' ' | '\n' | '\t']*}
    }
}

#[cfg(test)]
mod ex2_test {
    use super::*;

    #[test]
    fn test_parse_exp() {
        let expval: ExprT = parse_exp(String::from("2 + 3")).unwrap();
        assert_eq!(expval, ExprT::Add(Box::new(ExprT::Lit(2)), Box::new(ExprT::Lit(3))));
    }

    #[test]
    fn test_parse_parentheses() {
        let expval: ExprT = parse_exp(String::from("(2)")).unwrap();
        assert_eq!(expval, ExprT::Lit(2));
    }

    #[test]
    fn test_eval_str() {
        assert_eq!(eval_str(String::from("2 + 3")), Some(5));
        assert_eq!(eval_str(String::from("2 * 3")), Some(6));
        assert_eq!(eval_str(String::from("1 + 2 * 3")), Some(7));
        assert_eq!(eval_str(String::from("(1 + 2) * 3")), Some(9));
        assert_eq!(eval_str(String::from("hello, world!")), None);
    }
}
/*
            */
