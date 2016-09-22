use expr::Unescaped;
use greater_expr::GreaterExpr;
use std::borrow::Cow;

pub fn greater_expr<'a, T>(column: T, target: T) -> GreaterExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>> {
    GreaterExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use greater_expr::GreaterExpr;

    #[test]
    fn test_greater_expr() {
        let syntax = greater_expr("n_likes", "5");
        let expected = GreaterExpr::new("n_likes", "5");
        assert_eq!(expected, syntax);
    }
}
