use ruroonga_expr::dsl::*;
use ruroonga_expr::groupable::{Groupable, Fragmentable};

#[test]
fn test_build_with_nested_logicals() {

    let llexpr = phrase_expr("Rust lang").column("language").prepare().to_fragment();
    let lrexpr = phrase_expr("Haskell lang").column("language").prepare().to_fragment();
    let llogical_or_expr = logical::or(llexpr, lrexpr).to_group();
    let lcomb_lexpr = greater_equal_expr("n_likes", "10").prepare().to_fragment();
    let llogical_expr = logical::group(lcomb_lexpr, llogical_or_expr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Rust lang\" OR language:@\"Haskell lang\")\'"),
               llogical_expr.clone().build());

    let rlexpr = phrase_expr("Ruby lang").column("language").prepare().to_fragment();
    let rrexpr = phrase_expr("Python lang").column("language").prepare().to_fragment();
    let rlogical_or_expr = logical::or(rlexpr, rrexpr).to_group();
    let rcomb_lexpr = greater_equal_expr("n_likes", "10").prepare().to_fragment();
    let rlogical_expr = logical::group(rcomb_lexpr, rlogical_or_expr);
    assert_eq!(concat!("\'n_likes:>=10 ",
                       "(language:@\"Ruby lang\" OR language:@\"Python lang\")\'"),
               rlogical_expr.clone().build());

    let result = logical::and(llogical_expr.to_group(),
                              rlogical_expr.to_group()).build();
    let expected = concat!("\'(n_likes:>=10 ",
                           "(language:@\"Rust lang\" OR language:@\"Haskell lang\")) ",
                           "+ (n_likes:>=10 ",
                           "(language:@\"Ruby lang\" OR language:@\"Python lang\"))\'");
    assert_eq!(expected, result);
}
