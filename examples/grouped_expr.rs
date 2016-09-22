extern crate ruroonga_expr as expr;

use expr::fulltext_expr::FulltextExpr;
use expr::greater_equal_expr::GreaterEqualExpr;
use expr::groupable::{Groupable, Fragmentable};
use expr::groupable::group_builder::GroupBuilder;

fn logical_and_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let logical_or_expr = (lexpr + rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
    println!("Grouped Logical And: {}", result);
}

fn logical_or_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let logical_or_expr = (lexpr | rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
    println!("Grouped Logical OR: {}", result);
}

fn logical_not_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let logical_not_expr = (lexpr - rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_not_expr).build();
    println!("Grouped Logical Not: {}", result);
}

fn main() {
    logical_and_expr();
    logical_or_expr();
    logical_not_expr();
}
