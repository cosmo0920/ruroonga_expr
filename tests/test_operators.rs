extern crate ruroonga_expr;

use ruroonga_expr::fulltext_expr::FulltextExpr;
use ruroonga_expr::greater_equal_expr::GreaterEqualExpr;

#[test]
fn logical_and_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr + rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust + language:@Haskell)\'", result);
}

#[test]
fn logical_or_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr | rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust OR language:@Haskell)\'", result);
}

#[test]
fn logical_not_expr() {
    let lexpr = FulltextExpr::new("Rust").column("language").prepare();
    let rexpr = FulltextExpr::new("Haskell").column("language").prepare();
    let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare();
    let result = (comb_lexpr % (lexpr - rexpr)).build();
    assert_eq!("\'n_likes:>=10 (language:@Rust - language:@Haskell)\'", result);
}