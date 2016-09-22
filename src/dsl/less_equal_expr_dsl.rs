use expr::Unescaped;
use less_equal_expr::LessEqualExpr;
use std::borrow::Cow;

pub fn less_equal_expr<'a, T>(column: T, target: T) -> LessEqualExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>> {
    LessEqualExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use less_equal_expr::LessEqualExpr;

    #[test]
    fn test_less_equal_expr() {
        let syntax = less_equal_expr("n_likes", "5");
        let expected = LessEqualExpr::new("n_likes", "5");
        assert_eq!(expected, syntax);
    }
}
