use crate::artifacts::bmp::schema::mutations::{apply_bmp_mutation, BmpMutation};
use crate::artifacts::bmp::BmpSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut BmpSnapshot, mutation: &BmpMutation) {
    apply_bmp_mutation(projection, mutation);
}
