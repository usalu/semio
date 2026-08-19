use crate::artifacts::dwg::schema::mutations::DwgMutation;
use crate::artifacts::dwg::DwgSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &DwgSnapshot, mutation: &DwgMutation) -> Vec<DwgMutation> {
    <DwgMutation as Mutation<DwgSnapshot>>::inverse(mutation, base)
}
