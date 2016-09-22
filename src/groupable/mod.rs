pub mod logical_and_builder;
pub mod logical_or_builder;
pub mod logical_not_builder;

pub type Query = Vec<String>;
pub trait Groupable {
    fn to_fragment(self) -> Query;
}
