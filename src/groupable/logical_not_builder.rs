use groupable::Query;

pub struct LogicalNotBuilder {
    lhs: Query,
    rhs: Query,
}

impl LogicalNotBuilder {
    pub fn new(lhs: Query, rhs: Query) -> LogicalNotBuilder {
        LogicalNotBuilder {
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn build(self) -> String {
        format!("\'{} - {}\'",
                self.lhs.into_iter().map(|e| e).collect::<String>(),
                self.rhs.into_iter().map(|e| e).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use match_expr::MatchExpr;
    use less_expr::LessExpr;
    use groupable::Groupable;

    #[test]
    fn test_build() {
        let lexpr = MatchExpr::new("language", "[pP]ostscript").prepare().unwrap().to_fragment();
        let rexpr = LessExpr::new("n_likes", "10").prepare().to_fragment();
        let logical_and_expr = LogicalNotBuilder::new(lexpr, rexpr);
        let result = logical_and_expr.build();
        let expected = "\'language:~[pP]ostscript - n_likes:<10\'";
        assert_eq!(expected, result);
    }
}
