use crate::artifacts::xml::schema::mutations::XmlMutation;
use crate::artifacts::xml::XmlSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &XmlSnapshot, mutation: &XmlMutation) -> Vec<XmlMutation> {
    <XmlMutation as Mutation<XmlSnapshot>>::inverse(mutation, base)
}
