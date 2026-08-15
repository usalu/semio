use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{apply_semio_flow_mutation, SemioFlowMutation};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioFlowSnapshot, mutation: &SemioFlowMutation) {
    let _ = apply_semio_flow_mutation(projection, mutation);
}
