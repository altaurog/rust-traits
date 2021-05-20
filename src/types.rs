use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprTEnum {
    Lit(i32),
    Add(ExprT, ExprT),
    Mul(ExprT, ExprT),
}

pub type ExprT = Rc<ExprTEnum>;

pub trait Expr {
    fn lit(val: i32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

