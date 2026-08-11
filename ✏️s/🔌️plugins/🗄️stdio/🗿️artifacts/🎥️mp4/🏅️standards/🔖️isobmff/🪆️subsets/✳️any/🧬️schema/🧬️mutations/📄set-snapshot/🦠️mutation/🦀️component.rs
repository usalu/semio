use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::{Mp4Mutation, apply_mp4_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut Mp4Snapshot, mutation: &Mp4Mutation) {
    let _ = apply_mp4_mutation(projection, mutation);
}
