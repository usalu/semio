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

/// ↩️ The prior value of every slot a run step edits, read row by row: a slot not yet edited by an earlier step of the same
/// diff is read from `base`, an edited one from that step's own row. Nothing is applied and no document is copied.
struct RunRows<'a> {
    base: &'a RunArtifact,
    header: Option<&'a RunHeaderEdit>,
    status: RunStatus,
    started_at: &'a str,
    sealed: bool,
    finished_at: Option<&'a str>,
    nodes: std::collections::BTreeMap<&'a str, Option<&'a RunNodeRecord>>,
    appended: Vec<&'a RunLogLine>,
    kept: usize,
}

impl<'a> RunRows<'a> {
    fn new(base: &'a RunArtifact) -> Self {
        Self { base, header: None, status: base.status, started_at: &base.started_at, sealed: base.sealed, finished_at: base.finished_at.as_deref(), nodes: std::collections::BTreeMap::new(), appended: Vec::new(), kept: base.logs.len() }
    }

    fn prior_rows(&mut self, step: &'a RunStep) -> Vec<RunStep> {
        match step {
            RunStep::Header(header) => {
                let core = self.header.cloned().unwrap_or_else(|| RunHeaderEdit::of(self.base));
                let negation = RunHeaderEdit { status: self.status, started_at: self.started_at.to_owned(), ..core };
                self.header = Some(header);
                self.status = header.status;
                self.started_at = &header.started_at;
                vec![RunStep::Header(negation)]
            }
            RunStep::Node { node_id, record } => {
                let prior = self.nodes.get(node_id.as_str()).copied().unwrap_or_else(|| self.base.node_records.iter().find(|entry| entry.node_id == *node_id));
                self.nodes.insert(node_id.as_str(), record.as_ref());
                vec![RunStep::Node { node_id: node_id.clone(), record: prior.cloned() }]
            }
            RunStep::LogAppend(line) => {
                self.appended.push(line);
                vec![RunStep::LogRetract { count: 1 }]
            }
            RunStep::LogRetract { count } => {
                let count = usize::try_from(*count).unwrap_or(usize::MAX);
                let length = self.kept + self.appended.len();
                let negation = (length.saturating_sub(count)..length).map(|at| RunStep::LogAppend(if at < self.kept { self.base.logs[at].clone() } else { self.appended[at - self.kept].clone() })).collect();
                let from_appended = count.min(self.appended.len());
                self.appended.truncate(self.appended.len() - from_appended);
                self.kept = self.kept.saturating_sub(count - from_appended);
                negation
            }
            RunStep::Seal(seal) => {
                let negation = RunSealEdit { sealed: self.sealed, status: self.status, finished_at: self.finished_at.map(str::to_owned) };
                self.sealed = seal.sealed;
                self.status = seal.status;
                self.finished_at = seal.finished_at.as_deref();
                vec![RunStep::Seal(negation)]
            }
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
        let mut rows = RunRows::new(base);
        let undo: Vec<Vec<RunStep>> = self.steps.iter().map(|step| rows.prior_rows(step)).collect();
        Self { steps: undo.into_iter().rev().flatten().collect() }
    }

    fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}
