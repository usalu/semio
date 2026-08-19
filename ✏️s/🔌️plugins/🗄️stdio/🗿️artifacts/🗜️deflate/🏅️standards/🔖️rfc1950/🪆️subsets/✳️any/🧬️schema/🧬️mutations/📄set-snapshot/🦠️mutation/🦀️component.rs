use crate::artifacts::deflate::schema::mutations::{apply_deflate_mutation, DeflateMutation};
use crate::artifacts::deflate::DeflateSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut DeflateSnapshot, mutation: &DeflateMutation) {
    apply_deflate_mutation(projection, mutation);
}
