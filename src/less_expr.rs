use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Query};
use std::ops::{Add, Sub, BitOr};
use groupable::logical_or_builder::LogicalOrBuilder;
use groupable::logical_and_builder::LogicalAndBuilder;
use groupable::logical_not_builder::LogicalNotBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessExpr<'a, S: Any = Unescaped> {
    column: Cow<'a, str>,
    target: Cow<'a, str>,
    _marker: PhantomData<S>,
}

impl<'a> LessExpr<'a, Unescaped> {
    pub fn new<T>(column: T, target: T) -> LessExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        LessExpr {
            column: column.into(),
            target: target.into(),
            _marker: PhantomData,
        }
    }

    /// Prepare grn_expr
    pub fn prepare(self) -> LessExpr<'a, Escaped> {
        LessExpr {
            column: self.column,
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            _marker: PhantomData,
        }
    }
}

impl<'a> LessExpr<'a, Escaped> {
    /// Build grn_expr for less search
    pub fn build(self) -> String {
        format!("{}:<{}", self.column.into_owned(), self.target.into_owned())
    }
}

impl<'a> Fragmentable for LessExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

impl<'a, T: Fragmentable> Add<T>  for LessExpr<'a, Escaped> {
    type Output = LogicalAndBuilder;
    fn add(self, rhs: T) -> LogicalAndBuilder {
        LogicalAndBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> Sub<T> for LessExpr<'a, Escaped> {
    type Output = LogicalNotBuilder;
    fn sub(self, rhs: T) -> LogicalNotBuilder {
        LogicalNotBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> BitOr<T>  for LessExpr<'a, Escaped> {
    type Output = LogicalOrBuilder;
    fn bitor(self, rhs: T) -> LogicalOrBuilder {
        LogicalOrBuilder::new(self.to_fragment(), rhs.to_fragment())
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
        let expr = LessExpr::new("n_likes", "10");
        let expected = LessExpr::<Unescaped> {
            column: Cow::Borrowed("n_likes"),
            target: Cow::Borrowed("10"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = LessExpr::new("n_likes", "10").prepare();
        let expected = LessExpr::<Escaped> {
            column: Cow::Borrowed("n_likes"),
            target: Cow::Borrowed("10"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build() {
        let expr = LessExpr::new("n_likes", "10").prepare().build();
        let expected = "n_likes:<10";
        assert_eq!(expected, expr)
    }
}
