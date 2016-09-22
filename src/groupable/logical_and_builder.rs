use groupable::Query;

pub struct LogicalAndBuilder {
    lhs: Query,
    rhs: Query,
}

impl LogicalAndBuilder {
    pub fn new(lhs: Query, rhs: Query) -> LogicalAndBuilder {
        LogicalAndBuilder {
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn build(self) -> String {
        format!("\'{} + {}\'",
                self.lhs.into_iter().map(|e| e).collect::<String>(),
                self.rhs.into_iter().map(|e| e).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;
    use greater_expr::GreaterExpr;
    use groupable::Groupable;

    #[test]
    fn test_build() {
        let lexpr = FulltextExpr::new("text").column("freq").prepare().to_fragment();
        let rexpr = GreaterExpr::new("n_likes", "5").prepare().to_fragment();
        let logical_and_expr = LogicalAndBuilder::new(lexpr, rexpr);
        let result = logical_and_expr.build();
        let expected = "\'freq:@text + n_likes:>5\'";
        assert_eq!(expected, result);
    }
}