use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot;
use crate::artifacts::gif::standards::v89a::subsets::any::schema::mutations::GifMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &GifSnapshot, mutation: &GifMutation) -> Vec<GifMutation> {
    <GifMutation as Mutation<GifSnapshot>>::inverse(mutation, base)
}
