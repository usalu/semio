use crate::artifacts::png::{PngSnapshot};
use crate::artifacts::png::schema::mutations::PngMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &PngSnapshot, mutation: &PngMutation) -> Vec<PngMutation> {
    <PngMutation as Mutation<PngSnapshot>>::inverse(mutation, base)
}
