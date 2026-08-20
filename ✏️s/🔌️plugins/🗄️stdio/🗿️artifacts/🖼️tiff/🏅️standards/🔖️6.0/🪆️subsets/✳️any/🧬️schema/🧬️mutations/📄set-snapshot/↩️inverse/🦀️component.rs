use crate::artifacts::tiff::schema::mutations::TiffMutation;
use crate::artifacts::tiff::TiffSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &TiffSnapshot, mutation: &TiffMutation) -> Vec<TiffMutation> {
    <TiffMutation as Mutation<TiffSnapshot>>::inverse(mutation, base).await
}
