use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::{Mp3Mutation, apply_mp3_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut Mp3Snapshot, mutation: &Mp3Mutation) {
    let _ = apply_mp3_mutation(projection, mutation);
}
