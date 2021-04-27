
#[derive(Debug, PartialEq, Clone)]
pub enum ExprT {
    Lit(i32),
    Add(Box<ExprT>, Box<ExprT>),
    Mul(Box<ExprT>, Box<ExprT>),
}

pub trait Expr {
    fn lit(val: i32) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
}

