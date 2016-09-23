use groupable::Query;
use groupable::logical_and_builder::LogicalAndBuilder;

pub fn and(lhs: Query, rhs: Query) -> LogicalAndBuilder {
    LogicalAndBuilder::new(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;
    use greater_expr::GreaterExpr;
    use groupable::Fragmentable;
    use groupable::logical_and_builder::LogicalAndBuilder;

    #[test]
    fn test_logical_and() {
        let lexpr = FulltextExpr::new("text").column("freq").prepare().unwrap().to_fragment();
        let rexpr = GreaterExpr::new("n_likes", "5").prepare().to_fragment();
        let syntax = and(lexpr.clone(), rexpr.clone());
        let logical_and_expr = LogicalAndBuilder::new(lexpr, rexpr);
        assert_eq!(logical_and_expr, syntax);
    }
}
