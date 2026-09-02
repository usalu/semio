use crate::artifacts::tsv::standards::iana::subsets::any::schema::mutations::{apply_tsv_mutation, TsvMutation};
use crate::artifacts::tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut TsvSnapshot, mutation: &TsvMutation) {
    let _ = apply_tsv_mutation(projection, mutation);
}
