use std::collections::HashMap;
use std::rc::Rc;
use crate::types::*;

// exercise 6

type SymbolMap = HashMap<String, i32>;

/*
class HasVars a where
    var :: String -> a

data VarExprT = Lit Integer
        | Add VarExprT VarExprT
        | Mul VarExprT VarExprT
        | Var String
    deriving (Show, Eq)
*/
pub trait HasVars {
    fn var(sym: String) -> Self;
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarExprT {
    Lit(i32),
    Add(Box<VarExprT>, Box<VarExprT>),
    Mul(Box<VarExprT>, Box<VarExprT>),
    Var(String),
}

use VarExprT::*;

/*
instance Expr VarExprT where
    lit = Lit
    add = Add
    mul = Mul

instance HasVars VarExprT where
    var = Var
*/
impl HasVars for VarExprT {
    fn var(sym: String) -> Self { Var(sym) }
}

impl Expr for VarExprT {
    fn lit(val: i32) -> Self { Lit(val) }

    fn add(& self, other: & Self) -> Self {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Add(Box::new(my_self), Box::new(the_other))
    }

    fn mul(& self, other: & Self) -> Self {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Mul(Box::new(my_self), Box::new(the_other))
    }
}


/*
instance Expr (M.Map String Integer -> Maybe Integer) where
    lit = const . Just
    add a b = \env -> (liftM2 (+)) (a env) (b env)
    mul a b = \env -> (liftM2 (*)) (a env) (b env)

instance HasVars (M.Map String Integer -> Maybe Integer) where
    var = M.lookup
*/
type SymbolMapExpr = Rc<dyn Fn(&SymbolMap) -> Option<i32>>;

impl HasVars for SymbolMapExpr {
    fn var(sym: String) -> Self {
        let sym = sym.clone();
        Rc::new(move |valmap| valmap.get(&sym).map(|v| *v))
    }
}

impl Expr for SymbolMapExpr {
    fn lit(val: i32) -> Self {
        Rc::new(move |_| Some(val))
    }

    fn add(&self, other: &Self) -> Self {
        let a = Rc::clone(self);
        let b = Rc::clone(other);
        Rc::new(move |env| a(env).zip(b(env)).map(|t| t.0 + t.1))
    }

    fn mul(&self, other: &Self) -> Self {
        let a = Rc::clone(self);
        let b = Rc::clone(other);
        Rc::new(move |env| a(env).zip(b(env)).map(|t| t.0 * t.1))
    }
}

/*
withVars :: [(String, Integer)]
            -> (M.Map String Integer -> Maybe Integer)
            -> Maybe Integer
withVars vals expr = expr $ M.fromList vals
*/
fn with_vars(vals: Vec<(&str, i32)>, expr: SymbolMapExpr) -> Option<i32>
{
    let symbol_map: SymbolMap = vals
        .into_iter()
        .map(|p| (String::from(p.0), p.1))
        .collect();
    expr(&symbol_map)
}

#[cfg(test)]
mod ex6_test {
    use super::*;

    #[test]
    fn test_mul_lit_lit() {
        let a = SymbolMapExpr::lit(3);
        let b = SymbolMapExpr::lit(4);
        assert_eq!(eval(a.mul(&b)), Some(12));
    }

    #[test]
    fn test_mul_lit_var() {
        let a = SymbolMapExpr::lit(3);
        let b = SymbolMapExpr::var(String::from("x"));
        assert_eq!(eval(a.mul(&b)), Some(18));
    }

    #[test]
    fn test_add_var_var() {
        let a = SymbolMapExpr::var(String::from("x"));
        let b = SymbolMapExpr::var(String::from("y"));
        assert_eq!(eval(a.add(&b)), Some(9));
    }

    #[test]
    fn test_all() {
        let a = SymbolMapExpr::lit(3);
        let b = SymbolMapExpr::var(String::from("x"));
        let c = SymbolMapExpr::var(String::from("y"));
        assert_eq!(eval(a.add(&b).mul(&c)), Some(27));
    }

    #[test]
    fn test_missing_var() {
        let a = SymbolMapExpr::lit(3);
        let b = SymbolMapExpr::var(String::from("x"));
        let c = SymbolMapExpr::var(String::from("z"));
        assert_eq!(eval(a.add(&b).mul(&c)), None);
    }

    fn eval(expr: SymbolMapExpr) -> Option<i32> {
        let vals = vec![("x", 6), ("y", 3)];
        with_vars(vals, expr)
    }
}
