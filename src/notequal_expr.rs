use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotequalExpr<'a, S: Any = Unescaped> {
    column: Cow<'a, str>,
    target: Cow<'a, str>,
    _marker: PhantomData<S>,
}

impl<'a> NotequalExpr<'a, Unescaped> {
    pub fn new<T>(column: T, target: T) -> NotequalExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        NotequalExpr {
            column: column.into(),
            target: target.into(),
            _marker: PhantomData,
        }
    }

    /// Prepare grn_expr
    pub fn prepare(self) -> NotequalExpr<'a, Escaped> {
        NotequalExpr {
            column: self.column,
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            _marker: PhantomData,
        }
    }
}

impl<'a> NotequalExpr<'a, Escaped> {
    /// Build grn_expr for not equal search
    pub fn build(self) -> String {
        format!("{}:!{}", self.column.into_owned(), self.target.into_owned())
    }
}

impl<'a> Fragmentable for NotequalExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

operators!(NotequalExpr);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use expr::{Unescaped, Escaped};

    #[test]
    fn test_new() {
        let expr = NotequalExpr::new("key", "Rust");
        let expected = NotequalExpr::<Unescaped> {
            column: Cow::Borrowed("key"),
            target: Cow::Borrowed("Rust"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = NotequalExpr::new("key", "Rust").prepare();
        let expected = NotequalExpr::<Escaped> {
            column: Cow::Borrowed("key"),
            target: Cow::Borrowed("Rust"),
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build() {
        let expr = NotequalExpr::new("key", "Rust").prepare().build();
        let expected = "key:!Rust";
        assert_eq!(expected, expr)
    }
}
