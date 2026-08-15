use crate::artifacts::xlsx::schema::mutations::XlsxMutation;
use crate::artifacts::xlsx::XlsxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &XlsxSnapshot, mutation: &XlsxMutation) -> Vec<XlsxMutation> {
    <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(mutation, base)
}
