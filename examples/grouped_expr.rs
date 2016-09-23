extern crate ruroonga_expr as expr;

use expr::dsl::*;

fn logical_and_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr + rexpr)).build();
    println!("Grouped Logical And: {}", result);
}

fn logical_or_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr | rexpr)).build();
    println!("Grouped Logical OR: {}", result);
}

fn logical_not_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr - rexpr)).build();
    println!("Grouped Logical Not: {}", result);
}

fn nested_logicals_expr() {
    use expr::groupable::Groupable;

    let llexpr = phrase_expr("Rust lang").column("language").prepare().unwrap();
    let lrexpr = phrase_expr("Haskell lang").column("language").prepare().unwrap();
    let lcomb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let llogical_expr = lcomb_lexpr % (llexpr | lrexpr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Rust lang\" OR language:@\"Haskell lang\")\'"),
               llogical_expr.clone().build());

    let rlexpr = phrase_expr("Ruby lang").column("language").prepare().unwrap();
    let rrexpr = phrase_expr("Python lang").column("language").prepare().unwrap();
    let rcomb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let rlogical_expr = rcomb_lexpr % (rlexpr | rrexpr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Ruby lang\" OR language:@\"Python lang\")\'"),
               rlogical_expr.clone().build());

    let result = logical::and(llogical_expr.to_group(),
                              rlogical_expr.to_group()).build();
    println!("Nested logicals expr: {}", result);
}

fn main() {
    logical_and_expr();
    logical_or_expr();
    logical_not_expr();
    nested_logicals_expr();
}
