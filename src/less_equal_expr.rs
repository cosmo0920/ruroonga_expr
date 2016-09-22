use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessEqualExpr<'a, S: Any = Unescaped> {
    column: Cow<'a, str>,
    target: Cow<'a, str>,
    _marker: PhantomData<S>,
}

impl<'a> LessEqualExpr<'a, Unescaped> {
    pub fn new<T>(column: T, target: T) -> LessEqualExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        LessEqualExpr {
            column: column.into(),
            target: target.into(),
            _marker: PhantomData,
        }
    }

    /// Prepare grn_expr
    pub fn prepare(self) -> LessEqualExpr<'a, Escaped> {
        LessEqualExpr {
            column: self.column,
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            _marker: PhantomData,
        }
    }
}

impl<'a> LessEqualExpr<'a, Escaped> {
    /// Build grn_expr for less equal search
    pub fn build(self) -> String {
        format!("{}:<={}",
                self.column.into_owned(),
                self.target.into_owned())
    }
}

impl<'a> Fragmentable for LessEqualExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use expr::{Unescaped, Escaped};

    #[test]
    fn test_new() {
        let expr = LessEqualExpr::new("n_likes", "10");
        let expected = LessEqualExpr::<Unescaped> {
            column: Cow::Borrowed("n_likes"),
            target: Cow::Borrowed("10"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = LessEqualExpr::new("n_likes", "10").prepare();
        let expected = LessEqualExpr::<Escaped> {
            column: Cow::Borrowed("n_likes"),
            target: Cow::Borrowed("10"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build() {
        let expr = LessEqualExpr::new("n_likes", "10").prepare().build();
        let expected = "n_likes:<=10";
        assert_eq!(expected, expr)
    }
}
