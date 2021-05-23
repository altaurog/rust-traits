use typeclasses::{ExprT, Expr};

fn main() {
    let lit2 = ExprT::lit(2);
    println!("val is {:?}", lit2);
}
