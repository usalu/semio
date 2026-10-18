//! 📂️ Caller-retained member-open authority: bounded input ownership, scoped progress and close.

#[path = "🏭️operation/🛂️receiving/🦀️.rs"]
pub mod receiving;
#[path = "🏭️operation/🧬️schema/📦️decoder/🦀️.rs"]
pub mod decoder;
#[path = "📜️history/🦀️.rs"]
pub mod history;

pub use crate::decoder::{MemberSnapshotDecoder, ArtifactPackSnapshotDecoder};
#[path="🎟️admission/🦀️.rs"]
pub mod source_admission;
pub use source_admission::{MemberOpenSourceAdmission,diagnostic_error as member_admission_diagnostic};

use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_owned_schema_decode::{OwnedSchemaDecodePage, OwnedSchemaDecodePages};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use {semio_framework_artifact_reference::ArtifactRef};
use semio_framework_job::{Generation, OperationId, StepContext, OriginalTurnFailure};
use std::mem::ManuallyDrop;

#[path = "🌱️genesis/🦀️.rs"]
pub mod genesis;
pub use genesis::{MemberGenesisEnvelopeEncoder, MemberGenesisEnvelopeProgress, MemberGenesisEnvelopeSource};

pub const MEMBER_OPEN_IDENTITY_BYTES: usize = 256;

/// 🎟️ Intersects a nested policy with the remaining original five-axis context authority.
pub fn member_step_grant(cx: &StepContext<'_>, grant: RetainedCloneGrant) -> RetainedCloneGrant {
    let caller = cx.retained_grant();
    RetainedCloneGrant { maximum_items: grant.maximum_items.min(caller.maximum_items), maximum_copy_bytes: grant.maximum_copy_bytes.min(caller.maximum_copy_bytes), maximum_capacity_bytes: grant.maximum_capacity_bytes.min(caller.maximum_capacity_bytes), maximum_release_bytes: grant.maximum_release_bytes.min(caller.maximum_release_bytes), maximum_depth: grant.maximum_depth.min(caller.maximum_depth) }
}

/// 🧾️ Records the genuine completed receipt once in the original caller's turn.
pub fn record_member_step(cx: &mut StepContext<'_>, progress: RetainedCloneProgress) -> Result<(), MemberOpenDiagnostic> {
    let fuel = progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()).ok_or(MemberOpenDiagnostic::Capacity)?;
    cx.consume_retained(progress).map_err(|_| MemberOpenDiagnostic::Capacity)?;
    cx.consume_fuel(fuel);
    Ok(())
}

/// 🧾️ Receives a generated member shell's actual paid release with its complete original refusal carrier.
pub fn receive_member_close(cx:&mut StepContext<'_>,result:Result<RetainedCloneStep,semio_framework_value::ValueError>)->Result<RetainedCloneStep,OriginalTurnFailure>{
    let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
    cx.consume_fuel(progress.copied_items.saturating_add(progress.copied_bytes)as u64);
    let receipt_refusal=cx.consume_retained(progress).err();
    match result{Err(primary)=>Err(OriginalTurnFailure{primary:primary.into(),receipt_refusal,context_refusal:None}),Ok(step)=>match receipt_refusal{None=>Ok(step),Some(refusal)=>Err(OriginalTurnFailure{primary:semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original member close receipt was refused").with_retained_progress(progress).into(),receipt_refusal:Some(refusal),context_refusal:None})}}
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
    pub snapshot_start: usize,
    snapshot_bytes: usize,
    pub history_start: usize,
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
    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<MemberOpenStep<Self::Member>,OriginalTurnFailure>;
    fn close_step(&mut self, parent:&semio_framework_job::OriginalCallerSourceParent, cx: &mut StepContext<'_>) -> Result<RetainedCloneStep, OriginalTurnFailure>;
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError>;
    fn terminal_drop_byte_demand(&self) -> Option<usize>;
}

pub struct MemberOpenRequest {
    pub actor: ManuallyDrop<semio_framework_os_spr::ActorId>,
    pub operation: OperationId,
    pub generation: Generation,
    pub expires_at_us: u64,
    pub expected: ManuallyDrop<Option<ArtifactRef>>,
    pub owner: ManuallyDrop<Option<OwnerRef>>,
    pub pages: ManuallyDrop<Option<OwnedSchemaDecodePages>>,
    closing_identity_field: u8,
    pub closing_actor: Option<String>,
    pub input_offset: usize,
    snapshot_bytes: u64,
    framed: Option<MemberOpenFrame>,
    pub rejected: Option<MemberOpenDiagnostic>,
    pub admitted: bool,
    pub closing: bool,
    pub detached: bool,
}

pub struct OriginalMemberHistorySource<'a>{pub pages:&'a OwnedSchemaDecodePages,pub start:usize,pub identity:protocol::codec::OriginalWireSourceIdentity}
impl protocol::codec::RetainedWireByteSource for OriginalMemberHistorySource<'_>{
 fn original_source_identity(&self)->protocol::codec::OriginalWireSourceIdentity{self.identity}
 fn byte_at(&self,index:usize)->Option<u8>{if index>=self.identity.extent{return None}self.pages.byte_at(self.start.checked_add(index)?)}
}

impl MemberOpenRequest {
    pub fn original_history_source(&self,cx:&StepContext<'_>)->Result<OriginalMemberHistorySource<'_>,MemberOpenDiagnostic>{
        self.check_step_authority(cx)?;self.original_history_close_source(cx)
    }
    pub fn original_history_close_source(&self,cx:&StepContext<'_>)->Result<OriginalMemberHistorySource<'_>,MemberOpenDiagnostic>{
        if !self.admitted||self.detached||cx.operation()!=self.operation||cx.generation()!=self.generation{return Err(MemberOpenDiagnostic::Stale)}
        let frame=self.framed.ok_or(MemberOpenDiagnostic::Malformed)?;let(start,extent)=frame.history_range();let pages=self.pages.as_ref().ok_or(MemberOpenDiagnostic::Stale)?;
        if !pages.is_sealed()||start.checked_add(extent).is_none_or(|end|end>pages.byte_count()){return Err(MemberOpenDiagnostic::Malformed)}
        Ok(OriginalMemberHistorySource{pages,start,identity:protocol::codec::OriginalWireSourceIdentity{address:pages.slot_address(),extent,authority:Some((self.operation.0,self.generation.0))}})
    }
    pub fn new(operation: OperationId, generation: Generation, expires_at_us: u64, expected: ArtifactRef, owner: Option<OwnerRef>, pages: OwnedSchemaDecodePages, actor: semio_framework_os_spr::ActorId) -> Self {
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
    pub fn actor(&self) -> &semio_framework_os_spr::ActorId { &self.actor }

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
        let maximum = output.len().min(length - offset).min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES);
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
                if grant.maximum_release_bytes<semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<String>(){return Ok(RetainedCloneStep::Progress(empty));}
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

    pub fn identity_string_mut(&mut self) -> Option<&mut String> {
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
    pub fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
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












