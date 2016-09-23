macro_rules! operators { ($t:ty) => {
    impl<'a, T: $crate::groupable::Fragmentable> ::std::ops::Add<T> for $t {
        type Output = $crate::groupable::logical_and_builder::LogicalAndBuilder;
/// Make LogicalAndBuilder with add operation.
///
/// **@overloaded**
        fn add(self, rhs: T) -> $crate::groupable::logical_and_builder::LogicalAndBuilder {
            $crate::groupable::logical_and_builder::LogicalAndBuilder::new(self.to_fragment(), rhs.to_fragment())
        }
    }

    impl<'a, T: $crate::groupable::Fragmentable> ::std::ops::Sub<T> for $t {
        type Output = $crate::groupable::logical_not_builder::LogicalNotBuilder;
/// Make LogicalNotBuilder with sub operation.
///
/// **@overloaded**
        fn sub(self, rhs: T) -> $crate::groupable::logical_not_builder::LogicalNotBuilder {
            $crate::groupable::logical_not_builder::LogicalNotBuilder::new(self.to_fragment(), rhs.to_fragment())
        }
    }

    impl<'a, T: $crate::groupable::Fragmentable> ::std::ops::BitOr<T> for $t {
        type Output = $crate::groupable::logical_or_builder::LogicalOrBuilder;
/// Make LogicalOrBuilder with bitor operation.
///
/// **@overloaded**
        fn bitor(self, rhs: T) -> $crate::groupable::logical_or_builder::LogicalOrBuilder {
            $crate::groupable::logical_or_builder::LogicalOrBuilder::new(self.to_fragment(), rhs.to_fragment())
        }
    }

    impl<'a, T: $crate::groupable::Groupable> ::std::ops::Rem<T> for $t {
        type Output = $crate::groupable::group_builder::GroupBuilder;
/// Make GroupBuilder with rem operation.
///
/// **@overloaded**
        fn rem(self, rhs: T) -> $crate::groupable::group_builder::GroupBuilder {
            $crate::groupable::group_builder::GroupBuilder::new(self.to_fragment(), rhs.to_group())
        }
    }
} }
