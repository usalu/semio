use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::PngSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &PngSnapshot, mutation: &PngMutation) -> Vec<PngMutation> {
    <PngMutation as Mutation<PngSnapshot>>::inverse(mutation, base).await
}
