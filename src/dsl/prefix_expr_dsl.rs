use expr::Unescaped;
use prefix_expr::PrefixExpr;
use std::borrow::Cow;

pub fn prefix_expr<'a, T>(column: T, target: T) -> PrefixExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>>
{
    PrefixExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use prefix_expr::PrefixExpr;

    #[test]
    fn test_prefix_expr() {
        let syntax = prefix_expr("key", "Rust");
        let expected = PrefixExpr::new("key", "Rust");
        assert_eq!(expected, syntax);
    }
}
