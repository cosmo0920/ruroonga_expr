use groupable::Query;

pub struct LogicalOrBuilder {
    lhs: Query,
    rhs: Query,
}

impl LogicalOrBuilder {
    pub fn new(lhs: Query, rhs: Query) -> LogicalOrBuilder {
        LogicalOrBuilder {
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn build(self) -> String {
        format!("\'{} OR {}\'",
                self.lhs.into_iter().map(|e| e).collect::<String>(),
                self.rhs.into_iter().map(|e| e).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phrase_expr::PhraseExpr;
    use greater_equal_expr::GreaterEqualExpr;
    use groupable::Groupable;

    #[test]
    fn test_build() {
        let lexpr = PhraseExpr::new("Rust lang").column("language").prepare().to_fragment();
        let rexpr = GreaterEqualExpr::new("n_likes", "10").prepare().to_fragment();
        let logical_and_expr = LogicalOrBuilder::new(lexpr, rexpr);
        let result = logical_and_expr.build();
        let expected = "\'language:@\"Rust lang\" OR n_likes:>=10\'";
        assert_eq!(expected, result);
    }
}