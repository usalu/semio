//! ⚡️ The checked run admission seam: a run mutation's own diff goes through the central applier.
use crate::RunArtifact;
use crate::RunMutation;

/// 🔒️ The one real write seam for a `RunArtifact`: a refusal travels as the outcome's own messages (codes and levels
/// unchanged), and an apply-time rejection of the same `RunDiff` ordinary application uses joins them as the `Fatal`
/// `mutation.apply.*` message the store persists.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> Result<RunArtifact, Vec<protocol::MutationMessage>> {
    let (diff, messages) = protocol::Mutation::diff(&operation, document).into_parts();
    if messages.iter().any(|message| protocol::MergePolicy::default().rejects(message.level)) {
        return Err(messages);
    }
    protocol::apply_diff(&diff, document).map_err(|error| messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect())
}
