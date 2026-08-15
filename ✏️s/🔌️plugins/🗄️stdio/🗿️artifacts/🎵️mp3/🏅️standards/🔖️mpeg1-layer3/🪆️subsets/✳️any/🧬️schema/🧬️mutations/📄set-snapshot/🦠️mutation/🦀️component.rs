use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{apply_mp3_mutation, Mp3Mutation};
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut Mp3Snapshot, mutation: &Mp3Mutation) {
    let _ = apply_mp3_mutation(projection, mutation);
}
