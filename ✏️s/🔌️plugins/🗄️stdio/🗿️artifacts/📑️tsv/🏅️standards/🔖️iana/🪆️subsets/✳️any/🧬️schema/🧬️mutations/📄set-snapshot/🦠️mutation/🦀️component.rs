use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;
use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{TsvMutation, apply_tsv_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut TsvSnapshot, mutation: &TsvMutation) {
    let _ = apply_tsv_mutation(projection, mutation);
}
