use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation;
use crate::artifacts::mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &Mp3Snapshot, mutation: &Mp3Mutation) -> Vec<Mp3Mutation> {
    <Mp3Mutation as Mutation<Mp3Snapshot>>::inverse(mutation, base)
}
