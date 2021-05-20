use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprT {
    Lit(i32),
    Add(Rc<ExprT>, Rc<ExprT>),
    Mul(Rc<ExprT>, Rc<ExprT>),
}

pub trait Expr {
    fn lit(val: i32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

