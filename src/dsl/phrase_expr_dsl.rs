use expr::Unescaped;
use phrase_expr::PhraseExpr;
use std::borrow::Cow;

pub fn phrase_expr<'a, T>(target: T) -> PhraseExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>>
{
    PhraseExpr::new(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use phrase_expr::PhraseExpr;

    #[test]
    fn test_phrase_expr() {
        let syntax = phrase_expr("a phrase");
        let expected = PhraseExpr::new("a phrase");
        assert_eq!(expected, syntax);
    }
}
