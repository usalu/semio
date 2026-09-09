//! 🌳️ Retained admission of one complete recursively owned document closure.
use super::{ChildRestoreProjection, ChildRestoreProjectionError, OwnerRef, CHILD_RESTORE_MAXIMUM_FIELD_BYTES};
use crate::os_io::ArtifactRef;
use semio_framework_job::{Generation, OperationId, StepContext};
use std::hash::{Hash, Hasher};

pub const OWNED_DOCUMENT_MAXIMUM_MEMBERS: usize = 1024;
const TABLE_SIZE: usize = OWNED_DOCUMENT_MAXIMUM_MEMBERS * 2;
const EMPTY: usize = usize::MAX;

/// 🧬️ Immutable candidate authority; projections come from decoded typed member snapshots.
pub trait OwnedDocumentClosureSource {
    fn generation(&self) -> u64;
    fn root_reference(&self) -> &ArtifactRef;
    fn member_count(&self) -> usize;
    fn member_reference(&self, index: usize) -> Option<&ArtifactRef>;
    fn member_owner(&self, index: usize) -> Option<&OwnerRef>;
    fn child_projection(&self, parent: Option<usize>) -> Result<ChildRestoreProjection<'_>, ChildRestoreProjectionError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedDocumentClosureDiagnostic {
    Stale,
    Cancelled,
    Expired,
    MemberLimit,
    InvalidReference,
    InvalidOwner,
    InvalidProjection,
    DuplicateMember,
    DuplicateEdge,
    Incomplete,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OwnedDocumentClosureProgress {
    pub indexed: usize,
    pub visited: usize,
    pub steps: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedDocumentClosureStep {
    Pending(OwnedDocumentClosureProgress),
    Complete { members: usize },
    Rejected(OwnedDocumentClosureDiagnostic),
}

#[derive(Clone, Copy)]
enum Phase { Root, Index, Walk }

/// 🗂️ Fixed metadata only; candidate envelopes, projections and retirements remain caller-owned.
pub struct OwnedDocumentClosure {
    operation: OperationId,
    generation: Generation,
    source_generation: u64,
    expires_at_us: u64,
    expected_members: Option<usize>,
    phase: Phase,
    table: [usize; TABLE_SIZE],
    queue: [usize; OWNED_DOCUMENT_MAXIMUM_MEMBERS + 1],
    seen: [bool; OWNED_DOCUMENT_MAXIMUM_MEMBERS],
    head: usize,
    tail: usize,
    child: usize,
    probe: Option<usize>,
    progress: OwnedDocumentClosureProgress,
    terminal: Option<OwnedDocumentClosureStep>,
}

impl OwnedDocumentClosure {
    /// 🌱️ Reserves only fixed scalar metadata; no candidate payload or borrowed read is retained.
    pub fn new(operation: OperationId, generation: Generation, source_generation: u64, expires_at_us: u64) -> Self {
        Self {
            operation, generation, source_generation, expires_at_us, expected_members: None, phase: Phase::Root,
            table: [EMPTY; TABLE_SIZE], queue: [EMPTY; OWNED_DOCUMENT_MAXIMUM_MEMBERS + 1],
            seen: [false; OWNED_DOCUMENT_MAXIMUM_MEMBERS], head: 0, tail: 1, child: 0, probe: None,
            progress: OwnedDocumentClosureProgress::default(), terminal: None,
        }
    }

    /// 📊️ Counts completed metadata work independently of caller grant sizes.
    pub fn progress(&self) -> OwnedDocumentClosureProgress { self.progress }

    /// 🚦️ Each fuel item handles one bounded identity, hash-table probe or child projection step.
    pub fn step<S: OwnedDocumentClosureSource>(&mut self, source: &S, cx: &mut StepContext<'_>) -> OwnedDocumentClosureStep {
        if let Err(diagnostic) = self.authority(source, cx) { return self.reject(diagnostic); }
        if let Some(terminal) = self.terminal { return terminal; }
        cx.set_stage("document-closure");
        while !cx.should_yield() {
            if let Err(diagnostic) = self.authority(source, cx) { return self.reject(diagnostic); }
            cx.consume_fuel(1);
            self.progress.steps += 1;
            let result = match self.phase {
                Phase::Root => self.root(source),
                Phase::Index => self.index(source),
                Phase::Walk => self.walk(source),
            };
            if let Err(diagnostic) = result { return self.reject(diagnostic); }
            if let Err(diagnostic) = self.authority(source, cx) { return self.reject(diagnostic); }
            if let Some(terminal) = self.terminal { return terminal; }
        }
        OwnedDocumentClosureStep::Pending(self.progress)
    }

    fn authority<S: OwnedDocumentClosureSource>(&self, source: &S, cx: &StepContext<'_>) -> Result<(), OwnedDocumentClosureDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation || source.generation() != self.source_generation
            || self.expected_members.is_some_and(|count| count != source.member_count()) { return Err(OwnedDocumentClosureDiagnostic::Stale); }
        if cx.is_cancelled() { return Err(OwnedDocumentClosureDiagnostic::Cancelled); }
        if cx.now_us().is_none_or(|now| now >= self.expires_at_us) { return Err(OwnedDocumentClosureDiagnostic::Expired); }
        if source.member_count() > OWNED_DOCUMENT_MAXIMUM_MEMBERS { return Err(OwnedDocumentClosureDiagnostic::MemberLimit); }
        Ok(())
    }

    fn root<S: OwnedDocumentClosureSource>(&mut self, source: &S) -> Result<(), OwnedDocumentClosureDiagnostic> {
        if !valid_reference(source.root_reference()) { return Err(OwnedDocumentClosureDiagnostic::InvalidReference); }
        self.expected_members = Some(source.member_count());
        self.phase = Phase::Index;
        Ok(())
    }

    fn index<S: OwnedDocumentClosureSource>(&mut self, source: &S) -> Result<(), OwnedDocumentClosureDiagnostic> {
        let index = self.progress.indexed;
        if index == source.member_count() { self.phase = Phase::Walk; return Ok(()); }
        let reference = source.member_reference(index).ok_or(OwnedDocumentClosureDiagnostic::InvalidReference)?;
        if self.probe.is_none() {
            let owner = source.member_owner(index).ok_or(OwnedDocumentClosureDiagnostic::InvalidOwner)?;
            if !valid_reference(reference) { return Err(OwnedDocumentClosureDiagnostic::InvalidReference); }
            if !valid_reference(&owner.parent) || !valid_text(&owner.slot) || owner.child_id != reference.artifact_id { return Err(OwnedDocumentClosureDiagnostic::InvalidOwner); }
            if reference.artifact_id == source.root_reference().artifact_id { return Err(OwnedDocumentClosureDiagnostic::DuplicateMember); }
            self.probe = Some(bucket(&reference.artifact_id));
        }
        let probe = self.probe.expect("active index probe");
        let previous = self.table[probe];
        if previous == EMPTY {
            self.table[probe] = index;
            self.progress.indexed += 1;
            self.probe = None;
        } else {
            let prior = source.member_reference(previous).ok_or(OwnedDocumentClosureDiagnostic::InvalidReference)?;
            if prior.artifact_id == reference.artifact_id { return Err(OwnedDocumentClosureDiagnostic::DuplicateMember); }
            self.probe = Some((probe + 1) % TABLE_SIZE);
        }
        Ok(())
    }

    fn walk<S: OwnedDocumentClosureSource>(&mut self, source: &S) -> Result<(), OwnedDocumentClosureDiagnostic> {
        if self.head == self.tail {
            if self.tail - 1 != source.member_count() { return Err(OwnedDocumentClosureDiagnostic::Incomplete); }
            self.terminal = Some(OwnedDocumentClosureStep::Complete { members: source.member_count() });
            return Ok(());
        }
        let parent_index = self.queue[self.head];
        let parent = if parent_index == EMPTY { source.root_reference() } else { source.member_reference(parent_index).ok_or(OwnedDocumentClosureDiagnostic::InvalidReference)? };
        let projection = source.child_projection((parent_index != EMPTY).then_some(parent_index)).map_err(|_| OwnedDocumentClosureDiagnostic::InvalidProjection)?;
        let Some((slot, fields)) = projection.get(self.child) else {
            self.head += 1;
            self.child = 0;
            self.probe = None;
            self.progress.visited += 1;
            return Ok(());
        };
        let probe = *self.probe.get_or_insert_with(|| bucket(fields.child_id));
        let index = self.table[probe];
        if index == EMPTY { return Err(OwnedDocumentClosureDiagnostic::Incomplete); }
        let member = source.member_reference(index).ok_or(OwnedDocumentClosureDiagnostic::InvalidReference)?;
        if member.artifact_id != fields.child_id { self.probe = Some((probe + 1) % TABLE_SIZE); return Ok(()); }
        let owner = source.member_owner(index).ok_or(OwnedDocumentClosureDiagnostic::InvalidOwner)?;
        if member.artifact_id != fields.artifact_id || member.dialect.artifact_kind != fields.artifact_kind || member.dialect.standard != fields.standard || member.dialect.subset != fields.subset
            || owner.parent != *parent || owner.slot != slot || owner.child_id != fields.child_id { return Err(OwnedDocumentClosureDiagnostic::InvalidOwner); }
        if self.seen[index] { return Err(OwnedDocumentClosureDiagnostic::DuplicateEdge); }
        self.seen[index] = true;
        self.queue[self.tail] = index;
        self.tail += 1;
        self.child += 1;
        self.probe = None;
        Ok(())
    }

    fn reject(&mut self, diagnostic: OwnedDocumentClosureDiagnostic) -> OwnedDocumentClosureStep {
        let terminal = OwnedDocumentClosureStep::Rejected(diagnostic);
        self.terminal = Some(terminal);
        terminal
    }
}

fn valid_text(value: &str) -> bool {
    !value.is_empty() && value.len() <= CHILD_RESTORE_MAXIMUM_FIELD_BYTES && !value.chars().any(char::is_control)
}
fn valid_reference(value: &ArtifactRef) -> bool {
    [&value.artifact_id, &value.dialect.artifact_kind, &value.dialect.standard, &value.dialect.subset].into_iter().all(|field| valid_text(field))
        && crate::os_io::is_canonical_artifact_kind(&value.dialect.artifact_kind)
}
fn bucket(id: &str) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish() as usize % TABLE_SIZE
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
