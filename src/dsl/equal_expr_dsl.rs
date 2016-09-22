use expr::Unescaped;
use equal_expr::EqualExpr;
use std::borrow::Cow;

pub fn equal_expr<'a, T>(column: T, target: T) -> EqualExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>> {
    EqualExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use equal_expr::EqualExpr;

    #[test]
    fn test_equal_expr() {
        let syntax = equal_expr("key", "target");
        let expected = EqualExpr::new("key", "target");
        assert_eq!(expected, syntax);
    }
}
