use crate::schema::mutations::DxfMutation;
use crate::DxfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &DxfSnapshot, mutation: &DxfMutation) -> Vec<DxfMutation> {
    <DxfMutation as Mutation<DxfSnapshot>>::inverse(mutation, base)
}
