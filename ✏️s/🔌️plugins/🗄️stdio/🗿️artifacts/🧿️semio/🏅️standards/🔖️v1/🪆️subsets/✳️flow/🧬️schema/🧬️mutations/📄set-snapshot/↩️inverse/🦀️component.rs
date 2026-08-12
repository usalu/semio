use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioFlowSnapshot, mutation: &SemioFlowMutation) -> Vec<SemioFlowMutation> {
    <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(mutation, base)
}
