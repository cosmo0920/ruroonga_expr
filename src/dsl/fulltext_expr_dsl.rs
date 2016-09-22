use expr::Unescaped;
use fulltext_expr::FulltextExpr;
use std::borrow::Cow;

pub fn fulltext_expr<'a, T>(target: T) -> FulltextExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>>
{
    FulltextExpr::new(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fulltext_expr::FulltextExpr;

    #[test]
    fn test_fulltext_expr() {
        let syntax = fulltext_expr("target");
        let expected = FulltextExpr::new("target");
        assert_eq!(expected, syntax);
    }
}
