use crate::artifacts::avi::standards::v1_0::subsets::any::schema::mutations::AviMutation;
use crate::artifacts::avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &AviSnapshot, mutation: &AviMutation) -> Vec<AviMutation> {
    <AviMutation as Mutation<AviSnapshot>>::inverse(mutation, base)
}
