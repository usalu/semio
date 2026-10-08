//! 🔺️ Ordered run steps, each an absolute row edit; the central applier replays them against the guards below.
use crate::{RunArtifact, RunLogLine, RunNodeRecord, RunParameterValue, RunStatus, RunTrigger};

/// 🏁️ The absolute run header: the identity and trigger fields plus the lifecycle `status` and `started_at`. A header that
/// leaves `Pending` is a start and is guarded against an already started run; a `Pending` header is an absolute reset.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RunHeaderEdit {
    pub workflow_ref: String,
    pub workflow_checkpoint_id: String,
    pub input_collection_ref: String,
    pub input_snapshot_id: String,
    pub parameter_values: Vec<RunParameterValue>,
    pub output_collection_ref: String,
    pub trigger: RunTrigger,
    pub status: RunStatus,
    pub started_at: String,
}

impl RunHeaderEdit {
    pub fn of(document: &RunArtifact) -> Self {
        Self {
            workflow_ref: document.workflow_ref.clone(),
            workflow_checkpoint_id: document.workflow_checkpoint_id.clone(),
            input_collection_ref: document.input_collection_ref.clone(),
            input_snapshot_id: document.input_snapshot_id.clone(),
            parameter_values: document.parameter_values.clone(),
            output_collection_ref: document.output_collection_ref.clone(),
            trigger: document.trigger.clone(),
            status: document.status,
            started_at: document.started_at.clone(),
        }
    }

    fn differs_from(&self, document: &RunArtifact) -> bool {
        self != &Self::of(document)
    }
}

/// 🔒️ The absolute seal state: whether the run is sealed, its final `status` and its `finished_at`.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct RunSealEdit {
    pub sealed: bool,
    pub status: RunStatus,
    #[value(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
}

impl RunSealEdit {
    pub fn of(document: &RunArtifact) -> Self {
        Self { sealed: document.sealed, status: document.status, finished_at: document.finished_at.clone() }
    }
}

/// 🪜️ One ordered run step: an absolute header, node row or seal edit, or a log tail append or retraction.
#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum RunStep {
    Header(RunHeaderEdit),
    Node {
        node_id: String,
        record: Option<RunNodeRecord>,
    },
    LogAppend(RunLogLine),
    LogRetract {
        count: u64,
    },
    Seal(RunSealEdit),
}

/// 🔺️ Run events in application order; replay order matters because a sealed run accepts only an unseal and a started run
/// rejects a second start.
#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RunDiff {
    #[value(skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<RunStep>,
}

impl RunDiff {
    pub fn step(step: RunStep) -> Self {
        Self { steps: vec![step] }
    }
}

fn sealed_error() -> protocol::MutationApplyError {
    protocol::MutationApplyError::new("mutation.apply.sealed", "run document is sealed").at(["sealed"])
}

fn step_into(next: &mut RunArtifact, step: &RunStep) -> protocol::MutationApplyResult<()> {
    let unseals = matches!(step, RunStep::Seal(seal) if !seal.sealed);
    if next.sealed && !unseals {
        return Err(sealed_error());
    }
    match step {
        RunStep::Header(header) => {
            if header.status != RunStatus::Pending && (next.status != RunStatus::Pending || !next.started_at.is_empty()) {
                return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "run has already started").at(["status"]));
            }
            next.workflow_ref = header.workflow_ref.clone();
            next.workflow_checkpoint_id = header.workflow_checkpoint_id.clone();
            next.input_collection_ref = header.input_collection_ref.clone();
            next.input_snapshot_id = header.input_snapshot_id.clone();
            next.parameter_values = header.parameter_values.clone();
            next.output_collection_ref = header.output_collection_ref.clone();
            next.trigger = header.trigger.clone();
            next.status = header.status;
            next.started_at = header.started_at.clone();
        }
        RunStep::Node { node_id, record } => match (next.node_records.iter().position(|entry| entry.node_id == *node_id), record) {
            (Some(at), Some(record)) => next.node_records[at] = record.clone(),
            (None, Some(record)) => next.node_records.push(record.clone()),
            (Some(at), None) => {
                next.node_records.remove(at);
            }
            (None, None) => return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "run node record does not exist").at(["nodes", node_id.as_str()])),
        },
        RunStep::LogAppend(line) => next.logs.push(line.clone()),
        RunStep::LogRetract { count } => {
            let count = usize::try_from(*count).unwrap_or(usize::MAX);
            if count > next.logs.len() {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "run log has fewer lines than the retraction names").at(["logs"]));
            }
            next.logs.truncate(next.logs.len() - count);
        }
        RunStep::Seal(seal) => {
            next.sealed = seal.sealed;
            next.status = seal.status;
            next.finished_at = seal.finished_at.clone();
        }
    }
    Ok(())
}

impl RunStep {
    fn negation(&self, state: &RunArtifact) -> Vec<RunStep> {
        match self {
            RunStep::Header(_) => vec![RunStep::Header(RunHeaderEdit::of(state))],
            RunStep::Node { node_id, .. } => vec![RunStep::Node { node_id: node_id.clone(), record: state.node_records.iter().find(|entry| entry.node_id == *node_id).cloned() }],
            RunStep::LogAppend(_) => vec![RunStep::LogRetract { count: 1 }],
            RunStep::LogRetract { count } => {
                let kept = state.logs.len().saturating_sub(usize::try_from(*count).unwrap_or(usize::MAX));
                state.logs[kept..].iter().cloned().map(RunStep::LogAppend).collect()
            }
            RunStep::Seal(_) => vec![RunStep::Seal(RunSealEdit::of(state))],
        }
    }
}

impl protocol::MutationDiff<RunArtifact> for RunDiff {
    fn apply(&self, document: &RunArtifact, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<RunArtifact> {
        let mut next = document.clone();
        for step in &self.steps {
            step_into(&mut next, step)?;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.steps.extend(other.steps);
    }
}

impl protocol::DiffAlgebra<RunArtifact> for RunDiff {
    fn inverse(&self, base: &RunArtifact) -> Self {
        let mut state = base.clone();
        let mut negations = Vec::with_capacity(self.steps.len());
        for step in &self.steps {
            negations.push(step.negation(&state));
            if step_into(&mut state, step).is_err() {
                return Self::default();
            }
        }
        Self { steps: negations.into_iter().rev().flatten().collect() }
    }

    fn between(base: &RunArtifact, other: &RunArtifact) -> Self {
        let mut steps = Vec::new();
        if base.sealed {
            steps.push(RunStep::Seal(RunSealEdit { sealed: false, status: base.status, finished_at: base.finished_at.clone() }));
        }
        let header = RunHeaderEdit::of(other);
        if header.differs_from(base) {
            if header.status != RunStatus::Pending {
                steps.push(RunStep::Header(RunHeaderEdit { status: RunStatus::Pending, started_at: String::new(), ..header.clone() }));
            }
            steps.push(RunStep::Header(header));
        }
        let shared = base.node_records.iter().zip(&other.node_records).take_while(|(left, right)| left == right).count();
        steps.extend(base.node_records[shared..].iter().map(|record| RunStep::Node { node_id: record.node_id.clone(), record: None }));
        steps.extend(other.node_records[shared..].iter().map(|record| RunStep::Node { node_id: record.node_id.clone(), record: Some(record.clone()) }));
        let kept = base.logs.iter().zip(&other.logs).take_while(|(left, right)| left == right).count();
        if kept < base.logs.len() {
            steps.push(RunStep::LogRetract { count: (base.logs.len() - kept) as u64 });
        }
        steps.extend(other.logs[kept..].iter().cloned().map(RunStep::LogAppend));
        if other.sealed || base.sealed || base.finished_at != other.finished_at {
            steps.push(RunStep::Seal(RunSealEdit::of(other)));
        }
        Self { steps }
    }

    fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}
