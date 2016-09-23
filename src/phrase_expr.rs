use std::borrow::Cow;
use exprable::Escape;
use std::any::Any;
use std::marker::PhantomData;
use expr::{Unescaped, Escaped};
use groupable::{Fragmentable, Query};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhraseExpr<'a, S: Any = Unescaped> {
    target: Cow<'a, str>,
    column: Option<Cow<'a, str>>,
    _marker: PhantomData<S>,
}

impl<'a> PhraseExpr<'a, Unescaped> {
    pub fn new<T>(target: T) -> PhraseExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        PhraseExpr {
            target: target.into(),
            column: None,
            _marker: PhantomData,
        }
    }

    pub fn column<T>(mut self, column: T) -> PhraseExpr<'a, Unescaped>
        where T: Into<Cow<'a, str>>
    {
        self.column = Some(column.into());
        self
    }

    pub fn is_invalid(&self) -> bool {
        !self.target.contains(' ')
    }

    /// Prepare grn_expr
    ///
    /// ## Panics
    ///
    /// When target does not contain ' '(space).
    pub fn prepare(self) -> Result<PhraseExpr<'a, Escaped>, String> {
        if self.is_invalid() {
            return Err("Target must be contain space(s).".to_string());
        }

        Ok(PhraseExpr {
            target: format!("{}", Escape(&*self.target.into_owned())).into(),
            column: self.column,
            _marker: PhantomData,
        })
    }
}

impl<'a> PhraseExpr<'a, Escaped> {
    /// Build grn_expr for phrase search
    pub fn build(self) -> String {
        match self.column {
            Some(c) => format!("{}:@\"{}\"", c, self.target.into_owned()),
            None => format!("\"{}\"", self.target.into_owned()),
        }
    }
}

impl<'a> Fragmentable for PhraseExpr<'a, Escaped> {
    fn to_fragment(self) -> Query {
        vec![self.build()]
    }
}

operators!(PhraseExpr<'a, Escaped>);

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use expr::{Unescaped, Escaped};

    #[test]
    fn test_new() {
        let expr = PhraseExpr::new("a phrase");
        let expected = PhraseExpr::<Unescaped> {
            target: Cow::Borrowed("a phrase"),
            column: None,
            _marker: PhantomData,
        };
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_prepare() {
        let expr = PhraseExpr::new("a phrase").prepare();
        let expected = Ok(PhraseExpr::<Escaped> {
            target: Cow::Borrowed("a phrase"),
            column: None,
            _marker: PhantomData,
        });
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_is_invalid() {
        let valid_expr = PhraseExpr::new("a valid").is_invalid();
        assert_eq!(false, valid_expr);
        let invalid_expr = PhraseExpr::new("invalid").is_invalid();
        assert_eq!(true, invalid_expr)
    }

    #[test]
    fn test_build() {
        let expr = PhraseExpr::new("a phrase").prepare().unwrap().build();
        let expected = "\"a phrase\"";
        assert_eq!(expected, expr)
    }

    #[test]
    fn test_build_with_column() {
        let expr = PhraseExpr::new("a phrase").column("target").prepare().unwrap().build();
        let expected = "target:@\"a phrase\"";
        assert_eq!(expected, expr)
    }
}
