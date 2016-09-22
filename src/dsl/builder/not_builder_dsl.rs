use groupable::Query;
use groupable::logical_not_builder::LogicalNotBuilder;

pub fn not(lhs: Query, rhs: Query) -> LogicalNotBuilder {
    LogicalNotBuilder::new(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;
    use greater_expr::GreaterExpr;
    use groupable::Fragmentable;
    use groupable::logical_not_builder::LogicalNotBuilder;

    #[test]
    fn test_logical_not() {
        let lexpr = FulltextExpr::new("text").column("freq").prepare().to_fragment();
        let rexpr = GreaterExpr::new("n_likes", "5").prepare().to_fragment();
        let syntax = not(lexpr.clone(), rexpr.clone());
        let logical_not_expr = LogicalNotBuilder::new(lexpr, rexpr);
        assert_eq!(logical_not_expr, syntax);
    }
}
