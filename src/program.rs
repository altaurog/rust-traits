use std::collections::VecDeque;
use crate::types::*;
use crate::stackvm::*;
use crate::parse;
use StackExp::*;

// exercise 5
/*
instance Expr VM.Program where
    lit i = [VM.PushI i]
    add a b = concat [a, b, [VM.Add]]
    mul a b = concat [a, b, [VM.Mul]]


compile :: String -> Maybe VM.Program
compile = parseExp lit add mul
*/

impl <'a> Expr<'a> for Program {
    fn lit(val: i32) -> Self {
        VecDeque::from(vec![ PushI(val) ])
    }

    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self {
        let mut new = (*self).clone();
        new.extend((*other).clone().into_iter());
        new.push_back(Add);
        new
    }

    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self {
        let mut new = (*self).clone();
        new.extend((*other).clone().into_iter());
        new.push_back(Mul);
        new
    }
}

pub fn compile(expr: String) -> Option<Program> {
    parse::parse_exp(expr)
}


#[cfg(test)]
mod ex5_test {
    use super::*;

    #[test]
    fn test_single_val() {
        assert_eq!(run("1"), Ok(StackVal::IVal(1)));
    }

    #[test]
    fn test_simple_add() {
        assert_eq!(run("1 + 7"), Ok(StackVal::IVal(8)));
    }

    #[test]
    fn test_simple_mul() {
        assert_eq!(run("3 * 4"), Ok(StackVal::IVal(12)));
    }

    fn run(expr: &str) -> Result<StackVal, String> {
        match compile(String::from(expr)) {
            Some(program) => stackvm(program),
            None => Err(String::from("parse error")),
        }
    }
}
