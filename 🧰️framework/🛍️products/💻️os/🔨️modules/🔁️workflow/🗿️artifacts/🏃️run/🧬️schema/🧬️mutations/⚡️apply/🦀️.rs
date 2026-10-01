//! ⚡️ Applies run events through the typed admission boundary.
use crate::{AppendRunLog, FinishRunNode, RunArtifact, RunLogLine, RunMutation, RunStatus, SealRun, StartRun, StartRunNode};

pub fn apply_run_operation(document: &RunArtifact, operation: &RunMutation) -> RunArtifact {
    let mut next = document.clone();
    match operation {
        RunMutation::StartRun(StartRun { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger }) => {
            next.workflow_ref = workflow_ref.clone();
            next.workflow_checkpoint_id = workflow_checkpoint_id.clone();
            next.input_collection_ref = input_collection_ref.clone();
            next.input_snapshot_id = input_snapshot_id.clone();
            next.parameter_values = parameter_values.clone();
            next.output_collection_ref = output_collection_ref.clone();
            next.trigger = trigger.clone();
            next.status = RunStatus::Running;
            next.started_at = store::now_iso();
        }
        RunMutation::StartRunNode(StartRunNode { node_id }) => {
            next.logs.push(RunLogLine { node_id: node_id.clone(), level: "info".into(), message: "node started".into(), at: store::now_iso() });
        }
        RunMutation::FinishRunNode(FinishRunNode { node_record }) => {
            if let Some(existing) = next.node_records.iter_mut().find(|entry| entry.node_id == node_record.node_id) {
                *existing = node_record.clone();
            } else {
                next.node_records.push(node_record.clone());
            }
        }
        RunMutation::AppendRunLog(AppendRunLog { node_id, level, message, at }) => {
            next.logs.push(RunLogLine { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() });
        }
        RunMutation::SealRun(SealRun { status }) => {
            next.status = *status;
            next.finished_at = Some(store::now_iso());
            next.sealed = true;
        }
    }
    next
}

/// 🔒️ The one real write seam for a `RunArtifact`: a refusal travels as the outcome's own messages (codes and levels
/// unchanged), and an apply-time rejection of the same `RunDiff::apply` ordinary application uses joins them as the
/// `Fatal` `mutation.apply.*` message `MutationOutcome::apply_to` would persist.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> Result<RunArtifact, Vec<protocol::MutationMessage>> {
    let (diff, messages) = protocol::Mutation::diff(&operation, document).into_parts();
    if messages.iter().any(|message| protocol::MergePolicy::default().rejects(message.level)) {
        return Err(messages);
    }
    protocol::MutationDiff::apply(&diff, document).map_err(|error| messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect())
}
