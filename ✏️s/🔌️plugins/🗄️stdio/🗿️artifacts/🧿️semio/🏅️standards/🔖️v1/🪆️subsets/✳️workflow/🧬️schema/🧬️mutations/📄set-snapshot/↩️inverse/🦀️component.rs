use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::mutations::SemioWorkflowMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
pub fn inverse(base: &SemioWorkflowSnapshot, mutation: &SemioWorkflowMutation) -> Vec<SemioWorkflowMutation> {
    <SemioWorkflowMutation as Mutation<SemioWorkflowSnapshot>>::inverse(mutation, base)
}
