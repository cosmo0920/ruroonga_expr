pub mod logical_and_builder;
pub mod logical_or_builder;
pub mod logical_not_builder;
pub mod group_builder;

pub type Query = Vec<String>;
pub trait Fragmentable {
    /// Make groonga expression fragment.
    fn to_fragment(self) -> Query;
}

pub trait Groupable {
    /// Make groupable groonga expression fragment.
    fn to_group(self) -> Query;
}
