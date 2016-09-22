use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpr<'a, S: Any = Unescaped> {
    column: Cow<'a, str>,
    target: Cow<'a, str>,
    _marker: PhantomData<S>,
}

impl<'a> PrefixExpr<'a, Unescaped> {
    pub fn new<T>(column: T, target: T) -> PrefixExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        PrefixExpr {
            column: column.into(),
            target: target.into(),
            _marker: PhantomData,
        }
    }

    /// Prepare grn_expr
    pub fn prepare(self) -> PrefixExpr<'a, Escaped> {
        PrefixExpr {
            column: self.column,
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            _marker: PhantomData,
        }
    }
}

impl<'a> PrefixExpr<'a, Escaped> {
    /// Build grn_expr for prefix search
    pub fn build(self) -> String {
        format!("{}:^{}", self.column.into_owned(), self.target.into_owned())
    }
}

impl<'a> Fragmentable for PrefixExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

operators!(PrefixExpr);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use expr::{Unescaped, Escaped};

    #[test]
    fn test_new() {
        let expr = PrefixExpr::new("key", "Rust");
        let expected = PrefixExpr::<Unescaped> {
            column: Cow::Borrowed("key"),
            target: Cow::Borrowed("Rust"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = PrefixExpr::new("key", "Rust").prepare();
        let expected = PrefixExpr::<Escaped> {
            column: Cow::Borrowed("key"),
            target: Cow::Borrowed("Rust"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build() {
        let expr = PrefixExpr::new("key", "Rust").prepare().build();
        let expected = "key:^Rust";
        assert_eq!(expected, expr)
    }
}
