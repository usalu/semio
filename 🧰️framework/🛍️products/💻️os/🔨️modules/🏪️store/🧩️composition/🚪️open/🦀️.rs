//! 📂️ Caller-retained member-open authority: bounded input ownership, scoped progress and close.

#[path = "📜️history/🦀️.rs"]
pub(crate) mod history;
#[path = "🏭️operation/🦀️.rs"]
mod operation;
pub use history::factory::MemberOpenDeclaration;
pub use operation::{admit_member_input_buffer, MemberSnapshotOpenProgress, InitialMemberStoreOpen, MemberSnapshotOpenOperation, MemberSnapshotOpenStep, PackMemberSnapshotOpen, UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};

use super::{ErasedSnapshotRetirement, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OwnerRef, SpaceMember};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use {semio_framework_artifact_reference::ArtifactRef};
use semio_framework_job::{Generation, OperationId, StepContext};
use std::mem::ManuallyDrop;

#[path = "🌱️genesis/🦀️.rs"]
mod genesis;
pub use genesis::{MemberGenesisEnvelopeEncoder, MemberGenesisEnvelopeProgress, MemberGenesisEnvelopeSource};

pub const MEMBER_OPEN_IDENTITY_BYTES: usize = 256;

/// 🎟️ Intersects a nested policy with the remaining original five-axis context authority.
pub(crate) fn member_step_grant(cx: &StepContext<'_>, grant: RetainedCloneGrant) -> RetainedCloneGrant {
    let caller = cx.retained_grant();
    RetainedCloneGrant { maximum_items: grant.maximum_items.min(caller.maximum_items), maximum_copy_bytes: grant.maximum_copy_bytes.min(caller.maximum_copy_bytes), maximum_capacity_bytes: grant.maximum_capacity_bytes.min(caller.maximum_capacity_bytes), maximum_release_bytes: grant.maximum_release_bytes.min(caller.maximum_release_bytes), maximum_depth: grant.maximum_depth.min(caller.maximum_depth) }
}

/// 🧾️ Records the genuine completed receipt once in the original caller's turn.
pub(crate) fn record_member_step(cx: &mut StepContext<'_>, progress: RetainedCloneProgress) -> Result<(), MemberOpenDiagnostic> {
    let fuel = progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()).ok_or(MemberOpenDiagnostic::Capacity)?;
    cx.consume_retained(progress).map_err(|_| MemberOpenDiagnostic::Capacity)?;
    cx.consume_fuel(fuel);
    Ok(())
}

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
    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberOpenStep<Self::Member>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn terminal_drop_byte_demand(&self) -> Option<usize>;
}

pub struct MemberOpenRequest {
    actor: ManuallyDrop<crate::os_spr::ActorId>,
    operation: OperationId,
    generation: Generation,
    expires_at_us: u64,
    expected: ManuallyDrop<Option<ArtifactRef>>,
    owner: ManuallyDrop<Option<OwnerRef>>,
    pages: ManuallyDrop<Option<OwnedSchemaDecodePages>>,
    closing_identity_field: u8,
    closing_actor: Option<String>,
    input_offset: usize,
    snapshot_bytes: u64,
    framed: Option<MemberOpenFrame>,
    rejected: Option<MemberOpenDiagnostic>,
    admitted: bool,
    closing: bool,
    detached: bool,
}

impl MemberOpenRequest {
    pub fn new(operation: OperationId, generation: Generation, expires_at_us: u64, expected: ArtifactRef, owner: Option<OwnerRef>, pages: OwnedSchemaDecodePages, actor: crate::os_spr::ActorId) -> Self {
        Self {
            actor: ManuallyDrop::new(actor),
            operation,
            generation,
            expires_at_us,
            expected: ManuallyDrop::new(Some(expected)),
            owner: ManuallyDrop::new(owner),
            pages: ManuallyDrop::new(Some(pages)),
            closing_identity_field: 0,
            closing_actor: None,
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
    pub fn actor(&self) -> &crate::os_spr::ActorId { &self.actor }

    pub fn owner(&self) -> Option<&OwnerRef> {
        self.owner.as_ref()
    }
    pub fn operation(&self) -> OperationId {
        self.operation
    }
    pub fn generation(&self) -> Generation {
        self.generation
    }
    pub fn expires_at_us(&self) -> u64 {
        self.expires_at_us
    }
    pub fn retained_input_bytes(&self) -> usize {
        self.pages.as_ref().map_or(0, OwnedSchemaDecodePages::byte_count)
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
        } else if !reference(self.expected()) || !text(&self.actor().0) {
            Some(MemberOpenDiagnostic::Identity)
        } else if self.owner().is_some_and(|owner| !reference(&owner.parent) || !text(&owner.slot) || !text(&owner.child_id)) {
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

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.detached { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member request requires admitted retirement depth")); }
        self.closing = true;
        if let Some(pages) = self.pages.as_mut() {
            if let Some(page) = pages.close_take_page() {
                let _ = page;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if !pages.terminal_is_empty() {
                let step = pages.close_backing_step(grant)?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            drop(self.pages.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.closing_identity_field == 0 {
            if self.actor.0.has_owner() {
                let original=std::mem::take(&mut self.actor.0);
                match original.close_original_lease(grant) {
                    Ok((value,progress))=>{self.closing_actor=value;return Ok(RetainedCloneStep::Progress(progress));},
                    Err((error,original))=>{self.actor.0=original;return Err(error);}
                }
            }
            let released_bytes=self.closing_actor.as_ref().map_or(0,String::capacity);
            if released_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.closing_actor.take());self.closing_identity_field=1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty}));
        }
        if self.closing_identity_field < 11 {
            let field = self.identity_string_mut();
            let released_bytes = field.as_ref().map_or(0, |field| field.capacity());
            if released_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            if let Some(field) = field { drop(std::mem::take(field)); }
            self.closing_identity_field += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        drop(self.expected.take());
        drop(self.owner.take());
        self.detached = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.detached && !self.actor.0.has_owner() && self.closing_actor.is_none() && self.expected.is_none() && self.owner.is_none() && self.pages.is_none()
    }

    fn identity_string_mut(&mut self) -> Option<&mut String> {
        match self.closing_identity_field {
            0 => None,
            1 => self.expected.as_mut().map(|value| &mut value.artifact_id),
            2 => self.expected.as_mut().map(|value| &mut value.dialect.artifact_kind),
            3 => self.expected.as_mut().map(|value| &mut value.dialect.standard),
            4 => self.expected.as_mut().map(|value| &mut value.dialect.subset),
            5 => self.owner.as_mut().map(|value| &mut value.parent.artifact_id),
            6 => self.owner.as_mut().map(|value| &mut value.parent.dialect.artifact_kind),
            7 => self.owner.as_mut().map(|value| &mut value.parent.dialect.standard),
            8 => self.owner.as_mut().map(|value| &mut value.parent.dialect.subset),
            9 => self.owner.as_mut().map(|value| &mut value.slot),
            10 => self.owner.as_mut().map(|value| &mut value.child_id),
            _ => None,
        }
    }

    /// 📏️ Queries the next original whole allocation without creating a retirement owner.
    pub fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        if self.detached { return Ok(0); }
        if let Some(pages) = self.pages.as_ref() { return Ok(if pages.page_count() == 0 { pages.allocation_byte_demand() } else { 0 }); }
        if self.closing_identity_field==0{return Ok(if self.actor.0.has_owner(){semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>()}else{self.closing_actor.as_ref().map_or(0,String::capacity)});}
        let expected = self.expected.as_ref();
        let owner = self.owner.as_ref();
        let field = match self.closing_identity_field {
            0 => None,
            1 => expected.map(|value| &value.artifact_id),
            2 => expected.map(|value| &value.dialect.artifact_kind),
            3 => expected.map(|value| &value.dialect.standard),
            4 => expected.map(|value| &value.dialect.subset),
            5 => owner.map(|value| &value.parent.artifact_id),
            6 => owner.map(|value| &value.parent.dialect.artifact_kind),
            7 => owner.map(|value| &value.parent.dialect.standard),
            8 => owner.map(|value| &value.parent.dialect.subset),
            9 => owner.map(|value| &value.slot),
            10 => owner.map(|value| &value.child_id),
            _ => None,
        };
        Ok(field.map_or(0, String::capacity))
    }
    pub fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(if !self.detached&&self.pages.is_none()&&self.closing_identity_field==0&&self.actor.0.has_owner(){std::mem::size_of::<Option<String>>()}else{0}) }
    pub fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    pub fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(usize::from(!self.detached)) }

}

impl ErasedSnapshotRetirement for MemberOpenRequest {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> { MemberOpenRequest::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { MemberOpenRequest::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_capacity_byte_demand(self, body) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { MemberOpenRequest::next_depth_demand(self) }
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

pub(super) struct MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P>,
{
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    owners: ManuallyDrop<Option<super::DocumentStoreOwners<P, M>>>,
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
    P: Clone + super::ToValue + super::FromValue + Send + Sync + 'static,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    pub(super) fn new(request: MemberOpenRequest, owners: super::DocumentStoreOwners<P, M>) -> Self {
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

impl<P, M> MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue + Send + Sync + 'static,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    fn retirement_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        let nested = |mut demand: RetirementDemand| { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "member-open owner depth overflow"))?; Ok(demand) };
        if let Some(active) = self.active.as_ref() { return super::artifact_retirement_box_demands(active, copy); }
        if let Some(runtime) = self.runtime.as_ref() { return nested(RetirementDemand { copy_bytes: runtime.next_close_copy_byte_demand()?, capacity_bytes: runtime.next_close_capacity_byte_demand(copy)?, release_bytes: runtime.next_close_release_byte_demand()?, depth: runtime.next_close_depth_demand()? }); }
        if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if self.envelope.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactStoreEnvelopeRetirement<P, M>>(), depth: 2, ..Default::default() }); }
        if let Some(initial) = self.initial.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.owners.as_ref().expect("member-open retains its initial factory").initial_snapshot_retirement.retirement_birth_bytes(initial), depth: 2, ..Default::default() }); }
        if self.history.is_some() { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<crate::os_spr::HistoryLog>(), depth: 2, ..Default::default() }); }
        if let Some(request) = self.request.as_ref() { return nested(RetirementDemand { copy_bytes: request.next_copy_byte_demand()?, capacity_bytes: request.next_capacity_byte_demand(copy)?, release_bytes: request.next_release_byte_demand()?, depth: request.next_depth_demand()? }); }
        self.owners.as_ref().map_or(Ok(Default::default()), |owners| nested(owners.uninstalled_owners_demands(copy)?))
    }
}

impl<P, M> ErasedSnapshotRetirement for MemberStoreOpenRetained<P, M>
where
    P: Clone + super::ToValue + super::FromValue + Send + Sync + 'static,
    M: Clone + super::ToValue + super::FromValue + super::Mutation<P> + Send + 'static,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "member-open exceeds admitted original owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.reject(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return super::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let owners = self.owners.as_ref().expect("member-open retains original catalog");
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.close_step(&owners.initial_snapshot_retirement, child)?;
            let terminal = runtime.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "member-open original initialization")?;
            if terminal { drop(self.runtime.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(edit) = self.pending_edit.take() {
            return match super::admit_artifact_retirement(edit, child, |edit| super::ArtifactStoreDecodedEditRetirement::new(edit, owners.mutation_retirement.clone())) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, edit)) => { *self.pending_edit = Some(edit); Err(error) }
            };
        }
        if let Some(envelope) = self.envelope.take() {
            return match super::admit_artifact_retirement(envelope, child, |envelope| super::ArtifactStoreEnvelopeRetirement::new(super::ArtifactEnvelope::from_owners(envelope), owners.initial_snapshot_retirement.clone(), owners.mutation_retirement.clone())) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, envelope)) => { *self.envelope = Some(envelope); Err(error) }
            };
        }
        if let Some(initial) = self.initial.take() {
            return match owners.initial_snapshot_retirement.retire_owned(initial, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "member-open initial factory changed its constructor receipt")); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, initial)) => { *self.initial = Some(initial); Err(error) }
            };
        }
        if let Some(history) = self.history.take() {
            return match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(history, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, history)) => { *self.history = Some(history); Err(error) }
            };
        }
        if let Some(request) = self.request.as_mut() {
            let step = request.close_step(child)?;
            let terminal = request.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "member-open original request")?;
            if terminal { drop(self.request.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let owners = self.owners.as_mut().expect("member-open catalog remains until terminal");
        let step = owners.close_uninstalled_owners_step(child)?;
        let terminal = owners.uninstalled_owners_terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "member-open original catalog")?;
        if terminal { drop(self.owners.take()); self.terminal = true; }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.owners.is_none() && self.history.is_none() && self.initial.is_none() && self.pending_edit.is_none() && self.envelope.is_none() && self.runtime.is_none() && self.active.is_none()
    }
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.retirement_demands(0)?.depth) }
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
