//! 🏭️ Closed request-owned member opening: one typed decoder, one verified history, one store handoff.

use super::history::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryStep};
use super::history::factory::{MemberFactorySelection, MemberFactorySelectionStep, SelectedMemberHistoryDictionary, SelectedMemberHistoryInput, SelectedVerifiedMemberHistory};
use super::history::{MemberHistoryInputStep, MemberHistoryVerification};
use super::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
use crate::os_spr::format::retained::RetainedSprLimits;
use crate::os_store::{ArtifactPack, ArtifactStore, MemberFactory, MemberStoreOwner};
use crate::{FromValue, Mutation, OpBinary, OpText, ToValue};
use semio_framework_job::{Generation, OperationId, StepContext};
use std::{marker::PhantomData, mem::ManuallyDrop};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberSnapshotOpenStep {
    Pending(MemberOpenProgress),
    Ready,
    Rejected(MemberOpenDiagnostic),
}

pub trait MemberSnapshotOpenOperation: ErasedSnapshotRetirement {
    type Snapshot;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic>;
    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError>
    where
        Self: Sized;
    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep;
    fn take_ready(&mut self, cx: &mut StepContext<'_>) -> Option<(Self::Snapshot, MemberOpenRequest)>;
}

pub struct UnsupportedMemberSnapshotOpen<P> {
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    diagnostic: Option<MemberOpenDiagnostic>,
    marker: PhantomData<fn() -> P>,
}

impl<P: Send> MemberSnapshotOpenOperation for UnsupportedMemberSnapshotOpen<P> {
    type Snapshot = P;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> { request.admitted_expected().map(|_| 0) }

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self { request: ManuallyDrop::new(Some(request)), diagnostic: None, marker: PhantomData })
    }

    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep {
        let diagnostic = self.diagnostic.or_else(|| self.request.as_ref().and_then(|request| request.check_step_authority(cx).err())).unwrap_or(MemberOpenDiagnostic::Decode);
        self.diagnostic = Some(diagnostic);
        MemberSnapshotOpenStep::Rejected(diagnostic)
    }

    fn take_ready(&mut self, _cx: &mut StepContext<'_>) -> Option<(P, MemberOpenRequest)> {
        None
    }
}

impl<P: Send> ErasedSnapshotRetirement for UnsupportedMemberSnapshotOpen<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items != 0 && grant.maximum_depth >= self.next_depth_demand()? { self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled); }
        crate::os_store::artifact_retirement_owner_close(&mut self.request, grant)
    }
    fn terminal_is_empty(&self) -> bool { self.request.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.request, 0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.request, body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.request, 0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(crate::os_store::artifact_retirement_owner_demands(&self.request, 0)?.depth) }
}

impl<P> Drop for UnsupportedMemberSnapshotOpen<P> {
    fn drop(&mut self) {
        assert!(self.request.is_none(), "unsupported member decoder dropped retained request authority");
    }
}

/// 📦️ Retained member snapshot open over a member's own `ArtifactPack` codec — the framed snapshot
/// bytes are copied in bounded chunks into an exactly reserved buffer and decoded once through
/// `P::decode_pack`, the same whole-pack decode the parent's `RetainedPersistedDocumentHydration`
/// performs. The composition sibling of `UnsupportedMemberSnapshotOpen`: every member whose pack
/// codec is its generic value encoding (every `s.stdio.semio` subset but `flow`, which streams its own
/// binary protocol) opens through this instead of rejecting `Decode`.
pub struct PackMemberSnapshotOpen<P> {
    request: ManuallyDrop<Option<MemberOpenRequest>>,
    snapshot: ManuallyDrop<Option<P>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    input: Vec<u8>,
    expected_bytes: Option<usize>,
    diagnostic: Option<MemberOpenDiagnostic>,
    terminal: bool,
}

const PACK_MEMBER_SNAPSHOT_CHUNK_BYTES: usize = 4_096;

impl<P> PackMemberSnapshotOpen<P> {
    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberSnapshotOpenStep {
        self.diagnostic.get_or_insert(diagnostic);
        MemberSnapshotOpenStep::Rejected(self.diagnostic.unwrap_or(diagnostic))
    }
}

impl<P: ArtifactPack + semio_framework_value::retirement::RetireOwned> MemberSnapshotOpenOperation for PackMemberSnapshotOpen<P> {
    type Snapshot = P;
    fn begin_birth_bytes(request: &MemberOpenRequest) -> Result<usize, MemberOpenDiagnostic> { request.admitted_expected().map(|_| 0) }

    fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        if let Err(diagnostic) = request.admitted_expected() {
            return Err(MemberOpenAdmissionError { diagnostic, request });
        }
        Ok(Self { request: ManuallyDrop::new(Some(request)), snapshot: ManuallyDrop::new(None), active: ManuallyDrop::new(None), input: Vec::new(), expected_bytes: None, diagnostic: None, terminal: false })
    }

    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberSnapshotOpenStep {
        if let Some(diagnostic) = self.diagnostic {
            return MemberSnapshotOpenStep::Rejected(diagnostic);
        }
        if self.terminal {
            return MemberSnapshotOpenStep::Rejected(MemberOpenDiagnostic::Stale);
        }
        let frame = match self.request.as_mut().expect("pack member decoder retains its request").step_input(cx) {
            crate::os_store::MemberOpenInputStep::Framed(frame) => frame,
            crate::os_store::MemberOpenInputStep::Pending(progress) => return MemberSnapshotOpenStep::Pending(progress),
            crate::os_store::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
        };
        let expected_bytes = frame.snapshot_range().1;
        if self.expected_bytes.is_none() {
            if self.input.try_reserve_exact(expected_bytes).is_err() {
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.expected_bytes = Some(expected_bytes);
        }
        cx.set_stage("member-open.pack-snapshot");
        if self.input.len() < expected_bytes {
            let mut chunk = [0u8; PACK_MEMBER_SNAPSHOT_CHUNK_BYTES];
            let maximum = chunk.len().min(expected_bytes - self.input.len());
            let copied = match self.request.as_ref().expect("pack member decoder retains its request").copy_snapshot_chunk(self.input.len(), &mut chunk[..maximum], cx) {
                Ok(copied) => copied,
                Err(diagnostic) => return self.reject(diagnostic),
            };
            self.input.extend_from_slice(&chunk[..copied]);
            return MemberSnapshotOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: self.input.len() as u64, total: expected_bytes as u64 });
        }
        if let Err(diagnostic) = self.request.as_ref().expect("pack member decoder retains its request").check_step_authority(cx) {
            return self.reject(diagnostic);
        }
        if self.snapshot.is_none() {
            match P::decode_pack(&self.input) {
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

impl<P: semio_framework_value::retirement::RetireOwned> PackMemberSnapshotOpen<P> {
    fn demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return crate::os_store::artifact_retirement_box_demands(active, body); }
        if self.snapshot.is_some() { return Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<P>(), depth: 2, ..Default::default() }); }
        if self.request.is_some() { return crate::os_store::artifact_retirement_owner_demands(&self.request, body); }
        Ok(RetirementDemand { release_bytes: self.input.capacity(), depth: usize::from(!self.terminal), ..Default::default() })
    }
}
impl<P: semio_framework_value::retirement::RetireOwned> ErasedSnapshotRetirement for PackMemberSnapshotOpen<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "pack open retirement exceeds admitted depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return crate::os_store::artifact_retirement_box_close_step(&mut self.active, grant); }
        if let Some(snapshot) = self.snapshot.take() {
            return match semio_framework_value::retirement::admit_owned_retirement(snapshot, RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }) {
                Ok((owner, progress)) => { *self.active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, snapshot)) => { *self.snapshot = Some(snapshot); Err(error) }
            };
        }
        if self.request.is_some() { let step = crate::os_store::artifact_retirement_owner_close(&mut self.request, grant)?; return Ok(RetainedCloneStep::Progress(step.progress())); }
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

impl<P> Drop for PackMemberSnapshotOpen<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.capacity() == 0), "pack member decoder dropped before exact handoff or bounded retirement");
    }
}

pub struct UnsupportedMemberFactoryOpen<M: Send> {
    snapshot: UnsupportedMemberSnapshotOpen<M>,
}

impl<M: Send> UnsupportedMemberFactoryOpen<M> {
    pub fn begin(request: &mut Option<MemberOpenRequest>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<Self>, MemberOpenDiagnostic> {
        request.as_ref().ok_or(MemberOpenDiagnostic::Stale)?.admitted_expected()?;
        if grant.maximum_items == 0 { return Ok(None); }
        match UnsupportedMemberSnapshotOpen::begin(request.take().expect("funded inline member open retains its original request")) {
            Ok(snapshot) => Ok(Some(Self { snapshot })),
            Err(rejected) => { *request = Some(rejected.request); Err(rejected.diagnostic) }
        }
    }
}

impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
    type Member = M;

    fn step(&mut self, cx: &mut StepContext<'_>, _grant: RetainedCloneGrant) -> MemberOpenStep<M> {
        match self.snapshot.step(cx) {
            MemberSnapshotOpenStep::Rejected(diagnostic) => MemberOpenStep::Rejected(diagnostic),
            _ => MemberOpenStep::Rejected(MemberOpenDiagnostic::Decode),
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.snapshot.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_depth_demand()? { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "unsupported factory requires admitted child depth")); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = self.snapshot.close_step(child)?;
        admit_retained_clone_close(child, step, self.snapshot.terminal_is_empty(), "unsupported factory request")
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.terminal_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.snapshot.next_capacity_byte_demand(body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.snapshot.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.snapshot.terminal_is_empty() { return Ok(0); } self.snapshot.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "unsupported factory child depth overflow")) }
    fn terminal_drop_byte_demand(&self) -> Option<usize> { self.snapshot.terminal_is_empty().then_some(0) }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Snapshot,
    CaptureGenesis,
    History,
    Select,
    Selected,
    Dictionary,
    CopyHistory,
    DecodeHistory,
    BeginHydration,
    Hydrate,
    RetireHistoryBytes,
    RetireInput,
    Ready,
    Rejected,
}

pub struct InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    snapshot_open: ManuallyDrop<Option<P::SnapshotOpen>>,
    snapshot: ManuallyDrop<Option<P>>,
    genesis_request: ManuallyDrop<Option<MemberOpenRequest>>,
    genesis_pack: ManuallyDrop<Option<Vec<u8>>>,
    genesis_hasher: semio_framework_hash::Hasher,
    history: ManuallyDrop<Option<MemberHistoryVerification>>,
    selection: ManuallyDrop<Option<MemberFactorySelection<F>>>,
    /// The selected history input awaiting its dictionary owner: kept across steps when the step budget
    /// is spent between selection and dictionary admission, instead of failing the open as stale.
    selected: ManuallyDrop<Option<SelectedMemberHistoryInput<F>>>,
    rejected_history_input: ManuallyDrop<Option<super::history::VerifiedMemberHistoryInput>>,
    dictionary: ManuallyDrop<Option<SelectedMemberHistoryDictionary<F>>>,
    witness: ManuallyDrop<Option<SelectedVerifiedMemberHistory<F>>>,
    history_bytes: ManuallyDrop<Option<Vec<u8>>>,
    history_page: Box<[u8; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]>,
    history_decoder: ManuallyDrop<Option<crate::os_spr::RetainedHistoryDecode>>,
    history_auxiliary: ManuallyDrop<Option<(Vec<String>, Vec<String>)>>,
    decoded_history: ManuallyDrop<Option<crate::os_spr::HistoryLog>>,
    hydration: ManuallyDrop<Option<crate::os_store::RetainedPersistedDocumentHydration<P, M>>>,
    owners: ManuallyDrop<Option<crate::os_store::DocumentStoreOwners<P, M>>>,
    member: ManuallyDrop<Option<Box<ArtifactStore<P, M>>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    operation: OperationId,
    generation: Generation,
    expires_at_us: u64,
    completed_history_bytes: u64,
    completed_history_records: u64,
    phase: Phase,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl<F, P, M> InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
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
            && self.history_bytes.is_none()
            && self.history_decoder.is_none()
            && self.history_auxiliary.is_none()
            && self.decoded_history.is_none()
            && self.hydration.is_none()
            && self.owners.is_none()
            && self.member.is_none()
            && self.active.is_none()
    }
}

impl<F, P, M> InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return crate::os_store::artifact_retirement_box_demands(active, body); }
        if self.genesis_request.is_some() { return crate::os_store::artifact_retirement_owner_demands(&self.genesis_request, body); }
        if self.genesis_pack.is_some() { return crate::os_store::artifact_retirement_owned_birth_demands(&self.genesis_pack); }
        if self.hydration.is_some() { return crate::os_store::artifact_retirement_owner_demands(&self.hydration, body); }
        if let Some(member) = self.member.as_ref() {
            if crate::os_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()) { return Ok(RetirementDemand { release_bytes: std::mem::size_of_val(member.as_ref()), depth: 1, ..Default::default() }); }
            let mut demand = crate::os_store::SpaceMember::close_owned_demands(member.as_ref(), body)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "member Store depth overflow"))?;
            return Ok(demand);
        }
        if self.history_decoder.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.decoded_history.is_some() { return crate::os_store::artifact_retirement_owned_birth_demands(&self.decoded_history); }
        if self.history_auxiliary.is_some() { return crate::os_store::artifact_retirement_owned_birth_demands(&self.history_auxiliary); }
        if self.history_bytes.is_some() { return crate::os_store::artifact_retirement_owned_birth_demands(&self.history_bytes); }
        macro_rules! child { ($field:ident) => { if self.$field.is_some() { return crate::os_store::artifact_retirement_owner_demands(&self.$field, body); } }; }
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
        Ok(Default::default())
    }

    pub fn begin_birth_demand(request: &MemberOpenRequest) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, MemberOpenDiagnostic> {
        let source = P::member_store_owners_birth_demand().map_err(|_| MemberOpenDiagnostic::Capacity)?;
        let capacity_bytes = P::SnapshotOpen::begin_birth_bytes(request)?.checked_add(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES).and_then(|bytes| bytes.checked_add(source.capacity_bytes)).ok_or(MemberOpenDiagnostic::Capacity)?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: source.depth.max(1) })
    }

    pub fn begin(request: MemberOpenRequest, grant: RetainedCloneGrant) -> Result<Self, MemberOpenAdmissionError> {
        let operation = request.operation();
        let generation = request.generation();
        let expires_at_us = request.expires_at_us();
        let required = match Self::begin_birth_demand(&request) { Ok(required) => required, Err(diagnostic) => return Err(MemberOpenAdmissionError { request, diagnostic }) };
        if grant.maximum_items == 0 || grant.maximum_depth < required.depth || grant.maximum_capacity_bytes < required.capacity_bytes { return Err(MemberOpenAdmissionError { request, diagnostic: MemberOpenDiagnostic::Capacity }); }
        let source_capacity = match P::member_store_owners_birth_demand() { Ok(source) => source.capacity_bytes, Err(_) => return Err(MemberOpenAdmissionError { request, diagnostic: MemberOpenDiagnostic::Capacity }) };
        let source_grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: source_capacity, ..grant };
        let (owners, mut diagnostic) = match P::member_store_owners(source_grant) {
            Ok((owners, progress)) => {
                let diagnostic = if progress.fits(source_grant) && progress.retained_capacity_bytes == source_capacity { None } else { Some(MemberOpenDiagnostic::Capacity) };
                (Some(owners), diagnostic)
            }
            Err(refused) => (refused.owners, Some(MemberOpenDiagnostic::Capacity)),
        };
        let (snapshot_open, genesis_request) = if diagnostic.is_none() {
            match P::SnapshotOpen::begin(request) {
                Ok(snapshot_open) => (Some(snapshot_open), None),
                Err(rejected) => { diagnostic = Some(rejected.diagnostic); (None, Some(rejected.request)) }
            }
        } else { (None, Some(request)) };
        Ok(Self {
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
            history_bytes: ManuallyDrop::new(None),
            history_page: Box::new([0; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]),
            history_decoder: ManuallyDrop::new(None),
            history_auxiliary: ManuallyDrop::new(None),
            decoded_history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(owners),
            member: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            operation,
            generation,
            expires_at_us,
            completed_history_bytes: 0,
            completed_history_records: 0,
            phase: if diagnostic.is_some() { Phase::Rejected } else { Phase::Snapshot },
            diagnostic,
        })
    }

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        MemberOpenStep::Rejected(self.diagnostic.unwrap())
    }

    fn check_witness(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.witness.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx)
    }

    fn check_authority(&self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
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
        if self.active.is_none() && self.history_auxiliary.is_none() { return Ok(false); }
        if cx.should_yield() { return Ok(true); }
        let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        let step = if self.active.is_some() { crate::os_store::artifact_retirement_box_close_step(&mut self.active, child) } else { crate::os_store::artifact_retirement_admit_owned(&mut self.history_auxiliary, &mut self.active, child) }.map_err(|_| MemberOpenDiagnostic::Initialization)?;
        let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
        let fuel = progress.copied_items.checked_add(progress.copied_bytes).ok_or(MemberOpenDiagnostic::Capacity)?;
        cx.consume_fuel(u64::try_from(fuel).map_err(|_| MemberOpenDiagnostic::Capacity)?);
        Ok(true)
    }

    /// Admits the selected history input into its dictionary owner; a spent step budget or deadline
    /// keeps the selected input retained for the next step rather than rejecting the open.
    fn begin_selected_dictionary(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        let Some(selected) = self.selected.as_mut() else { return self.reject(MemberOpenDiagnostic::Stale) };
        match selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), cx) {
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

    pub fn step_store(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        if let Some(diagnostic) = self.diagnostic {
            return MemberOpenStep::Rejected(diagnostic);
        }
        if let Some(owners) = self.owners.as_mut() {
            if !owners.constructor_is_complete() {
                if cx.should_yield() { return MemberOpenStep::Pending(self.replay_progress()); }
                let demand = match owners.constructor_demands() { Ok(demand) => demand, Err(_) => return self.reject(MemberOpenDiagnostic::Capacity) };
                if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_depth < demand.depth { return MemberOpenStep::Pending(self.replay_progress()); }
                let progress = match owners.admit_constructor(grant) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Capacity) };
                let fuel = match progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()) { Some(fuel) => fuel, None => return self.reject(MemberOpenDiagnostic::Capacity) };
                cx.consume_fuel(fuel);
                return MemberOpenStep::Pending(self.replay_progress());
            }
        }
        match self.drive_active_hydration_retirement(cx, grant) {
            Ok(true) => return MemberOpenStep::Pending(self.replay_progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        match self.phase {
            Phase::Snapshot => match self.snapshot_open.as_mut().unwrap().step(cx) {
                MemberSnapshotOpenStep::Pending(progress) => MemberOpenStep::Pending(progress),
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
            },
            Phase::CaptureGenesis => {
                let request = self.genesis_request.as_mut().expect("genesis copy retains the admitted request");
                let frame = match request.step_input(cx) {
                    crate::os_store::MemberOpenInputStep::Framed(frame) => frame,
                    crate::os_store::MemberOpenInputStep::Pending(progress) => return MemberOpenStep::Pending(progress),
                    crate::os_store::MemberOpenInputStep::Rejected(diagnostic) => return self.reject(diagnostic),
                };
                let total = frame.snapshot_range().1;
                if self.genesis_pack.is_none() {
                    let mut pack = Vec::new();
                    if pack.try_reserve_exact(total).is_err() { return self.reject(MemberOpenDiagnostic::Capacity); }
                    *self.genesis_pack = Some(pack);
                }
                let offset = self.genesis_pack.as_ref().expect("genesis copy retains bytes").len();
                if offset < total {
                    let maximum = (total-offset).min(self.history_page.len());
                    let copied = match request.copy_snapshot_chunk(offset, &mut self.history_page[..maximum], cx) { Ok(copied) => copied, Err(error) => return self.reject(error) };
                    self.genesis_hasher.update(&self.history_page[..copied]);
                    self.genesis_pack.as_mut().expect("genesis copy retains bytes").extend_from_slice(&self.history_page[..copied]);
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: (offset+copied) as u64, total: total as u64 });
                }
                let request = self.genesis_request.take().expect("genesis copy retains request authority");
                match MemberHistoryVerification::new(request, RetainedSprLimits::default()) {
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
                        self.phase = Phase::CopyHistory;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: 1 })
                    }
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
            Phase::CopyHistory => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.history.copy");
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let total = match usize::try_from(self.witness.as_ref().unwrap().verified_end()) {
                    Ok(total) => total,
                    Err(_) => return self.reject(MemberOpenDiagnostic::Capacity),
                };
                if self.history_bytes.is_none() {
                    *self.history_bytes = Some(Vec::with_capacity(total));
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let offset = self.history_bytes.as_ref().unwrap().len();
                if offset == total {
                    let decoder = match crate::os_spr::RetainedHistoryDecode::new_persisted_document(total, RetainedSprLimits::default()) {
                        Ok(decoder) => decoder,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Capacity),
                    };
                    *self.history_decoder = Some(decoder);
                    self.phase = Phase::DecodeHistory;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let maximum = total.saturating_sub(offset).min(self.history_page.len());
                let copied = match self.witness.as_mut().unwrap().copy_verified_history_chunk(offset, &mut self.history_page[..maximum], cx) {
                    Ok(copied) if copied != 0 => copied,
                    Ok(_) => return MemberOpenStep::Pending(self.replay_progress()),
                    Err(diagnostic) => return self.reject(diagnostic),
                };
                self.history_bytes.as_mut().unwrap().extend_from_slice(&self.history_page[..copied]);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::DecodeHistory => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.history.decode");
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let bytes = self.history_bytes.as_ref().expect("copied history owner remains retained");
                let maximum_bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
                let step = match self.history_decoder.as_mut().expect("retained history decoder remains present").step(bytes, maximum_bytes, 1) {
                    Ok(step) => step,
                    Err(_) => {
                        let decoder = self.history_decoder.as_mut().expect("faulted retained history decoder remains present");
                        let history = decoder.take_partial();
                        let auxiliary = decoder.take_auxiliary_owners();
                        *self.decoded_history = history;
                        *self.history_auxiliary = Some(auxiliary);
                        let decoder = self.history_decoder.take().expect("terminal retained history decoder remains present");
                        assert!(decoder.terminal_is_empty());
                        drop(decoder);
                        return self.reject(MemberOpenDiagnostic::Replay);
                    }
                };
                match step {
                    crate::os_spr::RetainedHistoryDecodeStep::Pending { completed_bytes, decoded_records, .. } => {
                        let consumed = completed_bytes.saturating_sub(self.completed_history_bytes).saturating_add(decoded_records.saturating_sub(self.completed_history_records)).max(1);
                        self.completed_history_bytes = completed_bytes;
                        self.completed_history_records = decoded_records;
                        cx.consume_fuel(consumed.min(cx.fuel_remaining()));
                        MemberOpenStep::Pending(self.replay_progress())
                    }
                    crate::os_spr::RetainedHistoryDecodeStep::Ready => {
                        let decoder = self.history_decoder.as_mut().expect("ready retained history decoder remains present");
                        let history = decoder.take_ready().expect("ready history remains retained");
                        let auxiliary = decoder.take_auxiliary_owners();
                        *self.history_auxiliary = Some(auxiliary);
                        let decoder = self.history_decoder.take().expect("terminal retained history decoder remains present");
                        assert!(decoder.terminal_is_empty());
                        drop(decoder);
                        *self.decoded_history = Some(history);
                        self.phase = Phase::BeginHydration;
                        cx.consume_fuel(1);
                        MemberOpenStep::Pending(self.replay_progress())
                    }
                }
            }
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
                *self.hydration = Some(crate::os_store::RetainedPersistedDocumentHydration::from_decoded_pack(
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
                    crate::os_store::PersistedDocumentHydrationTarget::Store { generation: 0 },
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
                    crate::os_store::PersistedDocumentStoreHydrationStep::Pending(progress) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: progress.completed, total: progress.total }),
                    crate::os_store::PersistedDocumentStoreHydrationStep::Rejected(diagnostic) => self.reject(diagnostic),
                    crate::os_store::PersistedDocumentStoreHydrationStep::Ready(member) => {
                        let hydration = self.hydration.take().expect("terminal member persisted hydration remains present");
                        assert!(hydration.terminal_is_empty());
                        drop(hydration);
                        *self.member = Some(member);
                        self.phase = Phase::RetireHistoryBytes;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 })
                    }
                }
            }
            Phase::RetireHistoryBytes => {
                if cx.should_yield() { return MemberOpenStep::Pending(self.replay_progress()); }
                if self.history_bytes.is_some() {
                    let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
                    let step = match crate::os_store::artifact_retirement_admit_owned(&mut self.history_bytes, &mut self.active, child) {
                        Ok(step) => step,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Initialization),
                    };
                    let progress = step.progress();
                    let Some(fuel) = progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()) else { return self.reject(MemberOpenDiagnostic::Capacity); };
                    cx.consume_fuel(fuel);
                } else {
                    self.phase = Phase::RetireInput;
                }
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::RetireInput => {
                if let Err(diagnostic) = self.check_authority(cx) { return self.reject(diagnostic); }
                cx.set_stage("member-open.retire-input");
                if cx.should_yield() { return MemberOpenStep::Pending(self.replay_progress()); }
                let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
                let step = match crate::os_store::artifact_retirement_owner_close(&mut self.witness, child) {
                    Ok(step) => step,
                    Err(_) => return self.reject(MemberOpenDiagnostic::Initialization),
                };
                let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
                let Some(fuel) = progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()) else { return self.reject(MemberOpenDiagnostic::Capacity); };
                cx.consume_fuel(fuel);
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

impl<F, P, M> ErasedSnapshotRetirement for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "initial member close exceeds admitted depth")); }
        self.phase = Phase::Rejected;
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.active.is_some() { return crate::os_store::artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.genesis_request.is_some() { return crate::os_store::artifact_retirement_owner_close(&mut self.genesis_request, grant); }
        if self.genesis_pack.is_some() { return crate::os_store::artifact_retirement_admit_owned(&mut self.genesis_pack, &mut self.active, grant); }
        if self.hydration.is_some() { return crate::os_store::artifact_retirement_owner_close(&mut self.hydration, grant); }
        if let Some(member) = self.member.as_mut() {
            if crate::os_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()) {
                let extent = std::mem::size_of_val(member.as_ref());
                if grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(empty)); }
                self.member.take();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = crate::os_store::SpaceMember::close_owned_step(member.as_mut(), child)?;
            admit_retained_clone_close(child, step, crate::os_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()), "initial member Store child")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(decoder) = self.history_decoder.as_mut() {
            if self.decoded_history.is_none() { *self.decoded_history = decoder.take_partial(); }
            if self.history_auxiliary.is_none() { *self.history_auxiliary = Some(decoder.take_auxiliary_owners()); }
            if !decoder.terminal_is_empty() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "member history decoder retained an untransferred owner")); }
            self.history_decoder.take();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.decoded_history.is_some() { return crate::os_store::artifact_retirement_admit_owned(&mut self.decoded_history, &mut self.active, grant); }
        if self.history_auxiliary.is_some() { return crate::os_store::artifact_retirement_admit_owned(&mut self.history_auxiliary, &mut self.active, grant); }
        if self.history_bytes.is_some() { return crate::os_store::artifact_retirement_admit_owned(&mut self.history_bytes, &mut self.active, grant); }
        macro_rules! close_field {
            ($field:ident) => {
                if self.$field.is_some() { return crate::os_store::artifact_retirement_owner_close(&mut self.$field, grant); }
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
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.retirement_demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.retirement_demands(0)?.depth) }
}

impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    type Member = Box<ArtifactStore<P, M>>;

    fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> MemberOpenStep<Self::Member> {
        self.step_store(cx, grant)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        ErasedSnapshotRetirement::close_step(self, grant)
    }

    fn terminal_is_empty(&self) -> bool {
        ErasedSnapshotRetirement::terminal_is_empty(self)
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ErasedSnapshotRetirement::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { ErasedSnapshotRetirement::next_capacity_byte_demand(self, body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { ErasedSnapshotRetirement::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { ErasedSnapshotRetirement::next_depth_demand(self) }
    fn terminal_drop_byte_demand(&self) -> Option<usize> { self.ownership_is_empty().then_some(std::mem::size_of_val(self.history_page.as_ref())) }
}

impl<F, P, M> Drop for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + semio_framework_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    /// 💣️ A panic already unwinding through a live open must not become a double panic that aborts the
    /// whole process; the drop bomb still fires for every non-unwinding drop.
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "member-open operation dropped before exact member handoff or bounded close");
    }
}
