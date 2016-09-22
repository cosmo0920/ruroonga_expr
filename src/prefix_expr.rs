use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Groupable, Query};
use std::ops::{Add, Sub, BitOr, Rem};
use groupable::group_builder::GroupBuilder;
use groupable::logical_or_builder::LogicalOrBuilder;
use groupable::logical_and_builder::LogicalAndBuilder;
use groupable::logical_not_builder::LogicalNotBuilder;

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

impl<'a, T: Fragmentable> Add<T> for PrefixExpr<'a, Escaped> {
    type Output = LogicalAndBuilder;
    /// Make LogicalAndBuilder with add operation.
    ///
    /// **@overloaded**
    fn add(self, rhs: T) -> LogicalAndBuilder {
        LogicalAndBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> Sub<T> for PrefixExpr<'a, Escaped> {
    type Output = LogicalNotBuilder;
    /// Make LogicalNotBuilder with sub operation.
    ///
    /// **@overloaded**
    fn sub(self, rhs: T) -> LogicalNotBuilder {
        LogicalNotBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> BitOr<T> for PrefixExpr<'a, Escaped> {
    type Output = LogicalOrBuilder;
    /// Make LogicalOrBuilder with bitor operation.
    ///
    /// **@overloaded**
    fn bitor(self, rhs: T) -> LogicalOrBuilder {
        LogicalOrBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Groupable> Rem<T> for PrefixExpr<'a, Escaped> {
    type Output = GroupBuilder;
    /// Make GroupBuilder with rem operation.
    ///
    /// **@overloaded**
    fn rem(self, rhs: T) -> GroupBuilder {
        GroupBuilder::new(self.to_fragment(), rhs.to_group())
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
