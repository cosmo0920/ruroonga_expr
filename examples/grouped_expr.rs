extern crate ruroonga_expr as expr;

use expr::fulltext_expr::FulltextExpr;
use expr::greater_equal_expr::GreaterEqualExpr;
use expr::groupable::{Groupable, Fragmentable};
use expr::groupable::group_builder::GroupBuilder;
use expr::groupable::logical_or_builder::LogicalOrBuilder;
use expr::groupable::logical_and_builder::LogicalAndBuilder;
use expr::groupable::logical_not_builder::LogicalNotBuilder;

fn logical_and_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare().to_fragment();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare().to_fragment();
    let logical_or_expr = LogicalAndBuilder::new(lexpr, rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
    println!("Grouped Logical And: {}", result);
}

fn logical_or_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare().to_fragment();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare().to_fragment();
    let logical_or_expr = LogicalOrBuilder::new(lexpr, rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
    println!("Grouped Logical OR: {}", result);
}

fn logical_not_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare().to_fragment();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare().to_fragment();
    let logical_or_expr = LogicalNotBuilder::new(lexpr, rexpr).to_group();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
    let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
    println!("Grouped Logical Not: {}", result);
}

fn main() {
    logical_and_expr();
    logical_or_expr();
    logical_not_expr();
}
