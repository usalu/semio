use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioFlowSnapshot, mutation: &SemioFlowMutation) -> Vec<SemioFlowMutation> {
    <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(mutation, base)
}
