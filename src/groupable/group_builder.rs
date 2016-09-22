//! Create grouping Groonga expression.
//!
//! Put parentheses into outer places:
//!
//! `expr1 expr2` =(grouping)=> `(expr1 expr2)`

use groupable::{Fragmentable, Groupable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupBuilder {
    lhs: Query,
    rhs: Query,
}

impl GroupBuilder {
    pub fn new(lhs: Query, rhs: Query) -> GroupBuilder {
        GroupBuilder {
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn build(self) -> String {
        format!("\'{} {}\'",
                self.lhs.into_iter().map(|e| e).collect::<String>(),
                self.rhs.into_iter().map(|e| e).collect::<String>())
    }
}

impl Fragmentable for GroupBuilder {
    fn to_fragment(self) -> Query {
        vec![format!("{} {}",
                     self.lhs.into_iter().map(|e| e).collect::<String>(),
                     self.rhs.into_iter().map(|e| e).collect::<String>())]
    }
}

impl Groupable for GroupBuilder {
    fn to_group(self) -> Query {
        vec![format!("({})",
                     self.to_fragment().into_iter().map(|e| e).collect::<String>())]
    }
}

// TODO: More std::ops::* definition is needed?

#[cfg(test)]
mod tests {
    use super::*;
    use phrase_expr::PhraseExpr;
    use greater_equal_expr::GreaterEqualExpr;
    use groupable::{Groupable, Fragmentable};
    use groupable::logical_or_builder::LogicalOrBuilder;
    use groupable::logical_and_builder::LogicalAndBuilder;
    use groupable::logical_not_builder::LogicalNotBuilder;

    #[test]
    fn test_build_with_logical_or() {
        let lexpr = PhraseExpr::new("Rust lang").column("language").prepare().to_fragment();
        let rexpr = PhraseExpr::new("Haskell lang").column("language").prepare().to_fragment();
        let logical_or_expr = LogicalOrBuilder::new(lexpr, rexpr).to_group();
        let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
        let expected = concat!("\'n_likes:>=10 ",
                               "(language:@\"Rust lang\" OR language:@\"Haskell lang\")\'");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_build_with_logical_and() {
        let lexpr = PhraseExpr::new("Rust lang").column("language").prepare().to_fragment();
        let rexpr = PhraseExpr::new("Haskell lang").column("language").prepare().to_fragment();
        let logical_or_expr = LogicalAndBuilder::new(lexpr, rexpr).to_group();
        let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
        let expected = concat!("\'n_likes:>=10 ",
                               "(language:@\"Rust lang\" + language:@\"Haskell lang\")\'");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_build_with_logical_not() {
        let lexpr = PhraseExpr::new("Rust lang").column("language").prepare().to_fragment();
        let rexpr = PhraseExpr::new("Haskell lang").column("language").prepare().to_fragment();
        let logical_or_expr = LogicalNotBuilder::new(lexpr, rexpr).to_group();
        let comb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let result = GroupBuilder::new(comb_lexpr, logical_or_expr).build();
        let expected = concat!("\'n_likes:>=10 ",
                               "(language:@\"Rust lang\" - language:@\"Haskell lang\")\'");
        assert_eq!(expected, result);
    }

    #[test]
    fn test_build_with_nested_logicals() {
        let llexpr = PhraseExpr::new("Rust lang").column("language").prepare().to_fragment();
        let lrexpr = PhraseExpr::new("Haskell lang").column("language").prepare().to_fragment();
        let llogical_or_expr = LogicalOrBuilder::new(llexpr, lrexpr).to_group();
        let lcomb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let llogical_expr = GroupBuilder::new(lcomb_lexpr, llogical_or_expr);
        assert_eq!(concat!("\'n_likes:>=10 ",
                           "(language:@\"Rust lang\" OR language:@\"Haskell lang\")\'"),
                   llogical_expr.clone().build());

        let rlexpr = PhraseExpr::new("Ruby lang").column("language").prepare().to_fragment();
        let rrexpr = PhraseExpr::new("Python lang").column("language").prepare().to_fragment();
        let rlogical_or_expr = LogicalOrBuilder::new(rlexpr, rrexpr).to_group();
        let rcomb_lexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let rlogical_expr = GroupBuilder::new(rcomb_lexpr, rlogical_or_expr);
        assert_eq!(concat!("\'n_likes:>=10 ",
                           "(language:@\"Ruby lang\" OR language:@\"Python lang\")\'"),
                   rlogical_expr.clone().build());

        let result = LogicalAndBuilder::new(llogical_expr.to_group(), rlogical_expr.to_group())
            .build();
        let expected = concat!("\'(n_likes:>=10 ",
                               "(language:@\"Rust lang\" OR language:@\"Haskell lang\")) ",
                               "+ (n_likes:>=10 ",
                               "(language:@\"Ruby lang\" OR language:@\"Python lang\"))\'");
        assert_eq!(expected, result);
    }
}
