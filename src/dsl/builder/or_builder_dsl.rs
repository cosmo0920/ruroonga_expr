use groupable::Query;
use groupable::logical_or_builder::LogicalOrBuilder;

pub fn or(lhs: Query, rhs: Query) -> LogicalOrBuilder {
    LogicalOrBuilder::new(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;
    use greater_expr::GreaterExpr;
    use groupable::Fragmentable;
    use groupable::logical_or_builder::LogicalOrBuilder;

    #[test]
    fn test_logical_or() {
        let lexpr = FulltextExpr::new("text").column("freq").prepare().to_fragment();
        let rexpr = GreaterExpr::new("n_likes", "5").prepare().to_fragment();
        let syntax = or(lexpr.clone(), rexpr.clone());
        let logical_or_expr = LogicalOrBuilder::new(lexpr, rexpr);
        assert_eq!(logical_or_expr, syntax);
    }
}
