use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, TsvMutation};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut TsvSnapshot, mutation: &TsvMutation) {
    let _ = apply_tsv_mutation(projection, mutation);
}
