//! 🏭️ Closed request-owned member opening: one typed decoder, one verified history, one store handoff.

#[path = "📥️lifted/🚪️open/🦀️.rs"]
pub mod lifted_member_open;
use semio_framework_os_store_member_open::history::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryStep};
use semio_framework_os_store_space_member::factory::{MemberFactorySelection, MemberFactorySelectionStep, SelectedMemberHistoryDictionary, SelectedMemberHistoryInput, SelectedVerifiedMemberHistory};
use semio_framework_os_store_member_open::history::{MemberHistoryInputStep, MemberHistoryVerification};
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_os_store_member_open::source_admission::{MemberOpenSourceAdmission, diagnostic_error as member_admission_diagnostic};
use semio_framework_os_store_member_open::{MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
use semio_framework_os_spr::format::retained::RetainedSprLimits;
use pack::snapshot::ArtifactPack;

pub use semio_framework_os_store_member_open::decoder::{MemberSnapshotDecoder, ArtifactPackSnapshotDecoder};
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_space_member::lifted_store::MemberFactory;
use semio_framework_os_store_engine_owners::member_producer::MemberStoreProducer;
use semio_framework_value::{FromValue, ToValue};
use semio_framework_os_spr::Mutation;
use semio_framework_os_pack::{OpBinary, OpText};
use semio_framework_job::{Generation, OperationId, StepContext, OriginalTurnFailure, OriginalCallerSourceParent, OriginalCallerSourceLoan};
use std::{marker::PhantomData, mem::ManuallyDrop};


use semio_framework_os_store_member_open::receiving::MemberHistoryReceiving;
pub use semio_framework_os_store_engine_owners::lifted_operation::{MemberSnapshotOpenOperation, MemberSnapshotOpenProgress, MemberSnapshotOpenStep};
pub use semio_framework_os_store_space_member::lifted_operation::{UnsupportedMemberFactoryOpen, UnsupportedMemberSnapshotOpen};

/// 📥️ Funds one exact original contiguous input allocation before installing its retained owner.
pub fn admit_member_input_buffer(source: &mut Option<Vec<u8>>, capacity: usize, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneProgress>, ValueError> {
    if source.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "member input buffer already retained")); }
    if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < capacity { return Ok(None); }
    std::alloc::Layout::array::<u8>(capacity).map_err(|_| ValueError::literal(ValueRefusalKind::OwnershipLimit, "member input buffer extent overflow"))?;
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(capacity).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "member input buffer allocation refused"))?;
    let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: buffer.capacity(), ..Default::default() };
    *source = Some(buffer);
    semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original member input buffer birth")?;
    Ok(Some(progress))
}















/// 📥️ Original request-owned opening with a selected typed decoder and the same paid input/close mechanism.
pub struct ProducedMemberSnapshotOpen<P,D> {
    pub decoder:PhantomData<fn()->D>,
    pub request: ManuallyDrop<Option<MemberOpenRequest>>,
    pub snapshot: ManuallyDrop<Option<P>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub input: Vec<u8>,
    pub expected_bytes: Option<usize>,
    pub diagnostic: Option<MemberOpenDiagnostic>,
    pub terminal: bool,
}

const PACK_MEMBER_SNAPSHOT_CHUNK_BYTES: usize = 4_096;

impl<P,D> ProducedMemberSnapshotOpen<P,D> {
    fn publish_input(&mut self, opening: MemberOpenProgress, progress: RetainedCloneProgress, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep {
        if cx.consume_retained(progress).is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
        MemberSnapshotOpenStep::Pending(MemberSnapshotOpenProgress { opening, retained_progress: progress })
    }
    pub fn pending(&mut self, total: usize, progress: RetainedCloneProgress, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep {
        self.publish_input(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: self.input.len() as u64, total: total as u64 }, progress, cx)
    }
    pub fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberSnapshotOpenStep {
        self.diagnostic.get_or_insert(diagnostic);
        MemberSnapshotOpenStep::Rejected(self.diagnostic.unwrap_or(diagnostic))
    }
}

impl<P: semio_framework_value::retirement::RetireOwned,D:MemberSnapshotDecoder<P>> MemberSnapshotOpenOperation for ProducedMemberSnapshotOpen<P,D> {
    type Snapshot = P;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> { request.admitted_expected().map(|_| 0) }

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self { decoder:PhantomData, request: ManuallyDrop::new(Some(request)), snapshot: ManuallyDrop::new(None), active: ManuallyDrop::new(None), input: Vec::new(), expected_bytes: None, diagnostic: None, terminal: false })
    }

    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberSnapshotOpenStep {
        let caller = cx.retained_grant();
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(caller.maximum_items), maximum_copy_bytes: grant.maximum_copy_bytes.min(caller.maximum_copy_bytes), maximum_capacity_bytes: grant.maximum_capacity_bytes.min(caller.maximum_capacity_bytes), maximum_release_bytes: grant.maximum_release_bytes.min(caller.maximum_release_bytes), maximum_depth: grant.maximum_depth.min(caller.maximum_depth) };
        if let Some(diagnostic) = self.diagnostic {
            return MemberSnapshotOpenStep::Rejected(diagnostic);
        }
        if self.terminal {
            return MemberSnapshotOpenStep::Rejected(MemberOpenDiagnostic::Stale);
        }
        if let Err(diagnostic) = self.request.as_ref().expect("pack member decoder retains its request").check_step_authority(cx) { return self.reject(diagnostic); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || cx.should_yield() { return self.pending(self.expected_bytes.unwrap_or(0), RetainedCloneProgress::default(), cx); }
        let before = self.request.as_ref().expect("pack member decoder retains its request").input_offset;
        let frame = match self.request.as_mut().expect("pack member decoder retains its request").step_input(cx) {
            semio_framework_os_store_member_open::MemberOpenInputStep::Framed(frame) => frame,
            semio_framework_os_store_member_open::MemberOpenInputStep::Pending(progress) => return self.publish_input(progress, RetainedCloneProgress { copied_items: 1, ..Default::default() }, cx),
            semio_framework_os_store_member_open::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let expected_bytes = frame.snapshot_range().1;
        if self.request.as_ref().expect("pack member decoder retains its request").input_offset != before { return self.pending(expected_bytes, RetainedCloneProgress { copied_items: 1, ..Default::default() }, cx); }
        if self.expected_bytes.is_none() {
            let mut retained = None;
            let result = admit_member_input_buffer(&mut retained, expected_bytes, grant);
            if let Some(input) = retained { self.input = input; self.expected_bytes = Some(expected_bytes); }
            return match result {
                Ok(Some(progress)) => { cx.consume_fuel(progress.copied_items as u64); self.pending(expected_bytes, progress, cx) }
                Ok(None) => self.pending(expected_bytes, RetainedCloneProgress::default(), cx),
                Err(_) => self.reject(MemberOpenDiagnostic::Capacity),
            };
        }
        cx.set_stage("member-open.pack-snapshot");
        if self.input.len() < expected_bytes {
            let mut chunk = [0u8; PACK_MEMBER_SNAPSHOT_CHUNK_BYTES];
            let maximum = chunk.len().min(expected_bytes - self.input.len()).min(grant.maximum_copy_bytes / 2).min(usize::try_from(cx.fuel_remaining() / 2).unwrap_or(usize::MAX));
            if maximum == 0 { return self.pending(expected_bytes, RetainedCloneProgress::default(), cx); }
            let copied = match self.request.as_ref().expect("pack member decoder retains its request").copy_snapshot_chunk(self.input.len(), &mut chunk[..maximum], cx) {
                Ok(copied) => copied,
                Err(diagnostic) => return self.reject(diagnostic),
            };
            self.input.extend_from_slice(&chunk[..copied]);
            cx.consume_fuel(copied as u64);
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied * 2, ..Default::default() };
            return self.pending(expected_bytes, progress, cx);
        }
        if let Err(diagnostic) = self.request.as_ref().expect("pack member decoder retains its request").check_step_authority(cx) {
            return self.reject(diagnostic);
        }
        if self.snapshot.is_none() {
            match D::decode_snapshot(&self.input) {
                Ok(snapshot) => *self.snapshot = Some(snapshot),
                Err(_) => return self.reject(MemberOpenDiagnostic::Decode),
            }
        }
        MemberSnapshotOpenStep::Ready
    }

    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Option<(P, MemberOpenRequest)> {
        if self.terminal || self.diagnostic.is_some() || self.request.as_ref()?.check_step_authority(cx).is_err() || self.input.len() != self.expected_bytes? {
            return None;
        }
        let snapshot = self.snapshot.take()?;
        let request = self.request.take()?;
        self.expected_bytes = None;
        self.terminal = self.input.capacity() == 0;
        Some((snapshot, request))
    }
}

impl<P: semio_framework_value::retirement::RetireOwned,D> ProducedMemberSnapshotOpen<P,D> {
    pub fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return semio_framework_os_store_read::artifact_retirement_box_demands(active, body); }
        if self.snapshot.is_some() { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<P>(), depth: 2, ..Default::default() }); }
        if self.request.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.request, body); }
        Ok(RetirementDemand { release_bytes: self.input.capacity(), depth: usize::from(!self.terminal), ..Default::default() })
    }
}
impl<P: semio_framework_value::retirement::RetireOwned,D> ErasedSnapshotRetirement for ProducedMemberSnapshotOpen<P,D> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "pack open retirement exceeds admitted depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant); }
        if let Some(snapshot) = self.snapshot.take() {
            return match semio_framework_value::retirement::admit_owned_retirement(snapshot, RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, snapshot)) => { *self.snapshot = Some(snapshot); Err(error) }
            };
        }
        if self.request.is_some() { let step = semio_framework_os_store_read::retirement_owner_close(&mut self.request, grant)?; return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.input.capacity() != 0 {
            let released_bytes = self.input.capacity();
            drop(std::mem::take(&mut self.input));
            self.expected_bytes = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        self.expected_bytes = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.capacity() == 0 && self.expected_bytes.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
}

impl<P,D> Drop for ProducedMemberSnapshotOpen<P,D> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.capacity() == 0), "pack member decoder dropped before exact handoff or bounded retirement");
    }
}








#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Snapshot,
    CaptureGenesis,
    History,
    Select,
    Selected,
    Dictionary,
    AdmitHistory,
    DecodeHistory,
    BeginHydration,
    Hydrate,
    RetireInput,
    Ready,
    Rejected,
}

pub struct InitialMemberStoreOpen<F, P, M, C>
where
    F: MemberFactory + 'static,
    C: MemberStoreProducer<P,M>,
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    pub producer:C,
    pub source_loan: ManuallyDrop<Option<OriginalCallerSourceLoan>>,
    snapshot_open: ManuallyDrop<Option<C::SnapshotOpen>>,
    pub snapshot: ManuallyDrop<Option<P>>,
    genesis_request: ManuallyDrop<Option<MemberOpenRequest>>,
    pub genesis_pack: ManuallyDrop<Option<Vec<u8>>>,
    genesis_hasher: semio_framework_hash::Hasher,
    pub history: ManuallyDrop<Option<MemberHistoryVerification>>,
    pub selection: ManuallyDrop<Option<MemberFactorySelection<F>>>,
    /// The selected history input awaiting its dictionary owner: kept across steps when the step budget
    /// is spent between selection and dictionary admission, instead of failing the open as stale.
    pub selected: ManuallyDrop<Option<SelectedMemberHistoryInput<F>>>,
    rejected_history_input: ManuallyDrop<Option<semio_framework_os_store_member_open::history::VerifiedMemberHistoryInput>>,
    pub dictionary: ManuallyDrop<Option<SelectedMemberHistoryDictionary<F>>>,
    pub witness: ManuallyDrop<Option<SelectedVerifiedMemberHistory<F>>>,
    history_page: Option<Box<[u8; semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES]>>,
    history_receiving: ManuallyDrop<Option<MemberHistoryReceiving>>,
    decoded_history: ManuallyDrop<Option<semio_framework_os_spr::HistoryLog>>,
    hydration: ManuallyDrop<Option<semio_framework_os_store_document::persisted_document_hydration::RetainedPersistedDocumentHydration<P, M>>>,
    pub owners: ManuallyDrop<Option<semio_framework_os_store_engine_owners::DocumentStoreOwners<P, M>>>,
    constructor_error: ManuallyDrop<Option<ValueError>>,
    pub member: ManuallyDrop<Option<Box<ArtifactStore<P, M>>>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub operation: OperationId,
    pub generation: Generation,
    pub expires_at_us: u64,
    pub phase: Phase,
    input_buffer_progress: RetainedCloneProgress,
    pub diagnostic: Option<MemberOpenDiagnostic>,
}

impl<F, P, M, C> InitialMemberStoreOpen<F, P, M, C>
where
    F: MemberFactory + 'static,
    C: MemberStoreProducer<P,M>,
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn history_page_due(&self) -> bool { self.history_page.is_some() }
    pub fn payload_terminal_is_empty(&self)->bool{self.ownership_is_empty()&&!self.history_page_due()}
    pub fn validate_source_context(&self,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)}
    pub fn take_source_before_shell_return(&mut self,cx:&StepContext<'_>)->Result<OriginalCallerSourceLoan,OriginalTurnFailure>{if !self.payload_terminal_is_empty(){return Err(member_admission_diagnostic(MemberOpenDiagnostic::Stale).into())}self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;Ok(self.source_loan.take().unwrap())}

    fn ownership_is_empty(&self) -> bool {
        self.snapshot_open.is_none()
            && self.snapshot.is_none()
            && self.genesis_request.is_none()
            && self.genesis_pack.is_none()
            && self.history.is_none()
            && self.selection.is_none()
            && self.selected.is_none()
            && self.rejected_history_input.is_none()
            && self.dictionary.is_none()
            && self.witness.is_none()
            && self.history_receiving.is_none()
            && self.decoded_history.is_none()
            && self.hydration.is_none()
            && self.owners.is_none()
            && self.constructor_error.is_none()
            && self.member.is_none()
            && self.active.is_none()
    }
}

impl<F, P, M, C> InitialMemberStoreOpen<F, P, M, C>
where
    F: MemberFactory + 'static,
    C: MemberStoreProducer<P,M>,
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_owned_field(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.ownership_is_empty() && !self.history_page_due() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "initial member close exceeds admitted depth")); }
        self.phase = Phase::Rejected;
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.constructor_error.is_some() { return semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.constructor_error, &mut self.active, grant); }
        if self.genesis_request.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.genesis_request, grant); }
        if self.genesis_pack.is_some() { return semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.genesis_pack, &mut self.active, grant); }
        if self.hydration.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.hydration, grant); }
        if let Some(member) = self.member.as_mut() {
            if semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()) {
                let extent = size_of_val(member.as_ref());
                if grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(empty)); }
                self.member.take();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_step(member.as_mut(), child)?;
            admit_retained_clone_close(child, step, semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()), "initial member Store child")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.decoded_history.is_some() { return semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.decoded_history, &mut self.active, grant); }
        macro_rules! close_field {
            ($field:ident) => {
                if self.$field.is_some() { return semio_framework_os_store_read::retirement_owner_close(&mut self.$field, grant); }
            };
        }
        close_field!(witness);
        close_field!(dictionary);
        close_field!(selected);
        close_field!(rejected_history_input);
        close_field!(selection);
        close_field!(history);
        close_field!(snapshot_open);
        if self.snapshot.is_some() {
            let factory = &self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "member open lost original snapshot factory"))?.initial_snapshot_retirement;
            let birth = factory.retirement_birth_bytes(self.snapshot.as_ref().expect("observed original snapshot remains retained"));
            if grant.maximum_capacity_bytes < birth { return Ok(RetainedCloneStep::Progress(empty)); }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let original = self.snapshot.take().expect("member original snapshot remains retained");
            match factory.retire_owned(original, child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "member initial snapshot birth")?;
                    if progress.retained_capacity_bytes != birth { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "member snapshot birth differs from its exact quote")); }
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                Err((error, original)) => { *self.snapshot = Some(original); return Err(error); }
            }
        }
        if let Some(owners) = self.owners.as_mut() {
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owners.close_uninstalled_owners_step(child)?;
            let terminal = owners.uninstalled_owners_terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "member original catalog")?;
            if terminal { self.owners.take(); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.history_page.is_some() {
            let extent = semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES;
            if grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(empty)); }
            self.history_page = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..empty }));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(receiving)=self.history_receiving.as_ref(){return receiving.demands(body)}
        if let Some(active) = self.active.as_ref() { return semio_framework_os_store_read::artifact_retirement_box_demands(active, body); }
        if self.constructor_error.is_some() { return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.constructor_error); }
        if self.genesis_request.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.genesis_request, body); }
        if self.genesis_pack.is_some() { return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.genesis_pack); }
        if self.hydration.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.hydration, body); }
        if let Some(member) = self.member.as_ref() {
            if semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()) { return Ok(RetirementDemand { release_bytes: size_of_val(member.as_ref()), depth: 1, ..Default::default() }); }
            let mut demand = semio_framework_os_store_space_member::lifted_store::SpaceMember::close_owned_demands(member.as_ref(), body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "member Store depth overflow"))?;
            return Ok(demand);
        }
        if self.decoded_history.is_some() { return semio_framework_os_store_read::artifact_retirement_owned_birth_demands(&self.decoded_history); }
        macro_rules! child { ($field:ident) => { if self.$field.is_some() { return semio_framework_os_store_read::retirement_owner_demands(&self.$field, body); } }; }
        child!(witness); child!(dictionary); child!(selected); child!(rejected_history_input); child!(selection); child!(history); child!(snapshot_open);
        if self.snapshot.is_some() {
            let factory = &self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "member open lost original snapshot factory"))?.initial_snapshot_retirement;
            return Ok(RetirementDemand { capacity_bytes: factory.retirement_birth_bytes(self.snapshot.as_ref().expect("observed original snapshot remains retained")), depth: 2, ..Default::default() });
        }
        if let Some(owners) = self.owners.as_ref() {
            let mut demand = owners.uninstalled_owners_demands(body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "member catalog depth overflow"))?;
            return Ok(demand);
        }
        if self.history_page.is_some() { return Ok(RetirementDemand { release_bytes: semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES, depth: 1, ..Default::default() }); }
        Ok(RetirementDemand{depth:usize::from(self.source_loan.is_some()),..Default::default()})
    }

    pub fn begin_birth_demand(request: &MemberOpenRequest,producer:&C) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, MemberOpenDiagnostic> {
        let source = producer.member_store_owners_birth_demand().map_err(|_| MemberOpenDiagnostic::Capacity)?;
        let capacity_bytes = C::SnapshotOpen::begin_birth_bytes(request)?.checked_add(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).and_then(|bytes| bytes.checked_add(source.capacity_bytes)).ok_or(MemberOpenDiagnostic::Capacity)?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: source.depth.max(1) })
    }

    pub fn begin(admission:&mut MemberOpenSourceAdmission,cx:&mut StepContext<'_>,grant:RetainedCloneGrant,producer:C)->Result<(Self,RetainedCloneProgress),OriginalTurnFailure>{
        let request=admission.request.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?;
        request.check_step_authority(cx).map_err(member_admission_diagnostic)?;
        admission.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;
        let operation=request.operation();let generation=request.generation();let expires_at_us=request.expires_at_us();
        let required=Self::begin_birth_demand(request,&producer).map_err(member_admission_diagnostic)?;
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<required.depth||grant.maximum_capacity_bytes<required.capacity_bytes{return Err(member_admission_diagnostic(MemberOpenDiagnostic::Capacity).into())}
        let snapshot_birth=C::SnapshotOpen::begin_birth_bytes(request).map_err(member_admission_diagnostic)?;
        let source_capacity=producer.member_store_owners_birth_demand()?.capacity_bytes;
        let source_grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: source_capacity, ..grant };
        let (owners, mut diagnostic, constructor_error, source_progress) = match producer.member_store_owners(source_grant) {
            Ok((owners, progress)) => {
                let diagnostic = if progress.fits(source_grant) && progress.retained_capacity_bytes == source_capacity { None } else { Some(MemberOpenDiagnostic::Capacity) };
                (Some(owners), diagnostic, None, progress)
            }
            Err(refused) => (refused.owners, Some(MemberOpenDiagnostic::Capacity), Some(refused.error), refused.progress),
        };
        let request=admission.request.take().unwrap();
        let source_loan=admission.source_loan.take().unwrap();
        let (snapshot_open, genesis_request) = if diagnostic.is_none() {
            match C::SnapshotOpen::begin(request) {
                Ok(snapshot_open) => (Some(snapshot_open), None),
                Err(rejected) => { diagnostic = Some(rejected.diagnostic); (None, Some(rejected.request)) }
            }
        } else { (None, Some(request)) };
        let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:source_progress.retained_capacity_bytes+usize::from(snapshot_open.is_some())*snapshot_birth+semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES,..source_progress};
        Ok((Self {
            producer,
            source_loan: ManuallyDrop::new(Some(source_loan)),
            snapshot_open: ManuallyDrop::new(snapshot_open),
            snapshot: ManuallyDrop::new(None),
            genesis_request: ManuallyDrop::new(genesis_request),
            genesis_pack: ManuallyDrop::new(None),
            genesis_hasher: semio_framework_hash::Hasher::new(),
            history: ManuallyDrop::new(None),
            selection: ManuallyDrop::new(None),
            selected: ManuallyDrop::new(None),
            rejected_history_input: ManuallyDrop::new(None),
            dictionary: ManuallyDrop::new(None),
            witness: ManuallyDrop::new(None),
            history_page: Some(Box::new([0; semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES])),
            history_receiving: ManuallyDrop::new(None),
            decoded_history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(owners),
            constructor_error: ManuallyDrop::new(constructor_error),
            member: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            operation,
            generation,
            expires_at_us,
            phase: if diagnostic.is_some() { Phase::Rejected } else { Phase::Snapshot },
            input_buffer_progress: Default::default(),
            diagnostic,
        },progress))
    }

    pub fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        MemberOpenStep::Rejected(self.diagnostic.unwrap())
    }

    fn check_witness(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.witness.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx)
    }

    pub fn check_authority(&self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
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

    fn replay_progress(&self) -> MemberOpenProgress {
        let total = self.decoded_history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: total as u64 }
    }

    fn drive_active_hydration_retirement(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<bool, MemberOpenDiagnostic> {
        if self.active.is_none() { return Ok(false); }
        if cx.should_yield() { return Ok(true); }
        let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        let step = semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, child).map_err(|_| MemberOpenDiagnostic::Initialization)?;
        let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
        semio_framework_os_store_member_open::record_member_step(cx, progress)?;
        Ok(true)
    }

    /// Admits the selected history input into its dictionary owner; a spent step budget or deadline
    /// keeps the selected input retained for the next step rather than rejecting the open.
    fn begin_selected_dictionary(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        let Some(selected) = self.selected.as_mut() else { return self.reject(MemberOpenDiagnostic::Stale) };
        let original=selected.history_limits();
        let limits=MemberHistoryDictionaryLimits{dictionary_entries:original.records as usize,dictionary_bytes:u64::from(original.maximum_text_bytes),records:u64::from(original.records)};
        match selected.begin_dictionary(limits, cx) {
            Ok(Some(dictionary)) => {
                let selected = self.selected.take().expect("selected member history input remains retained");
                debug_assert!(selected.terminal_is_empty());
                drop(selected);
                *self.dictionary = Some(dictionary);
                self.phase = Phase::Dictionary;
                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 })
            }
            Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 }),
            Err(diagnostic) => {
                self.reject(diagnostic)
            }
        }
    }

    fn history_verification_limits()->Result<RetainedSprLimits,MemberOpenDiagnostic>{
        if F::OPEN_DECLARATIONS.is_empty(){return Err(MemberOpenDiagnostic::Identity)}
        F::OPEN_DECLARATIONS.iter().try_fold(RetainedSprLimits{file_bytes:0,frame_body_bytes:0,records:0},|envelope,declaration|{let original=declaration.history_limits;original.validate().map_err(|_|MemberOpenDiagnostic::Capacity)?;Ok(RetainedSprLimits{file_bytes:envelope.file_bytes.max(u64::from(original.file_bytes)),frame_body_bytes:envelope.frame_body_bytes.max(u64::from(original.frame_body_bytes)),records:envelope.records.max(u64::from(original.records))})})
    }

    fn step_original_history(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<MemberOpenStep<Box<ArtifactStore<P,M>>>,OriginalTurnFailure>{
        if let Err(diagnostic)=self.check_witness(cx){return Ok(self.reject(diagnostic))}
        let grant=semio_framework_os_store_member_open::member_step_grant(cx,grant);
        cx.set_stage("member-open.history.decode");
        if cx.should_yield(){return Ok(MemberOpenStep::Pending(self.replay_progress()))}
        if self.history_receiving.is_none(){
            let witness=self.witness.as_ref().unwrap();let end=usize::try_from(witness.verified_end()).map_err(|_|member_admission_diagnostic(MemberOpenDiagnostic::Capacity))?;
            *self.history_receiving=Some(MemberHistoryReceiving::new((0,end),witness.history_limits(),witness.maximum_history_allocation_bytes()));
            self.phase=Phase::DecodeHistory;
        }
        let source=self.witness.as_mut().unwrap().original_history_source(cx).map_err(member_admission_diagnostic)?;
        let step=self.history_receiving.as_mut().unwrap().step(&source,cx,grant,&mut self.decoded_history)?;
        match step{
            semio_framework_os_spr::history::RetainedHistorySourceStep::Fault=>Ok(self.reject(MemberOpenDiagnostic::Replay)),
            semio_framework_os_spr::history::RetainedHistorySourceStep::Ready if self.history_receiving.as_ref().unwrap().terminal_is_empty()=>{self.history_receiving.take();self.phase=Phase::BeginHydration;Ok(MemberOpenStep::Pending(self.replay_progress()))},
            _=>Ok(MemberOpenStep::Pending(self.replay_progress())),
        }
    }

    pub fn step_store(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<MemberOpenStep<Box<ArtifactStore<P,M>>>,OriginalTurnFailure>{self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;if matches!(self.phase,Phase::AdmitHistory|Phase::DecodeHistory){return self.step_original_history(cx,grant)}Ok(self.step_store_inner(cx,grant))}

    fn step_store_inner(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        let grant = semio_framework_os_store_member_open::member_step_grant(cx, grant);
        self.input_buffer_progress = Default::default();
        if let Some(diagnostic) = self.diagnostic {
            return MemberOpenStep::Rejected(diagnostic);
        }
        if let Some(owners) = self.owners.as_mut() {
            if !owners.constructor_is_complete() {
                if cx.should_yield() { return MemberOpenStep::Pending(self.replay_progress()); }
                let demand = match owners.constructor_demands(grant.maximum_copy_bytes) { Ok(demand) => demand, Err(error) => { *self.constructor_error=Some(error); return self.reject(MemberOpenDiagnostic::Capacity); } };
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth { return MemberOpenStep::Pending(self.replay_progress()); }
                let progress = match owners.admit_constructor(grant) { Ok(progress) => progress, Err((error,progress)) => { *self.constructor_error=Some(error); self.input_buffer_progress=progress; if let Err(diagnostic)=semio_framework_os_store_member_open::record_member_step(cx,progress){return self.reject(diagnostic)} return self.reject(MemberOpenDiagnostic::Capacity); } };
                self.input_buffer_progress=progress;
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                return MemberOpenStep::Pending(self.replay_progress());
            }
        }
        match self.drive_active_hydration_retirement(cx, grant) {
            Ok(true) => return MemberOpenStep::Pending(self.replay_progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        match self.phase {
            Phase::Snapshot => {
                let Some(depth) = grant.maximum_depth.checked_sub(1) else { return MemberOpenStep::Pending(self.replay_progress()); };
                let child = RetainedCloneGrant { maximum_depth: depth, ..grant };
                match self.snapshot_open.as_mut().unwrap().step(cx, child) {
                MemberSnapshotOpenStep::Pending(progress) => { if !progress.retained_progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); } MemberOpenStep::Pending(progress.opening) },
                MemberSnapshotOpenStep::Rejected(diagnostic) => self.reject(diagnostic),
                MemberSnapshotOpenStep::Ready => match self.snapshot_open.as_mut().unwrap().take_ready(cx) {
                    Some((snapshot, request)) => {
                        if self.snapshot_open.as_ref().unwrap().terminal_is_empty() { self.snapshot_open.take(); }
                        *self.snapshot = Some(snapshot);
                        *self.genesis_request = Some(request);
                        self.phase = Phase::CaptureGenesis;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 })
                    }
                    None => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: 0, total: 1 }),
                },
            } },
            Phase::CaptureGenesis => {
                let request = self.genesis_request.as_mut().expect("genesis copy retains the admitted request");
                let frame = match request.step_input(cx) {
                    semio_framework_os_store_member_open::MemberOpenInputStep::Framed(frame) => frame,
                    semio_framework_os_store_member_open::MemberOpenInputStep::Pending(progress) => return MemberOpenStep::Pending(progress),
                    semio_framework_os_store_member_open::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
                };
                let total = frame.snapshot_range().1;
                if self.genesis_pack.is_none() {
                    match admit_member_input_buffer(&mut self.genesis_pack, total, grant) {
                        Ok(Some(progress)) => { self.input_buffer_progress = progress; if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); } }
                        Ok(None) => {}
                        Err(_) => return self.reject(MemberOpenDiagnostic::Capacity),
                    }
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: 0, total: total as u64 });
                }
                let offset = self.genesis_pack.as_ref().expect("genesis copy retains bytes").len();
                if offset < total {
                    if grant.maximum_items == 0 || grant.maximum_depth == 0 { return MemberOpenStep::Pending(self.replay_progress()); }
                    let maximum = (total-offset).min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(grant.maximum_copy_bytes / 2).min((cx.fuel_remaining().min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES as u64 * 2) / 2) as usize);
                    if maximum == 0 { return MemberOpenStep::Pending(self.replay_progress()); }
                    let copied = match request.copy_snapshot_chunk(offset, &mut self.history_page.as_deref_mut().expect("live member history page")[..maximum], cx) { Ok(copied) if copied != 0 => copied, Ok(_) => return MemberOpenStep::Pending(self.replay_progress()), Err(error) => return self.reject(error) };
                    self.genesis_hasher.update(&self.history_page.as_deref().expect("live member history page")[..copied]);
                    self.genesis_pack.as_mut().expect("genesis copy retains bytes").extend_from_slice(&self.history_page.as_deref().expect("live member history page")[..copied]);
                    self.input_buffer_progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied * 2, ..Default::default() };
                    if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, self.input_buffer_progress) { return self.reject(diagnostic); }
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: (offset+copied) as u64, total: total as u64 });
                }
                let request = self.genesis_request.take().expect("genesis copy retains request authority");
                let limits=match Self::history_verification_limits(){Ok(limits)=>limits,Err(diagnostic)=>{*self.genesis_request=Some(request);return self.reject(diagnostic)}};
                match MemberHistoryVerification::new(request, limits) {
                    Ok(history) => *self.history = Some(history),
                    Err(rejected) => { *self.genesis_request = Some(rejected.request); return self.reject(rejected.diagnostic); },
                }
                self.phase = Phase::History;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 })
            }
            Phase::History => match self.history.as_mut().unwrap().step(cx) {
                MemberHistoryInputStep::Pending(progress) => MemberOpenStep::Pending(progress),
                MemberHistoryInputStep::Rejected(diagnostic) => self.reject(diagnostic),
                MemberHistoryInputStep::Ready => match self.history.as_mut().unwrap().take_ready(cx) {
                    Ok(Some(input)) => match MemberFactorySelection::<F>::begin(input, cx) {
                        Ok(selection) => {
                            self.history.take();
                            *self.selection = Some(selection);
                            self.phase = Phase::Select;
                            MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Validate, completed: 0, total: F::OPEN_DECLARATIONS.len() as u64 })
                        }
                        Err(rejected) => {
                            self.history.take();
                            *self.rejected_history_input = Some(rejected.input);
                            self.reject(rejected.diagnostic)
                        }
                    },
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
            Phase::Select => match self.selection.as_mut().unwrap().step(cx) {
                MemberFactorySelectionStep::Pending(progress) => MemberOpenStep::Pending(progress),
                MemberFactorySelectionStep::Rejected(diagnostic) => self.reject(diagnostic),
                MemberFactorySelectionStep::Ready => match self.selection.as_mut().unwrap().take_ready(cx) {
                    Ok(Some(selected)) => {
                        self.selection.take();
                        *self.selected = Some(selected);
                        self.phase = Phase::Selected;
                        self.begin_selected_dictionary(cx)
                    }
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Validate, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
            Phase::Selected => self.begin_selected_dictionary(cx),
            Phase::Dictionary => match self.dictionary.as_mut().unwrap().step(cx) {
                MemberHistoryDictionaryStep::Pending(progress) => MemberOpenStep::Pending(progress),
                MemberHistoryDictionaryStep::Rejected(diagnostic) => self.reject(diagnostic),
                MemberHistoryDictionaryStep::Ready => match self.dictionary.as_mut().unwrap().take_ready(cx) {
                    Ok(Some(witness)) => {
                        self.dictionary.take();
                        *self.witness = Some(witness);
                        self.phase = Phase::AdmitHistory;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: 1 })
                    }
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
            Phase::AdmitHistory | Phase::DecodeHistory => unreachable!("original History phase is received before other Member work"),
            Phase::BeginHydration => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.history.begin-hydration");
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let (expected, owner, schema, actor) = match self.witness.as_mut().unwrap().clone_initial_identity(cx) {
                    Ok(identity) => identity,
                    Err(diagnostic) => return self.reject(diagnostic),
                };
                let snapshot = match self.snapshot.take() {
                    Some(snapshot) => snapshot,
                    None => return self.reject(MemberOpenDiagnostic::Stale),
                };
                let history = self.decoded_history.take().expect("decoded history remains retained");
                *self.hydration = Some(semio_framework_os_store_document::persisted_document_hydration::RetainedPersistedDocumentHydration::from_decoded_pack(
                    snapshot,
                    self.genesis_pack.take().expect("admitted genesis Pack remains retained"),
                    *self.genesis_hasher.finalize().as_bytes(),
                    history,
                    expected,
                    owner,
                    schema.to_string(),
                    self.owners.take().expect("original admitted member owner catalog remains retained"),
                    self.operation,
                    self.generation,
                    self.expires_at_us,
                    semio_framework_os_store_document::persisted_document_hydration::PersistedDocumentHydrationTarget::Store { generation: 0 },
                    actor,
                ));
                self.phase = Phase::Hydrate;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::Hydrate => {
                cx.set_stage("member-open.history.hydrate");
                let hydration = self.hydration.as_mut().expect("member persisted hydration remains retained");
                match hydration.step_store(cx, grant) {
                    semio_framework_os_store_document::persisted_document_hydration::PersistedDocumentStoreHydrationStep::Pending(progress) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: progress.completed, total: progress.total }),
                    semio_framework_os_store_document::persisted_document_hydration::PersistedDocumentStoreHydrationStep::Rejected(diagnostic) => self.reject(diagnostic),
                    semio_framework_os_store_document::persisted_document_hydration::PersistedDocumentStoreHydrationStep::Ready(member) => {
                        let hydration = self.hydration.take().expect("terminal member persisted hydration remains present");
                        assert!(hydration.terminal_is_empty());
                        drop(hydration);
                        *self.member = Some(member);
                        self.phase = Phase::RetireInput;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 })
                    }
                }
            }
            Phase::RetireInput => {
                if let Err(diagnostic) = self.check_authority(cx) { return self.reject(diagnostic); }
                cx.set_stage("member-open.retire-input");
                if cx.should_yield() { return MemberOpenStep::Pending(self.replay_progress()); }
                let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
                let step = match semio_framework_os_store_read::retirement_owner_close(&mut self.witness, child) {
                    Ok(step) => step,
                    Err(_) => return self.reject(MemberOpenDiagnostic::Initialization),
                };
                let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                if self.witness.is_none() {
                    self.phase = Phase::Ready;
                    MemberOpenStep::Ready(self.member.take().expect("initialized member handoff remains exact"))
                } else {
                    MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 })
                }
            }
            Phase::Ready => MemberOpenStep::Rejected(MemberOpenDiagnostic::Stale),
            Phase::Rejected => MemberOpenStep::Rejected(self.diagnostic.unwrap_or(MemberOpenDiagnostic::Stale)),
        }
    }
}

impl<F, P, M, C> MemberOpenOperation for InitialMemberStoreOpen<F, P, M, C>
where
    F: MemberFactory + 'static,
    C: MemberStoreProducer<P,M>,
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    type Member = Box<ArtifactStore<P, M>>;
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.retirement_demands(body)}

    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<MemberOpenStep<Self::Member>,OriginalTurnFailure> {
        self.step_store(cx, grant)
    }

    fn close_step(&mut self,parent:&OriginalCallerSourceParent,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
        if self.ownership_is_empty()&&!self.history_page_due(){if let Some(loan)=self.source_loan.as_mut(){let step=loan.close_step(parent,cx)?;let progress=step.progress();if loan.terminal_is_empty(){self.source_loan.take();}return Ok(if self.source_loan.is_none(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})}return Ok(RetainedCloneStep::Complete(Default::default()))}
        self.source_loan.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.validate_context(cx)?;
        if cx.operation()!=self.operation||cx.generation()!=self.generation{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original member close changed operation identity").into())}
        let mut grant=cx.retained_grant();
        if !self.terminal_is_empty()&&grant.maximum_items>0&&grant.maximum_depth<self.next_depth_demand()?{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"initial member close exceeds original depth").into())}
        if cx.should_yield(){return Ok(RetainedCloneStep::Progress(Default::default()))}
        grant.maximum_copy_bytes=grant.maximum_copy_bytes.min(usize::try_from(cx.fuel_remaining().saturating_sub(1)).unwrap_or(usize::MAX));
        if let Some(receiving)=self.history_receiving.as_mut(){
            self.phase=Phase::Rejected;self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
            let source=self.witness.as_ref().ok_or_else(||member_admission_diagnostic(MemberOpenDiagnostic::Stale))?.original_history_close_source(cx).map_err(member_admission_diagnostic)?;
            let step=receiving.close_step(&source,cx,grant)?;
            if receiving.terminal_is_empty(){self.history_receiving.take();}
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let result=self.close_owned_field(grant).map(|step|match step{RetainedCloneStep::Complete(progress)if !self.ownership_is_empty()||self.source_loan.is_some()=>RetainedCloneStep::Progress(progress),step=>step});
        semio_framework_os_store_member_open::receive_member_close(cx,result)
    }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty()&&!self.history_page_due()&&self.source_loan.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
    fn terminal_drop_byte_demand(&self) -> Option<usize> { self.terminal_is_empty().then_some(0) }
}

impl<F, P, M, C> Drop for InitialMemberStoreOpen<F, P, M, C>
where
    F: MemberFactory + 'static,
    C: MemberStoreProducer<P,M>,
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    /// 💣️ A panic already unwinding through a live open must not become a double panic that aborts the
    /// whole process; the drop bomb still fires for every non-unwinding drop.
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty()&&self.source_loan.is_none()&&!self.history_page_due(), "member-open operation dropped before exact member handoff or bounded close");
    }
}



