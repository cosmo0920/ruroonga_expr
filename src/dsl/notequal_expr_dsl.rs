use expr::Unescaped;
use notequal_expr::NotequalExpr;
use std::borrow::Cow;

pub fn notequal_expr<'a, T>(column: T, target: T) -> NotequalExpr<'a, Unescaped>
    where T: Into<Cow<'a, str>>
{
    NotequalExpr::new(column, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use notequal_expr::NotequalExpr;

    #[test]
    fn test_notequal_expr() {
        let syntax = notequal_expr("key", "target");
        let expected = NotequalExpr::new("key", "target");
        assert_eq!(expected, syntax);
    }
}
