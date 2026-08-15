use crate::artifacts::dxf::schema::mutations::DxfMutation;
use crate::artifacts::dxf::DxfSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &DxfSnapshot, mutation: &DxfMutation) -> Vec<DxfMutation> {
    <DxfMutation as Mutation<DxfSnapshot>>::inverse(mutation, base)
}
