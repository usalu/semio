use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::{apply_mp4_mutation, Mp4Mutation};
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut Mp4Snapshot, mutation: &Mp4Mutation) {
    let _ = apply_mp4_mutation(projection, mutation);
}
