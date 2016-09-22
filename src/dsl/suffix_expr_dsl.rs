use expr::Unescaped;
use suffix_expr::SuffixExpr;
use std::borrow::Cow;

pub fn suffix_expr<'a, T>(column: T,target: T) -> SuffixExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>> {
    SuffixExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use suffix_expr::SuffixExpr;

    #[test]
    fn test_suffix_expr() {
        let syntax = suffix_expr("key", "Rust");
        let expected = SuffixExpr::new("key", "Rust");
        assert_eq!(expected, syntax);
    }
}
