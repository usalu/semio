use crate::artifacts::xml::{XmlSnapshot};
use crate::artifacts::xml::schema::mutations::XmlMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &XmlSnapshot, mutation: &XmlMutation) -> Vec<XmlMutation> {
    <XmlMutation as Mutation<XmlSnapshot>>::inverse(mutation, base)
}
