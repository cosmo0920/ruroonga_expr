use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use regex_syntax::Expr;
use groupable::{Fragmentable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpr<'a, S: Any = Unescaped> {
    column: Cow<'a, str>,
    regex: Cow<'a, str>,
    _marker: PhantomData<S>,
}

impl<'a> MatchExpr<'a, Unescaped> {
    pub fn new<T>(column: T, regex: T) -> MatchExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        MatchExpr {
            column: column.into(),
            regex: regex.into(),
            _marker: PhantomData,
        }
    }

    /// Prepare grn_expr
    pub fn prepare(self) -> Result<MatchExpr<'a, Escaped>, String> {
        match Expr::parse(self.regex.as_ref()) {
            Ok(_) => {
                Ok(MatchExpr {
                    column: self.column,
                    regex: format!("{}", Escape(&*self.regex.into_owned())).into(),
                    _marker: PhantomData,
                })
            }
            Err(e) => Err(format!("{} Reason: {}", "Could not parse regex.", e)),
        }
    }
}

impl<'a> MatchExpr<'a, Escaped> {
    /// Build grn_expr for greater search
    pub fn build(self) -> String {
        format!("{}:~{}", self.column.into_owned(), self.regex.into_owned())
    }
}

impl<'a> Fragmentable for MatchExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

operators!(MatchExpr<'a, Escaped>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use expr::{Unescaped, Escaped};

    #[test]
    fn test_new() {
        let expr = MatchExpr::new("content", ".roonga");
        let expected = MatchExpr::<Unescaped> {
            column: Cow::Borrowed("content"),
            regex: Cow::Borrowed(".roonga"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = MatchExpr::new("content", ".roonga").prepare();
        let expected = Ok(MatchExpr::<Escaped> {
            column: Cow::Borrowed("content"),
            regex: Cow::Borrowed(".roonga"),
            _marker: PhantomData,
        });
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare_with_invalid_regexp() {
        let expr = MatchExpr::new("content", ".(roonga").prepare();
        assert_eq!(false, expr.is_ok())
    }

    #[test]
    fn test_build() {
        let expr = MatchExpr::new("content", ".roonga").prepare().unwrap().build();
        let expected = "content:~.roonga";
        assert_eq!(expected, expr)
    }
}
