use crate::artifacts::bcf::schema::mutations::{apply_bcf_mutation, BcfMutation};
use crate::artifacts::bcf::BcfSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut BcfSnapshot, mutation: &BcfMutation) {
    apply_bcf_mutation(projection, mutation);
}
