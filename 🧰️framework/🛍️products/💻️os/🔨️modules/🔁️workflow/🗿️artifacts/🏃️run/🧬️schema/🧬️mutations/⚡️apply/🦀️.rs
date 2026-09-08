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

/// 🔒️ The one real write seam for a `RunArtifact`: preserves rejecting outcome diagnostics and
/// delegates every admission decision to the same `RunDiff::apply` implementation as ordinary
/// mutation application.
pub async fn apply_run_operation_checked(document: &RunArtifact, operation: RunMutation) -> protocol::MutationApplyResult<RunArtifact> {
    let outcome = protocol::Mutation::diff(&operation, document);
    if let Some(message) = outcome.messages().iter().find(|message| protocol::MergePolicy::default().rejects(message.level)) {
        return Err(protocol::MutationApplyError { code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone() });
    }
    protocol::MutationDiff::apply(outcome.diff(), document)
}
