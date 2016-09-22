extern crate ruroonga_expr as expr;

use expr::fulltext_expr::FulltextExpr;
use expr::greater_equal_expr::GreaterEqualExpr;

fn logical_and_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr + rexpr)).build();
    println!("Grouped Logical And: {}", result);
}

fn logical_or_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr | rexpr)).build();
    println!("Grouped Logical OR: {}", result);
}

fn logical_not_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr - rexpr)).build();
    println!("Grouped Logical Not: {}", result);
}

fn main() {
    logical_and_expr();
    logical_or_expr();
    logical_not_expr();
}
