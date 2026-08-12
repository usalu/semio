use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{SemioFlowMutation, apply_semio_flow_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioFlowSnapshot, mutation: &SemioFlowMutation) {
    let _ = apply_semio_flow_mutation(projection, mutation);
}
