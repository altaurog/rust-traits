
#[derive(Debug, PartialEq, Clone)]
pub enum ExprT {
    Lit(i32),
    Add(Box<ExprT>, Box<ExprT>),
    Mul(Box<ExprT>, Box<ExprT>),
}

pub trait Expr <'a> {
    fn lit(val: i32) -> Self;
    fn add<'b: 'a>(&'b self, other: &'b Self) -> Self;
    fn mul<'b: 'a>(&'b self, other: &'b Self) -> Self;
}

