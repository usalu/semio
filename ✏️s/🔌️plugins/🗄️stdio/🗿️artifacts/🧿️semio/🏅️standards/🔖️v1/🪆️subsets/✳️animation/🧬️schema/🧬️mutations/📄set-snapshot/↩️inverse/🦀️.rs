use crate::artifacts::semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioAnimationSnapshot, mutation: &SemioAnimationMutation) -> Vec<SemioAnimationMutation> {
    <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(mutation, base)
}
