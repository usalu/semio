//! 📂️ Caller-retained member-open authority: bounded input ownership, scoped progress and close.

#[path = "📜️history/🦀️.rs"]
pub(crate) mod history;
#[path = "🏭️operation/🦀️.rs"]
mod operation;
pub use history::factory::MemberOpenDeclaration;
pub use operation::{InitialMemberStoreOpen, MemberSnapshotOpenOperation, MemberSnapshotOpenStep, UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};

use super::{ErasedSnapshotRetirement, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OwnerRef, SnapshotRetirementStep, SpaceMember};
use crate::os_io::ArtifactRef;
use semio_framework_job::{Generation, OperationId, StepContext};
use std::mem::ManuallyDrop;

pub const MEMBER_OPEN_IDENTITY_BYTES: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberOpenDiagnostic {
    Unsealed,
    Empty,
    Expired,
    Identity,
    Owner,
    Capacity,
    Stale,
    Cancelled,
    Malformed,
    Decode,
    Replay,
    Initialization,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberOpenPhase {
    Input,
    Header,
    Snapshot,
    History,
    Validate,
    Replay,
    Initialize,
    Publish,
    Retire,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemberOpenProgress {
    pub phase: MemberOpenPhase,
    pub completed: u64,
    pub total: u64,
}

pub enum MemberOpenStep<M> {
    Pending(MemberOpenProgress),
    Ready(M),
    Rejected(MemberOpenDiagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemberOpenFrame {
    snapshot_start: usize,
    snapshot_bytes: usize,
    history_start: usize,
    history_bytes: usize,
}

impl MemberOpenFrame {
    pub fn snapshot_range(self) -> (usize, usize) {
        (self.snapshot_start, self.snapshot_bytes)
    }
    pub fn history_range(self) -> (usize, usize) {
        (self.history_start, self.history_bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberOpenInputStep {
    Pending(MemberOpenProgress),
    Framed(MemberOpenFrame),
    Rejected(MemberOpenDiagnostic),
}

pub trait MemberOpenOperation {
    type Member;
    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<Self::Member>;
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct MemberOpenRequest {
    operation: OperationId,
    generation: Generation,
    expires_at_us: u64,
    expected: ManuallyDrop<Option<ArtifactRef>>,
    owner: ManuallyDrop<Option<OwnerRef>>,
    pages: ManuallyDrop<Option<OwnedSchemaDecodePages>>,
    closing_page: Option<OwnedSchemaDecodePage>,
    closing_bytes: usize,
    closing_identity: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    input_offset: usize,
    snapshot_bytes: u64,
    framed: Option<MemberOpenFrame>,
    rejected: Option<MemberOpenDiagnostic>,
    admitted: bool,
    closing: bool,
    detached: bool,
}

impl MemberOpenRequest {
    pub fn new(operation: OperationId, generation: Generation, expires_at_us: u64, expected: ArtifactRef, owner: Option<OwnerRef>, pages: OwnedSchemaDecodePages) -> Self {
        Self {
            operation,
            generation,
            expires_at_us,
            expected: ManuallyDrop::new(Some(expected)),
            owner: ManuallyDrop::new(owner),
            pages: ManuallyDrop::new(Some(pages)),
            closing_page: None,
            closing_bytes: 0,
            closing_identity: ManuallyDrop::new(None),
            input_offset: 0,
            snapshot_bytes: 0,
            framed: None,
            rejected: None,
            admitted: false,
            closing: false,
            detached: false,
        }
    }

    pub fn expected(&self) -> &ArtifactRef {
        self.expected.as_ref().expect("open request identity remains retained")
    }
    pub fn admitted_expected(&self) -> Result<&ArtifactRef, MemberOpenDiagnostic> {
        if self.closing || self.detached {
            return Err(MemberOpenDiagnostic::Stale);
        }
        if !self.admitted {
            return Err(MemberOpenDiagnostic::Unsealed);
        }
        self.expected.as_ref().ok_or(MemberOpenDiagnostic::Stale)
    }
    pub fn owner(&self) -> Option<&OwnerRef> {
        self.owner.as_ref()
    }
    pub fn operation(&self) -> OperationId {
        self.operation
    }
    pub fn generation(&self) -> Generation {
        self.generation
    }
    pub fn retained_input_bytes(&self) -> usize {
        self.pages.as_ref().map_or(0, OwnedSchemaDecodePages::byte_count) + self.closing_bytes
    }

    pub fn admit(mut self, now_us: u64) -> Result<Self, MemberOpenAdmissionError> {
        let text = |value: &str| !value.is_empty() && value.len() <= MEMBER_OPEN_IDENTITY_BYTES && !value.chars().any(char::is_control);
        let reference = |value: &ArtifactRef| [value.artifact_id.as_str(), value.dialect.artifact_kind.as_str(), value.dialect.standard.as_str(), value.dialect.subset.as_str()].into_iter().all(text);
        let diagnostic = if self.pages.as_ref().is_none_or(|pages| !pages.is_sealed()) {
            Some(MemberOpenDiagnostic::Unsealed)
        } else if self.retained_input_bytes() == 0 {
            Some(MemberOpenDiagnostic::Empty)
        } else if now_us >= self.expires_at_us {
            Some(MemberOpenDiagnostic::Expired)
        } else if !reference(self.expected()) {
            Some(MemberOpenDiagnostic::Identity)
        } else if self.owner().is_some_and(|owner| owner.child_id != self.expected().artifact_id || !reference(&owner.parent) || !text(&owner.slot) || !text(&owner.child_id)) {
            Some(MemberOpenDiagnostic::Owner)
        } else {
            None
        };
        match diagnostic {
            Some(diagnostic) => Err(MemberOpenAdmissionError { diagnostic, request: self }),
            None => {
                self.admitted = true;
                Ok(self)
            }
        }
    }

    pub fn check_step_authority(&self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.admitted_expected()?;
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(MemberOpenDiagnostic::Stale);
        }
        if cx.is_cancelled() {
            return Err(MemberOpenDiagnostic::Cancelled);
        }
        if cx.now_us().is_none_or(|now| now >= self.expires_at_us) {
            return Err(MemberOpenDiagnostic::Expired);
        }
        Ok(())
    }

    pub fn step_input(&mut self, cx: &mut StepContext<'_>) -> MemberOpenInputStep {
        if let Some(diagnostic) = self.rejected {
            return MemberOpenInputStep::Rejected(diagnostic);
        }
        if let Err(diagnostic) = self.check_step_authority(cx) {
            self.rejected = Some(diagnostic);
            return MemberOpenInputStep::Rejected(diagnostic);
        }
        if let Some(frame) = self.framed {
            return MemberOpenInputStep::Framed(frame);
        }
        cx.set_stage("member-open.input");
        while !cx.should_yield() {
            if let Err(diagnostic) = self.check_step_authority(cx) {
                self.rejected = Some(diagnostic);
                return MemberOpenInputStep::Rejected(diagnostic);
            }
            let byte = self.pages.as_ref().and_then(|pages| pages.byte_at(self.input_offset));
            let valid = byte.is_some_and(|byte| self.input_offset < 10 && (self.input_offset < 9 || byte <= 1));
            if !valid {
                self.rejected = Some(MemberOpenDiagnostic::Malformed);
                return MemberOpenInputStep::Rejected(MemberOpenDiagnostic::Malformed);
            }
            let byte = byte.expect("bounded frame byte was checked");
            self.snapshot_bytes |= u64::from(byte & 127) << (self.input_offset * 7);
            self.input_offset += 1;
            cx.consume_fuel(1);
            if let Err(diagnostic) = self.check_step_authority(cx) {
                self.rejected = Some(diagnostic);
                return MemberOpenInputStep::Rejected(diagnostic);
            }
            if byte & 128 == 0 {
                let length = usize::try_from(self.snapshot_bytes).ok();
                let end = length.and_then(|length| self.input_offset.checked_add(length));
                let total = self.retained_input_bytes();
                if self.snapshot_bytes == 0 || (self.input_offset > 1 && byte == 0) || end.is_none_or(|end| end >= total) {
                    self.rejected = Some(MemberOpenDiagnostic::Malformed);
                    return MemberOpenInputStep::Rejected(MemberOpenDiagnostic::Malformed);
                }
                let end = end.expect("complete bounded frame end");
                let frame = MemberOpenFrame { snapshot_start: self.input_offset, snapshot_bytes: end - self.input_offset, history_start: end, history_bytes: total - end };
                self.framed = Some(frame);
                return MemberOpenInputStep::Framed(frame);
            }
        }
        MemberOpenInputStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Input, completed: self.input_offset as u64, total: self.retained_input_bytes() as u64 })
    }

    pub fn copy_snapshot_chunk(&self, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        self.copy_input_chunk(true, offset, output, cx)
    }

    pub fn copy_history_chunk(&self, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        self.copy_input_chunk(false, offset, output, cx)
    }

    fn copy_input_chunk(&self, snapshot: bool, offset: usize, output: &mut [u8], cx: &mut StepContext<'_>) -> Result<usize, MemberOpenDiagnostic> {
        self.check_step_authority(cx)?;
        if let Some(diagnostic) = self.rejected {
            return Err(diagnostic);
        }
        let frame = self.framed.ok_or(MemberOpenDiagnostic::Malformed)?;
        let (start, length) = if snapshot { frame.snapshot_range() } else { frame.history_range() };
        if offset > length {
            return Err(MemberOpenDiagnostic::Malformed);
        }
        let maximum = output.len().min(length - offset).min(super::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        let mut copied = 0;
        while copied < maximum && !cx.should_yield() {
            self.check_step_authority(cx)?;
            output[copied] = self.pages.as_ref().and_then(|pages| pages.byte_at(start + offset + copied)).ok_or(MemberOpenDiagnostic::Malformed)?;
            copied += 1;
            cx.consume_fuel(1);
        }
        Ok(copied)
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.detached {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.closing = true;
        if self.closing_page.is_some() {
            if self.closing_bytes != 0 {
                let bytes = maximum_bytes.min(self.closing_bytes);
                self.closing_bytes -= bytes;
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: bytes });
            }
            self.closing_page.take();
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(pages) = self.pages.as_mut() {
            if let Some(page) = pages.close_take_page() {
                self.closing_bytes = page.len();
                self.closing_page = Some(page);
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            assert!(pages.terminal_is_empty(), "input page registry must be terminal before release");
            self.pages.take();
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(identity) = self.closing_identity.as_mut() {
            return match identity.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if identity.terminal_is_empty() => {
                    self.closing_identity.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member request identity returned false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("member request identity exceeded its close grant".into()),
                step => Ok(step),
            };
        }
        if let Some(expected) = self.expected.take() {
            *self.closing_identity = Some(super::retirement::owned_retirement((expected, self.owner.take())));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        assert!(self.owner.is_none());
        self.detached = true;
        Ok(SnapshotRetirementStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.detached && self.expected.is_none() && self.owner.is_none() && self.pages.is_none() && self.closing_page.is_none() && self.closing_identity.is_none()
    }
}

impl ErasedSnapshotRetirement for MemberOpenRequest {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        MemberOpenRequest::close_step(self, maximum_items, maximum_bytes)
    }
    fn terminal_is_empty(&self) -> bool {
        MemberOpenRequest::terminal_is_empty(self)
    }
}

impl Drop for MemberOpenRequest {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "member-open input reached Drop before adoption or bounded retirement");
    }
}

pub struct MemberOpenAdmissionError {
    pub diagnostic: MemberOpenDiagnostic,
    pub request: MemberOpenRequest,
}

crate::artifact_retire_struct!(crate::os_spr::HistoryLog { doc_id, schema, edits, changes, checkpoints, alternatives, active_alternative_id, cursor, composition, conflicts });
crate::artifact_retire_struct!(crate::os_spr::HistoryComposition { owner, dialect, checkpoint_pins });
crate::artifact_retire_struct!(crate::os_spr::history::HistoryConflict { id, kind, status, actors, hlt, edit_ids, envelopes, messages });
crate::artifact_retire_struct!(crate::os_spr::history::HistoryMessage { level, code, message, target, op_index });
crate::artifact_retire_struct!(crate::os_spr::HistoryEdit { id, actor, started_at, finished_at, coalesce_key, description, ops, inverse, meta });
crate::artifact_retire_struct!(crate::os_spr::OpPayload { text, binary });
crate::artifact_retire_struct!(crate::os_spr::HistoryCursor { applied_edit_ids, redo_edit_ids, checkpoint_id });
crate::artifact_retire_struct!(crate::os_spr::HistoryOpMeta { op_id, dependencies, base_version, author_id, hlt, undo_policy, payload_hash, group_id, origin, messages });
crate::artifact_retire_struct!(crate::os_spr::HistoryChange { id, saved_at, edit_ids, description });
crate::artifact_retire_struct!(crate::os_spr::HistoryCheckpoint { id, timestamp, change_ids, parent_id, authors, message });
crate::artifact_retire_struct!(crate::os_spr::HistoryAuthor { id, name });
crate::artifact_retire_struct!(crate::os_spr::HistoryAlternative { id, name, checkpoint_ids });

impl super::retirement::RetireOwned for crate::os_spr::MutationOrigin {
    fn retirement(self) -> Box<dyn super::retirement::RetirementCursor> {
        match self {
            Self::Owner => super::retirement::sequence(Vec::new()),
            Self::Contributed { plugin_id, mutation_id, payload_hash } => crate::artifact_retirement_sequence![plugin_id, mutation_id.0, payload_hash.0],
            Self::Transaction { initiator } => crate::artifact_retirement_sequence![initiator.artifact_id, initiator.artifact_kind, initiator.dialect],
        }
    }
}

pub(super) struct MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P>,
{
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    owners: ManuallyDrop<Option<super::MemberStoreOwners<P, M>>>,
    history: ManuallyDrop<Option<crate::os_spr::HistoryLog>>,
    initial: ManuallyDrop<Option<P>>,
    pending_edit: ManuallyDrop<Option<super::Edit<M>>>,
    envelope: ManuallyDrop<Option<super::ArtifactEnvelopeOwners<P, M>>>,
    runtime: ManuallyDrop<Option<super::ArtifactStoreInitializationRuntime<P>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    diagnostic: Option<MemberOpenDiagnostic>,
    terminal: bool,
}

impl<P, M> MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue + Send + 'static,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    pub(super) fn new(request: MemberOpenRequest, owners: super::MemberStoreOwners<P, M>) -> Self {
        Self {
            request: ManuallyDrop::new(Some(request)),
            owners: ManuallyDrop::new(Some(owners)),
            history: ManuallyDrop::new(None),
            initial: ManuallyDrop::new(None),
            pending_edit: ManuallyDrop::new(None),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            diagnostic: None,
            terminal: false,
        }
    }

    pub(super) fn stage_history(&mut self, history: crate::os_spr::HistoryLog) -> Result<(), crate::os_spr::HistoryLog> {
        if self.diagnostic.is_some() || self.terminal || self.history.is_some() || self.initial.is_some() || self.envelope.is_some() {
            return Err(history);
        }
        *self.history = Some(history);
        Ok(())
    }

    pub(super) fn stage_initial(&mut self, initial: P) -> Result<(), P> {
        if self.diagnostic.is_some() || self.terminal || self.initial.is_some() || self.envelope.is_some() {
            return Err(initial);
        }
        *self.initial = Some(initial);
        Ok(())
    }

    pub(super) fn stage_edit(&mut self, edit: super::Edit<M>) -> Result<(), super::Edit<M>> {
        if self.diagnostic.is_some() || self.terminal || self.pending_edit.is_some() {
            return Err(edit);
        }
        *self.pending_edit = Some(edit);
        Ok(())
    }

    pub(super) fn stage_envelope(&mut self, envelope: super::ArtifactEnvelopeOwners<P, M>) -> Result<(), super::ArtifactEnvelopeOwners<P, M>> {
        if self.diagnostic.is_some() || self.terminal || self.envelope.is_some() || self.initial.is_some() {
            return Err(envelope);
        }
        *self.envelope = Some(envelope);
        Ok(())
    }

    pub(super) fn stage_runtime(&mut self, runtime: super::ArtifactStoreInitializationRuntime<P>) -> Result<(), super::ArtifactStoreInitializationRuntime<P>> {
        if self.diagnostic.is_some() || self.terminal || self.runtime.is_some() || self.envelope.is_none() {
            return Err(runtime);
        }
        *self.runtime = Some(runtime);
        Ok(())
    }

    pub(super) fn check_step_authority(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if let Some(diagnostic) = self.diagnostic {
            return Err(diagnostic);
        }
        let result = self.request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx);
        if let Err(diagnostic) = result {
            self.reject(diagnostic);
        }
        result
    }

    pub(super) fn reject(&mut self, diagnostic: MemberOpenDiagnostic) {
        self.diagnostic.get_or_insert(diagnostic);
    }

    pub(super) fn retained_input_bytes(&self) -> usize {
        self.request.as_ref().map_or(0, MemberOpenRequest::retained_input_bytes)
    }

    pub(super) fn retained_typed_owners(&self) -> (bool, bool, bool, bool) {
        (self.initial.is_some(), self.pending_edit.is_some(), self.envelope.is_some(), self.runtime.is_some())
    }
}

impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue + Send + 'static,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.reject(MemberOpenDiagnostic::Cancelled);
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    self.active.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open nested owner reported false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("member-open nested owner exceeded its close grant".into()),
                step => Ok(step),
            };
        }
        let owners = self.owners.as_ref().expect("member-open retirement retains the original owner bundle");
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), 1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    self.runtime.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open initialization reported false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("member-open initialization exceeded its close grant".into()),
                step => Ok(step),
            };
        }
        if let Some(edit) = self.pending_edit.take() {
            *self.active = Some(Box::new(super::ArtifactStoreDecodedEditRetirement::new(edit, owners.mutation_retirement.clone())));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(envelope) = self.envelope.take() {
            *self.active = Some(Box::new(super::ArtifactStoreEnvelopeRetirement::new(super::ArtifactEnvelope::from_owners(envelope), owners.initial_snapshot_retirement.clone(), owners.mutation_retirement.clone())));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(initial) = self.initial.take() {
            *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(history) = self.history.take() {
            *self.active = Some(super::retirement::owned_retirement(history));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(request) = self.request.as_mut() {
            return match request.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
                    self.request.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open request reported false terminal".into()),
                step => Ok(step),
            };
        }
        let disposer = &mut self.owners.as_mut().expect("original owner bundle remains until rejection is terminal").store_disposer;
        match disposer.close_uninstalled_step(1)? {
            SnapshotRetirementStep::Complete if disposer.uninstalled_terminal_is_empty() => {}
            SnapshotRetirementStep::Complete => return Err("member-open uninstalled disposer reported false terminal".into()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes != 0 => return Err("member-open uninstalled disposer exceeded its close grant".into()),
            step => return Ok(step),
        }
        self.owners.take();
        self.terminal = true;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.owners.is_none() && self.history.is_none() && self.initial.is_none() && self.pending_edit.is_none() && self.envelope.is_none() && self.runtime.is_none() && self.active.is_none()
    }
}

impl<P, M> Drop for MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P>,
{
    fn drop(&mut self) {
        assert!(
            self.terminal && self.request.is_none() && self.owners.is_none() && self.history.is_none() && self.initial.is_none() && self.pending_edit.is_none() && self.envelope.is_none() && self.runtime.is_none() && self.active.is_none(),
            "member-open reached Drop before exact adoption or bounded rejection retirement"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_store::{OwnedSchemaDecodeCredits, OWNED_SCHEMA_DECODE_PAGE_BYTES};

    fn request_for(bytes: &[u8]) -> MemberOpenRequest {
        let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.len().max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.len().max(1) }).unwrap();
        for chunk in bytes.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
            pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
        }
        pages.seal().unwrap();
        let expected = ArtifactRef { artifact_id: "member".into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
        MemberOpenRequest::new(OperationId(1), Generation(1), 1000, expected, None, pages).admit(1).unwrap_or_else(|_| panic!("admissible test input"))
    }

    fn retire_request(request: &mut MemberOpenRequest) {
        for _ in 0..100_000 {
            match request.close_step(1, 7).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 7),
                SnapshotRetirementStep::Complete => {
                    assert!(request.terminal_is_empty());
                    return;
                }
                SnapshotRetirementStep::Blocked => panic!("inline pages have no shared owner"),
            }
        }
        panic!("request retirement did not converge");
    }

    #[test]
    fn member_open_input_framing_is_canonical_scoped_and_budgeted() {
        use semio_framework_job::{root_cancel_token, StepBudget};
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap();
        for row in fixture["framing"].as_array().unwrap() {
            let bytes: Vec<u8> = serde_json::from_value(row["bytes"].clone()).unwrap();
            for fuel in [1, 2, 13] {
                let mut request = request_for(&bytes);
                let mut sequence = 0;
                let cancel = root_cancel_token();
                let mut zero = StepContext::new(OperationId(1), Generation(1), StepBudget::new(0, 999), cancel.clone(), || Some(1), &mut sequence);
                assert!(matches!(request.step_input(&mut zero), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
                let mut outcome = None;
                for _ in 0..16 {
                    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(fuel, 999), cancel.clone(), || Some(1), &mut sequence);
                    let before = request.input_offset;
                    let step = request.step_input(&mut cx);
                    assert!(request.input_offset - before <= fuel as usize);
                    if !matches!(step, MemberOpenInputStep::Pending(_)) {
                        outcome = Some(step);
                        break;
                    }
                }
                match outcome.expect("bounded varint is terminal") {
                    MemberOpenInputStep::Framed(frame) => {
                        assert!(row["reason"].is_null(), "{}", row["id"]);
                        assert_eq!(serde_json::json!(frame.snapshot_range()), row["snapshot"]);
                        assert_eq!(serde_json::json!(frame.history_range()), row["history"]);
                        let mut output = [0xcc; 8];
                        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(1, 999), cancel.clone(), || Some(1), &mut sequence);
                        assert_eq!(request.copy_snapshot_chunk(0, &mut output, &mut cx).unwrap(), 1);
                        assert_eq!(output[0], bytes[frame.snapshot_start]);
                        assert_eq!(&output[1..], &[0xcc; 7]);
                        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), cancel.clone(), || Some(1), &mut sequence);
                        let count = request.copy_history_chunk(0, &mut output, &mut cx).unwrap();
                        assert_eq!(&output[..count], &bytes[frame.history_start..frame.history_start + count]);
                    }
                    MemberOpenInputStep::Rejected(reason) => {
                        assert_eq!(reason, MemberOpenDiagnostic::Malformed);
                        assert_eq!(row["reason"], "malformed");
                    }
                    MemberOpenInputStep::Pending(_) => unreachable!(),
                }
                assert_eq!(request.retained_input_bytes(), bytes.len());
                retire_request(&mut request);
            }
        }
        for (operation, generation, now, cancelled, expected) in
            [(2, 1, 1, false, MemberOpenDiagnostic::Stale), (1, 2, 1, false, MemberOpenDiagnostic::Stale), (1, 1, 1000, false, MemberOpenDiagnostic::Expired), (1, 1, 1, true, MemberOpenDiagnostic::Cancelled)]
        {
            let mut request = request_for(&[1, 97, 83]);
            let mut sequence = 0;
            let cancel = root_cancel_token();
            if cancelled {
                cancel.cancel_now();
            }
            let clock: fn() -> Option<u64> = if now == 1000 { || Some(1000) } else { || Some(1) };
            let mut cx = StepContext::new(OperationId(operation), Generation(generation), StepBudget::new(8, 2000), cancel, clock, &mut sequence);
            assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
            assert_eq!(request.input_offset, 0);
            let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 999), root_cancel_token(), || Some(1), &mut sequence);
            assert_eq!(request.step_input(&mut cx), MemberOpenInputStep::Rejected(expected));
            assert_eq!(request.retained_input_bytes(), 3);
            retire_request(&mut request);
        }
        let mut request = request_for(&[1, 97, 83]);
        let mut sequence = 0;
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(8, 1), root_cancel_token(), || Some(1), &mut sequence);
        assert!(matches!(request.step_input(&mut cx), MemberOpenInputStep::Pending(MemberOpenProgress { completed: 0, .. })));
        retire_request(&mut request);
        eprintln!("[DEBUG] member input: 8 canonical frames x 3 fuel grants, 4 sticky authority denials, exact step deadline and retained close");
    }

    #[test]
    fn member_open_request_rejection_retains_exact_pages_and_identity() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixture/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let bytes = row["bytes"].as_u64().unwrap() as usize;
            let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: bytes.max(1).div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: bytes.max(1) }).unwrap();
            for chunk in vec![0x63; bytes].chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
                pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap();
            }
            if row["sealed"].as_bool().unwrap() {
                pages.seal().unwrap();
            }
            let expected = ArtifactRef { artifact_id: row["artifactId"].as_str().unwrap().into(), dialect: crate::os_io::ArtifactDialect { artifact_kind: "s.test.member".into(), standard: "1".into(), subset: "*".into() } };
            let owner = row["ownerChildId"].as_str().map(|child_id| OwnerRef { parent: ArtifactRef { artifact_id: "parent".into(), dialect: expected.dialect.clone() }, slot: "content".into(), child_id: child_id.into() });
            let request = MemberOpenRequest::new(OperationId(1), Generation(1), row["expiresAtUs"].as_u64().unwrap(), expected.clone(), owner.clone(), pages);
            let result = request.admit(row["nowUs"].as_u64().unwrap());
            assert_eq!(result.is_ok(), row["admitted"].as_bool().unwrap(), "{}", row["id"]);
            let mut request = match result {
                Ok(request) => request,
                Err(rejected) => {
                    let reason = match rejected.diagnostic {
                        MemberOpenDiagnostic::Unsealed => "unsealed",
                        MemberOpenDiagnostic::Empty => "empty",
                        MemberOpenDiagnostic::Expired => "expired",
                        MemberOpenDiagnostic::Identity => "identity",
                        MemberOpenDiagnostic::Owner => "owner",
                        _ => panic!("unexpected admission reason"),
                    };
                    assert_eq!(reason, row["reason"].as_str().unwrap());
                    rejected.request
                }
            };
            assert_eq!(request.expected(), &expected);
            assert_eq!(request.owner(), owner.as_ref());
            assert_eq!(request.retained_input_bytes(), bytes);
            assert!(matches!(request.close_step(0, 0).unwrap(), SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }));
            assert_eq!(request.retained_input_bytes(), bytes);
            let identity_bytes = expected.artifact_id.len()
                + expected.dialect.artifact_kind.len()
                + expected.dialect.standard.len()
                + expected.dialect.subset.len()
                + owner.as_ref().map_or(0, |owner| owner.parent.artifact_id.len() + owner.parent.dialect.artifact_kind.len() + owner.parent.dialect.standard.len() + owner.parent.dialect.subset.len() + owner.slot.len() + owner.child_id.len());
            let mut released = 0;
            for _ in 0..100_000 {
                match request.close_step(1, 7).unwrap() {
                    SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1 && released_bytes <= 7);
                        released += released_bytes;
                    }
                    SnapshotRetirementStep::Complete => break,
                    SnapshotRetirementStep::Blocked => panic!("request owns no shared root"),
                }
            }
            assert!(request.terminal_is_empty());
            assert_eq!(released, bytes + identity_bytes, "{}: exact input and identity byte ownership", row["id"]);
        }
    }
}
