use crate::artifacts::zip::schema::mutations::ZipMutation;
use crate::artifacts::zip::ZipSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub async fn inverse(base: &ZipSnapshot, mutation: &ZipMutation) -> Vec<ZipMutation> {
    <ZipMutation as Mutation<ZipSnapshot>>::inverse(mutation, base).await
}
