extern crate ruroonga_expr;

use ruroonga_expr::dsl::*;

#[test]
fn logical_and_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr + rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust + language:@Haskell)\'", result);
}

#[test]
fn logical_or_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr | rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust OR language:@Haskell)\'", result);
}

#[test]
fn logical_not_expr() {
    let lexpr = fulltext_expr("Rust").column("language").prepare().unwrap();
    let rexpr = fulltext_expr("Haskell").column("language").prepare().unwrap();
    let comb_lexpr = greater_equal_expr("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr - rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust - language:@Haskell)\'", result);
}
