use groupable::Query;
use groupable::group_builder::GroupBuilder;

pub fn group(lhs: Query, rhs: Query) -> GroupBuilder {
    GroupBuilder::new(lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;
    use greater_expr::GreaterExpr;
    use groupable::Fragmentable;
    use groupable::group_builder::GroupBuilder;

    #[test]
    fn test_logical_or() {
        let lexpr = FulltextExpr::new("text").column("freq").prepare().unwrap().to_fragment();
        let rexpr = GreaterExpr::new("n_likes", "5").prepare().to_fragment();
        let syntax = group(lexpr.clone(), rexpr.clone());
        let logical_or_expr = GroupBuilder::new(lexpr, rexpr);
        assert_eq!(logical_or_expr, syntax);
    }
}
