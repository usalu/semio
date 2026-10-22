//! ⚡️ The checked run admission seam above the run mutation vocabulary: a run mutation's own diff is built and its refusals are read here; the diff is applied by
//! [`apply_run_operation_checked`] through the central applier.

extern crate semio_framework_os_kernel as protocol;

use semio_framework_artifact_workflow_run::{RunArtifact, RunDiff, RunMutation};

/// 🔒️ Admits one run operation: its own diff plus the messages it produced, or the messages when any of them is a refusal
/// (codes and levels unchanged).
pub fn admit_run_operation(document: &RunArtifact, operation: &RunMutation) -> Result<(RunDiff, Vec<protocol::MutationMessage>), Vec<protocol::MutationMessage>> {
    let (diff, messages) = <RunMutation as protocol::ColdMutation<RunArtifact>>::diff(operation, document).into_parts();
    if messages.iter().any(|message| protocol::MergePolicy::default().rejects(message.level)) {
        return Err(messages);
    }
    Ok((diff, messages))
}

/// 🔒️ The one real write seam for a `RunArtifact`: a refusal travels as the outcome's own messages (codes and levels
/// unchanged), and an apply-time rejection of the same `RunDiff` ordinary application uses joins them as the `Fatal`
/// `mutation.apply.*` message the store persists.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> Result<RunArtifact, Vec<protocol::MutationMessage>> {
    let (diff, messages) = admit_run_operation(document, &operation)?;
    protocol::apply_diff_cold(&diff, document).map_err(|error| messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect())
}
