extern crate ruroonga_expr as expr;

use expr::fulltext_expr::FulltextExpr;
use expr::phrase_expr::PhraseExpr;
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

fn nested_logicals_expr() {
    use expr::groupable::Groupable;
    use expr::groupable::logical_and_builder::LogicalAndBuilder;

    let llexpr = PhraseExpr::new("Rust lang").column("language").prepare();
    let lrexpr = PhraseExpr::new("Haskell lang").column("language").prepare();
    let lcomb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let llogical_expr = lcomb_lexpr % (llexpr | lrexpr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Rust lang\" OR language:@\"Haskell lang\")\'"),
               llogical_expr.clone().build());

    let rlexpr = PhraseExpr::new("Ruby lang").column("language").prepare();
    let rrexpr = PhraseExpr::new("Python lang").column("language").prepare();
    let rcomb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let rlogical_expr = rcomb_lexpr % (rlexpr | rrexpr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Ruby lang\" OR language:@\"Python lang\")\'"),
               rlogical_expr.clone().build());

    let result = LogicalAndBuilder::new(llogical_expr.to_group(),
                                        rlogical_expr.to_group()).build();
    println!("Nested logicals expr: {}", result);
}

fn main() {
    logical_and_expr();
    logical_or_expr();
    logical_not_expr();
    nested_logicals_expr();
}
