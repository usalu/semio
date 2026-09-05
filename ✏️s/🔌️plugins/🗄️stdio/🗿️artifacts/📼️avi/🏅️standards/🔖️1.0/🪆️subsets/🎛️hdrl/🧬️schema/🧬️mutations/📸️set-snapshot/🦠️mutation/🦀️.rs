use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::{apply_avi_mutation, AviMutation};
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut AviSnapshot, mutation: &AviMutation) {
    let _ = apply_avi_mutation(projection, mutation);
}
