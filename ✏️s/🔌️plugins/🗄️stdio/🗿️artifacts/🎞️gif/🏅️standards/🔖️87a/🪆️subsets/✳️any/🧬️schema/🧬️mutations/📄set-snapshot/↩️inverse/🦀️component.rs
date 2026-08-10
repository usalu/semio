use crate::artifacts::gif::{GifSnapshot};
use crate::artifacts::gif::schema::mutations::GifMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &GifSnapshot, mutation: &GifMutation) -> Vec<GifMutation> {
    <GifMutation as Mutation<GifSnapshot>>::inverse(mutation, base)
}
