use crate::artifacts::semio::standards::v1::subsets::workflow::schema::snapshot::SemioWorkflowSnapshot;
use crate::artifacts::semio::standards::v1::subsets::workflow::schema::mutations::{SemioWorkflowMutation, apply_semio_workflow_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut SemioWorkflowSnapshot, mutation: &SemioWorkflowMutation) {
    let _ = apply_semio_workflow_mutation(projection, mutation);
}
