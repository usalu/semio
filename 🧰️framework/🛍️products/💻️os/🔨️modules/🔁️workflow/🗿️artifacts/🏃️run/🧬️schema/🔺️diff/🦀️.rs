//! 🔺️ Run event composition and admission.
use crate::{apply_run_operation, AppendRunLog, FinishRunNode, RunArtifact, RunMutation, RunNodeRecord, RunParameterValue, RunStatus, RunTrigger, SealRun, StartRun, StartRunNode};

#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RunDiff {
    #[default]
    Empty,
    Start {
        workflow_ref: String,
        workflow_checkpoint_id: String,
        input_collection_ref: String,
        input_snapshot_id: String,
        parameter_values: Vec<RunParameterValue>,
        output_collection_ref: String,
        trigger: RunTrigger,
    },
    NodeStarted {
        node_id: String,
    },
    NodeFinished {
        node_record: RunNodeRecord,
    },
    Log {
        node_id: String,
        level: String,
        message: String,
        at: String,
    },
    Seal {
        status: RunStatus,
    },
    Sequence {
        steps: Vec<RunDiff>,
    },
}

impl protocol::MutationDiff<RunArtifact> for RunDiff {
    fn apply(&self, document: &RunArtifact) -> protocol::MutationApplyResult<RunArtifact> {
        if let RunDiff::Sequence { steps } = self {
            let mut current = document.clone();
            for step in steps {
                current = protocol::MutationDiff::apply(step, &current)?;
            }
            return Ok(current);
        }
        if document.sealed && !matches!(self, RunDiff::Empty) {
            return Err(protocol::MutationApplyError::new("mutation.apply.sealed", "run document is sealed").at(["sealed"]));
        }
        if matches!(self, RunDiff::Start { .. }) && (document.status != RunStatus::Pending || !document.started_at.is_empty()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "run has already started").at(["status"]));
        }
        let operation = match self {
            RunDiff::Empty => return Ok(document.clone()),
            RunDiff::Start { workflow_ref, workflow_checkpoint_id, input_collection_ref, input_snapshot_id, parameter_values, output_collection_ref, trigger } => RunMutation::StartRun(StartRun {
                workflow_ref: workflow_ref.clone(),
                workflow_checkpoint_id: workflow_checkpoint_id.clone(),
                input_collection_ref: input_collection_ref.clone(),
                input_snapshot_id: input_snapshot_id.clone(),
                parameter_values: parameter_values.clone(),
                output_collection_ref: output_collection_ref.clone(),
                trigger: trigger.clone(),
            }),
            RunDiff::NodeStarted { node_id } => RunMutation::StartRunNode(StartRunNode { node_id: node_id.clone() }),
            RunDiff::NodeFinished { node_record } => RunMutation::FinishRunNode(FinishRunNode { node_record: node_record.clone() }),
            RunDiff::Log { node_id, level, message, at } => RunMutation::AppendRunLog(AppendRunLog { node_id: node_id.clone(), level: level.clone(), message: message.clone(), at: at.clone() }),
            RunDiff::Seal { status } => RunMutation::SealRun(SealRun { status: *status }),
            RunDiff::Sequence { .. } => unreachable!("RunDiff sequence applies above"),
        };
        Ok(apply_run_operation(document, &operation))
    }

    fn absorb(&mut self, other: Self) {
        fn append(steps: &mut Vec<RunDiff>, diff: RunDiff) {
            match diff {
                RunDiff::Empty => {}
                RunDiff::Sequence { steps: nested } => {
                    for step in nested {
                        append(steps, step);
                    }
                }
                step => steps.push(step),
            }
        }

        let mut steps = Vec::new();
        append(&mut steps, std::mem::take(self));
        append(&mut steps, other);
        *self = match steps.len() {
            0 => RunDiff::Empty,
            1 => steps.pop().expect("single run diff step"),
            _ => RunDiff::Sequence { steps },
        };
    }
}
