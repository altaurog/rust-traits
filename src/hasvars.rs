use std::collections::HashMap;
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

impl <'a> Expr<'a> for VarExprT {
    fn lit(val: i32) -> Self { Lit(val) }

    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self {
        let my_self = (*self).clone();
        let the_other = (*other).clone();
        Add(Box::new(my_self), Box::new(the_other))
    }

    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self {
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
impl HasVars for Box<dyn Fn(SymbolMap) -> Option<i32>> {
    fn var(sym: String) -> Self {
        let sym = sym.clone();
        Box::new(move |valmap| valmap.get(&sym).map(|v| *v))
    }
}

impl <'a> Expr<'a> for Box<dyn Fn(SymbolMap) -> Option<i32> + 'a> {
    fn lit(val: i32) -> Self {
        Box::new(move |_| Some(val))
    }

    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self {
        Box::new(|env| self(env).zip(other(env)).map(|t| t.0 + t.1))
    }

    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self {
        Box::new(|env| self(env).zip(other(env)).map(|t| t.0 * t.1))
    }
}

/*

withVars :: [(String, Integer)]
            -> (M.Map String Integer -> Maybe Integer)
            -> Maybe Integer
withVars vals expr = expr $ M.fromList vals
*/
