use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::BmpSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &BmpSnapshot, mutation: &BmpMutation) -> Vec<BmpMutation> {
    <BmpMutation as Mutation<BmpSnapshot>>::inverse(mutation, base)
}
