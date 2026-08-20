use crate::artifacts::semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation;
use crate::artifacts::semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioVideoSnapshot, mutation: &SemioVideoMutation) -> Vec<SemioVideoMutation> {
    <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(mutation, base)
}
