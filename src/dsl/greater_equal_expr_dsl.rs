use expr::Unescaped;
use greater_equal_expr::GreaterEqualExpr;
use std::borrow::Cow;

pub fn greater_equal_expr<'a, T>(column: T, target: T) -> GreaterEqualExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>> {
    GreaterEqualExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use greater_equal_expr::GreaterEqualExpr;

    #[test]
    fn test_greater_equal_expr() {
        let syntax = greater_equal_expr("n_likes", "5");
        let expected = GreaterEqualExpr::new("n_likes", "5");
        assert_eq!(expected, syntax);
    }
}
