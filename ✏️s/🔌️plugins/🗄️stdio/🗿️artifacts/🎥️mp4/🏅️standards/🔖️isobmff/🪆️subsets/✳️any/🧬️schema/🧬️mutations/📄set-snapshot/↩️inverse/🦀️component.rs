use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &Mp4Snapshot, mutation: &Mp4Mutation) -> Vec<Mp4Mutation> {
    <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(mutation, base)
}
