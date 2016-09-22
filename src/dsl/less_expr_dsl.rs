use expr::Unescaped;
use less_expr::LessExpr;
use std::borrow::Cow;

pub fn less_expr<'a, T>(column: T, target: T) -> LessExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>>
{
    LessExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use less_expr::LessExpr;

    #[test]
    fn test_less_expr() {
        let syntax = less_expr("n_likes", "5");
        let expected = LessExpr::new("n_likes", "5");
        assert_eq!(expected, syntax);
    }
}
