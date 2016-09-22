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
pub struct FulltextExpr<'a, S: Any = Unescaped> {
    target: Cow<'a, str>,
    column: Option<Cow<'a, str>>,
    _marker: PhantomData<S>,
}

impl<'a> FulltextExpr<'a, Unescaped> {
    pub fn new<T>(target: T) -> FulltextExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        FulltextExpr {
            target: target.into(),
            column: None,
            _marker: PhantomData,
        }
    }

    pub fn column<T>(mut self, column: T) -> FulltextExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        self.column = Some(column.into());
        self
    }

    pub fn is_invalid(&self) -> bool {
        self.target.contains(' ')
    }

    /// Prepare grn_expr
    ///
    /// ## Panics
    ///
    /// When target contains ' '(space).
    pub fn prepare(self) -> FulltextExpr<'a, Escaped> {
        if self.is_invalid() {
            panic!("Could not contain space(s) in target.");
        }

        FulltextExpr {
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            column: self.column,
            _marker: PhantomData,
        }
    }
}

impl<'a> FulltextExpr<'a, Escaped> {
    /// Build grn_expr for fulltext search
    pub fn build(self) -> String {
        match self.column {
            Some(c) => format!("{}:@{}", c, self.target.into_owned()),
            None => format!("{}", self.target.into_owned()),
        }
    }
}

impl<'a> Fragmentable for FulltextExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

impl<'a, T: Fragmentable> Add<T> for FulltextExpr<'a, Escaped> {
    type Output = LogicalAndBuilder;
    /// Make LogicalAndBuilder with add operation.
    ///
    /// **@overloaded**
    fn add(self, rhs: T) -> LogicalAndBuilder
    {
        LogicalAndBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> Sub<T> for FulltextExpr<'a, Escaped> {
    type Output = LogicalNotBuilder;
    /// Make LogicalNotBuilder with sub operation.
    ///
    /// **@overloaded**
    fn sub(self, rhs: T) -> LogicalNotBuilder {
        LogicalNotBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Fragmentable> BitOr<T> for FulltextExpr<'a, Escaped> {
    type Output = LogicalOrBuilder;
    /// Make LogicalOrBuilder with bitor operation.
    ///
    /// **@overloaded**
    fn bitor(self, rhs: T) -> LogicalOrBuilder {
        LogicalOrBuilder::new(self.to_fragment(), rhs.to_fragment())
    }
}

impl<'a, T: Groupable> Rem<T> for FulltextExpr<'a, Escaped> {
    type Output = GroupBuilder;
    /// Make GroupBuilder with rem operation.
    ///
    /// **@overloaded**
    fn rem(self, rhs: T) -> GroupBuilder
    {
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
        let expr = FulltextExpr::new("test");
        let expected = FulltextExpr::<Unescaped> {
            target: Cow::Borrowed("test"),
            column: None,
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = FulltextExpr::new("test").prepare();
        let expected = FulltextExpr::<Escaped> {
            target: Cow::Borrowed("test"),
            column: None,
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_is_invalid() {
        let valid_expr = FulltextExpr::new("valid").is_invalid();
        assert_eq!(false, valid_expr);
        let invalid_expr = FulltextExpr::new("an invalid").is_invalid();
        assert_eq!(true, invalid_expr)
    }

    #[test]
    fn test_build() {
        let expr = FulltextExpr::new("test").prepare().build();
        let expected = "test";
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build_with_column() {
        let expr = FulltextExpr::new("test").column("target").prepare().build();
        let expected = "target:@test";
        assert_eq!(expected, expr)
    }
}
