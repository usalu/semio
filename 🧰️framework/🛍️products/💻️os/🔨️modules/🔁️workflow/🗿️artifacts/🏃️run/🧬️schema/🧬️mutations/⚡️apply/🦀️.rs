//! ⚡️ The checked run admission seam: a run mutation's own diff is built and its refusals are read here; the diff is applied by
//! [`crate::apply_run_operation_checked`] through the central applier.
use crate::{RunArtifact, RunDiff, RunMutation};

/// 🔒️ Admits one run operation: its own diff plus the messages it produced, or the messages when any of them is a refusal
/// (codes and levels unchanged).
pub fn admit_run_operation(document: &RunArtifact, operation: &RunMutation) -> Result<(RunDiff, Vec<protocol::MutationMessage>), Vec<protocol::MutationMessage>> {
    let (diff, messages) = protocol::Mutation::diff(operation, document).into_parts();
    if messages.iter().any(|message| protocol::MergePolicy::default().rejects(message.level)) {
        return Err(messages);
    }
    Ok((diff, messages))
}
