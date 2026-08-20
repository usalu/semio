use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation;
use crate::artifacts::mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &Mp4Snapshot, mutation: &Mp4Mutation) -> Vec<Mp4Mutation> {
    <Mp4Mutation as Mutation<Mp4Snapshot>>::inverse(mutation, base)
}
