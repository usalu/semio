use crate::artifacts::xlsx::schema::mutations::XlsxMutation;
use crate::artifacts::xlsx::XlsxSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &XlsxSnapshot, mutation: &XlsxMutation) -> Vec<XlsxMutation> {
    <XlsxMutation as Mutation<XlsxSnapshot>>::inverse(mutation, base)
}
