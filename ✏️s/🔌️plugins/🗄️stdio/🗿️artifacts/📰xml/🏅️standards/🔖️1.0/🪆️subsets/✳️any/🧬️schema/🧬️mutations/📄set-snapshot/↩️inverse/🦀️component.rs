use crate::artifacts::xml::schema::mutations::XmlMutation;
use crate::artifacts::xml::XmlSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &XmlSnapshot, mutation: &XmlMutation) -> Vec<XmlMutation> {
    <XmlMutation as Mutation<XmlSnapshot>>::inverse(mutation, base).await
}
