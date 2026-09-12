//! 🏭️ Closed request-owned member opening: one typed decoder, one verified history, one store handoff.

use super::history::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryStep};
use super::history::factory::{MemberFactorySelection, MemberFactorySelectionStep, SelectedMemberHistoryDictionary, SelectedVerifiedMemberHistory};
use super::history::{MemberHistoryInputStep, MemberHistoryVerification};
use super::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep, SnapshotRetirementStep};
use crate::os_spr::format::retained::RetainedSprLimits;
use crate::os_store::{ArtifactPack, ArtifactStore, MemberFactory, MemberStoreOwner};
use crate::{CompositionPin, Edit, FromValue, Mutation, MutationDiff, OpBinary, OpText, ToValue};
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
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        let Some(request) = self.request.as_mut() else {
            return Ok(SnapshotRetirementStep::Complete);
        };
        match request.close_step(items, bytes)? {
            SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
                self.request.take();
                Ok(SnapshotRetirementStep::Complete)
            }
            SnapshotRetirementStep::Complete => Err("unsupported member decoder returned false terminal".into()),
            SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > items || released_bytes > bytes => Err("unsupported member decoder exceeded retirement grant".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.request.is_none()
    }
}

impl<P> Drop for UnsupportedMemberSnapshotOpen<P> {
    fn drop(&mut self) {
        assert!(self.request.is_none(), "unsupported member decoder dropped retained request authority");
    }
}

pub struct UnsupportedMemberFactoryOpen<M: Send> {
    snapshot: UnsupportedMemberSnapshotOpen<M>,
}

impl<M: Send> UnsupportedMemberFactoryOpen<M> {
    pub fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        Ok(Self { snapshot: UnsupportedMemberSnapshotOpen::begin(request)? })
    }
}

impl<M: Send> MemberOpenOperation for UnsupportedMemberFactoryOpen<M> {
    type Member = M;

    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<M> {
        match self.snapshot.step(cx) {
            MemberSnapshotOpenStep::Rejected(diagnostic) => MemberOpenStep::Rejected(diagnostic),
            _ => MemberOpenStep::Rejected(MemberOpenDiagnostic::Decode),
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.snapshot.close_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot.terminal_is_empty()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Snapshot,
    History,
    Select,
    Dictionary,
    CopyHistory,
    DecodeHistory,
    BeginHydration,
    Hydrate,
    BeginEdit,
    DecodeForward,
    DecodeInverse,
    DecodeMetadata,
    FinishEdit,
    HydrateChanges,
    HydrateCheckpoints,
    HydrateAlternatives,
    HydrateConflicts,
    HydratePins,
    Replay,
    SeedApplied,
    SeedRedo,
    RetireHistory,
    RetireHistoryBytes,
    Initialize,
    RetireInput,
    Ready,
    Rejected,
}

pub struct InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    snapshot_open: ManuallyDrop<Option<P::SnapshotOpen>>,
    snapshot: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<MemberHistoryVerification>>,
    selection: ManuallyDrop<Option<MemberFactorySelection<F>>>,
    dictionary: ManuallyDrop<Option<SelectedMemberHistoryDictionary<F>>>,
    witness: ManuallyDrop<Option<SelectedVerifiedMemberHistory<F>>>,
    history_bytes: ManuallyDrop<Option<Vec<u8>>>,
    history_page: Box<[u8; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]>,
    history_decoder: ManuallyDrop<Option<crate::os_spr::RetainedHistoryDecode>>,
    decoded_history: ManuallyDrop<Option<crate::os_spr::HistoryLog>>,
    hydration: ManuallyDrop<Option<crate::os_store::RetainedPersistedDocumentHydration<P, M>>>,
    envelope: ManuallyDrop<Option<crate::os_store::ArtifactEnvelope<P, M>>>,
    runtime: ManuallyDrop<Option<crate::os_store::ArtifactStoreInitializationRuntime<P>>>,
    owners: ManuallyDrop<Option<crate::os_store::DocumentStoreOwners<P, M>>>,
    pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pending_messages: ManuallyDrop<Option<crate::os_spr::EditMessages>>,
    member: ManuallyDrop<Option<Box<ArtifactStore<P, M>>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    operation: OperationId,
    generation: Generation,
    expires_at_us: u64,
    completed_history_bytes: u64,
    completed_history_records: u64,
    edit_index: usize,
    operation_index: usize,
    record_index: usize,
    pin_group_index: usize,
    pin_index: usize,
    phase: Phase,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl<F, P, M> InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn ownership_is_empty(&self) -> bool {
        self.snapshot_open.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.selection.is_none()
            && self.dictionary.is_none()
            && self.witness.is_none()
            && self.history_bytes.is_none()
            && self.history_decoder.is_none()
            && self.decoded_history.is_none()
            && self.hydration.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.member.is_none()
            && self.active.is_none()
    }
}

impl<F, P, M> InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        let operation = request.operation();
        let generation = request.generation();
        let expires_at_us = request.expires_at_us();
        let snapshot_open = P::SnapshotOpen::begin(request)?;
        Ok(Self {
            snapshot_open: ManuallyDrop::new(Some(snapshot_open)),
            snapshot: ManuallyDrop::new(None),
            history: ManuallyDrop::new(None),
            selection: ManuallyDrop::new(None),
            dictionary: ManuallyDrop::new(None),
            witness: ManuallyDrop::new(None),
            history_bytes: ManuallyDrop::new(None),
            history_page: Box::new([0; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]),
            history_decoder: ManuallyDrop::new(None),
            decoded_history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(None),
            pending_edit: ManuallyDrop::new(None),
            pending_messages: ManuallyDrop::new(None),
            member: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            operation,
            generation,
            expires_at_us,
            completed_history_bytes: 0,
            completed_history_records: 0,
            edit_index: 0,
            operation_index: 0,
            record_index: 0,
            pin_group_index: 0,
            pin_index: 0,
            phase: Phase::Snapshot,
            diagnostic: None,
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

    fn decode_operation(payload: &crate::os_spr::OpPayload) -> Result<M, MemberOpenDiagnostic> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => M::decode_op(bytes).map_err(|_| MemberOpenDiagnostic::Replay),
            (None, Some(text)) => M::parse_op(text).map_err(|_| MemberOpenDiagnostic::Replay),
            (None, None) => Err(MemberOpenDiagnostic::Malformed),
        }
    }

    fn replay_progress(&self) -> MemberOpenProgress {
        let total = self.decoded_history.as_ref().map_or(1, |history| {
            history.edits.len()
                + history.changes.len()
                + history.checkpoints.len()
                + history.alternatives.len()
                + history.conflicts.len()
                + 1
        });
        MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: self.record_index as u64, total: total as u64 }
    }

    fn drive_active_hydration_retirement(&mut self, cx: &mut StepContext<'_>) -> Result<bool, MemberOpenDiagnostic> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        if cx.should_yield() {
            return Ok(true);
        }
        let bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        match active.close_step(1, bytes) {
            Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= bytes => {
                cx.consume_fuel((released_items + released_bytes).max(1) as u64);
                Ok(true)
            }
            Ok(SnapshotRetirementStep::Complete) if active.terminal_is_empty() => {
                self.active.take();
                cx.consume_fuel(1);
                Ok(true)
            }
            _ => Err(MemberOpenDiagnostic::Initialization),
        }
    }

    pub fn step_store(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<Box<ArtifactStore<P, M>>> {
        if let Some(diagnostic) = self.diagnostic {
            return MemberOpenStep::Rejected(diagnostic);
        }
        match self.drive_active_hydration_retirement(cx) {
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
                        self.snapshot_open.take();
                        *self.snapshot = Some(snapshot);
                        match MemberHistoryVerification::new(request, RetainedSprLimits::default()) {
                            Ok(history) => *self.history = Some(history),
                            Err(rejected) => {
                                *self.active = Some(Box::new(rejected.request));
                                return self.reject(rejected.diagnostic);
                            }
                        }
                        self.phase = Phase::History;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 })
                    }
                    None => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Snapshot, completed: 0, total: 1 }),
                },
            },
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
                            *self.active = Some(Box::new(rejected.input));
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
                    Ok(Some(mut selected)) => {
                        self.selection.take();
                        match selected.begin_dictionary(MemberHistoryDictionaryLimits::default(), cx) {
                            Ok(Some(dictionary)) => {
                                *self.dictionary = Some(dictionary);
                                self.phase = Phase::Dictionary;
                                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 })
                            }
                            Ok(None) => {
                                *self.active = Some(Box::new(selected));
                                self.reject(MemberOpenDiagnostic::Stale)
                            }
                            Err(diagnostic) => {
                                *self.active = Some(Box::new(selected));
                                self.reject(diagnostic)
                            }
                        }
                    }
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Validate, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
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
                        *self.active = Some(crate::os_store::retirement::owned_retirement((history, auxiliary)));
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
                        *self.active = Some(crate::os_store::retirement::owned_retirement(auxiliary));
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
                let (expected, owner, schema) = match self.witness.as_mut().unwrap().clone_initial_identity(cx) {
                    Ok(identity) => identity,
                    Err(diagnostic) => return self.reject(diagnostic),
                };
                let snapshot = match self.snapshot.take() {
                    Some(snapshot) => snapshot,
                    None => return self.reject(MemberOpenDiagnostic::Stale),
                };
                let history = self.decoded_history.take().expect("decoded history remains retained");
                *self.hydration = Some(crate::os_store::RetainedPersistedDocumentHydration::from_initial(
                    snapshot,
                    history,
                    expected,
                    owner,
                    schema.to_string(),
                    P::member_store_owners(),
                    self.operation,
                    self.generation,
                    self.expires_at_us,
                    crate::os_store::PersistedDocumentHydrationTarget::Store { generation: 0 },
                ));
                self.phase = Phase::Hydrate;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::Hydrate => {
                cx.set_stage("member-open.history.hydrate");
                let hydration = self.hydration.as_mut().expect("member persisted hydration remains retained");
                match hydration.step(cx) {
                    crate::os_store::PersistedDocumentHydrationStep::Pending(progress) => MemberOpenStep::Pending(MemberOpenProgress {
                        phase: MemberOpenPhase::Replay,
                        completed: progress.completed,
                        total: progress.total,
                    }),
                    crate::os_store::PersistedDocumentHydrationStep::Rejected(diagnostic) => self.reject(diagnostic),
                    crate::os_store::PersistedDocumentHydrationStep::Ready(crate::os_store::PersistedDocumentHydrationOutput::Store(member)) => {
                        let hydration = self.hydration.take().expect("terminal member persisted hydration remains present");
                        assert!(hydration.terminal_is_empty());
                        drop(hydration);
                        *self.member = Some(member);
                        self.phase = Phase::RetireHistoryBytes;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 })
                    }
                    crate::os_store::PersistedDocumentHydrationStep::Ready(crate::os_store::PersistedDocumentHydrationOutput::Envelope(envelope)) => {
                        *self.owners = Some(P::member_store_owners());
                        *self.active = Some(self.owners.as_ref().expect("unexpected envelope handoff owner catalog remains retained").retire_decoded_envelope(envelope));
                        self.reject(MemberOpenDiagnostic::Initialization)
                    }
                }
            }
            Phase::BeginEdit => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let history = self.decoded_history.as_ref().expect("decoded history remains retained");
                let Some(source) = history.edits.get(self.edit_index) else {
                    self.record_index = self.edit_index;
                    self.operation_index = 0;
                    self.phase = Phase::HydrateChanges;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                };
                let edit = Edit {
                    id: source.id.clone(),
                    actor: source.actor.clone(),
                    forwards: Vec::with_capacity(source.ops.len()),
                    inverse: Vec::with_capacity(source.inverse.len()),
                    mutation_meta: Vec::with_capacity(source.meta.as_ref().map_or(0, Vec::len)),
                    description: source.description.clone(),
                    coalesce_key: source.coalesce_key.clone(),
                    sequence_number: self.edit_index as i32 + 1,
                    started_at: source.started_at.clone(),
                    finished_at: source.finished_at.clone(),
                };
                let runtime = self.runtime.as_mut().expect("member hydration runtime remains retained");
                if runtime.seed_mutation(crate::os_spr::MutationId(edit.id.clone())).is_err() {
                    *self.active = Some(self.owners.as_ref().expect("member hydration owners remain retained").retire_decoded_edit(edit));
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
                runtime.observe_sequence(self.edit_index as i32 + 1);
                *self.pending_messages = Some(crate::os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
                *self.pending_edit = Some(edit);
                self.operation_index = 0;
                self.phase = Phase::DecodeForward;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::DecodeForward | Phase::DecodeInverse => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let source = &self.decoded_history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
                let payloads = if self.phase == Phase::DecodeForward { &source.ops } else { &source.inverse };
                if let Some(payload) = payloads.get(self.operation_index) {
                    let operation = match Self::decode_operation(payload) {
                        Ok(operation) => operation,
                        Err(diagnostic) => return self.reject(diagnostic),
                    };
                    let edit = self.pending_edit.as_mut().expect("pending typed edit remains retained");
                    if self.phase == Phase::DecodeForward {
                        edit.forwards.push(operation);
                    } else {
                        edit.inverse.push(operation);
                    }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = if self.phase == Phase::DecodeForward { Phase::DecodeInverse } else { Phase::DecodeMetadata };
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::DecodeMetadata => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let source = &self.decoded_history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
                let metadata = source.meta.as_ref().ok_or(MemberOpenDiagnostic::Replay);
                let metadata = match metadata {
                    Ok(metadata) => metadata,
                    Err(diagnostic) => return self.reject(diagnostic),
                };
                if let Some(meta) = metadata.get(self.operation_index) {
                    let (meta, messages) = match super::decode_history_mutation_meta(meta.clone()) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending typed edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        if id.0 != edit_id && self.runtime.as_mut().expect("member hydration runtime remains retained").seed_mutation(id.clone()).is_err() {
                            return self.reject(MemberOpenDiagnostic::Replay);
                        }
                    }
                    self.runtime.as_mut().expect("member hydration runtime remains retained").observe_timestamp(meta.timestamp);
                    self.pending_edit.as_mut().expect("pending typed edit remains retained").mutation_meta.push(meta);
                    self.pending_messages.as_mut().expect("pending message owner remains retained").messages.extend(messages);
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::FinishEdit;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::FinishEdit => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let edit = self.pending_edit.take().expect("completed typed edit remains retained");
                if let Err(edit) = self.envelope.as_mut().expect("member envelope remains retained").vcs.edits.try_push(edit) {
                    *self.active = Some(self.owners.as_ref().expect("member hydration owners remain retained").retire_decoded_edit(edit));
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                let messages = self.pending_messages.take().expect("completed edit message owner remains retained");
                if !messages.messages.is_empty()
                    && self.envelope.as_mut().expect("member envelope remains retained").edit_messages.try_push(messages).is_err()
                {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                self.edit_index += 1;
                self.record_index += 1;
                self.phase = Phase::BeginEdit;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::HydrateChanges => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let history = self.decoded_history.as_ref().expect("decoded history remains retained");
                if let Some(change) = history.changes.get(self.operation_index) {
                    let change = crate::os_store::Change { id: change.id.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone(), saved_at: change.saved_at.clone() };
                    if self.envelope.as_mut().expect("member envelope remains retained").vcs.changes.try_push(change).is_err() {
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateCheckpoints;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::HydrateCheckpoints => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let history = self.decoded_history.as_ref().expect("decoded history remains retained");
                if let Some(checkpoint) = history.checkpoints.get(self.operation_index) {
                    let checkpoint = crate::os_store::Checkpoint {
                        id: checkpoint.id.clone(),
                        change_ids: checkpoint.change_ids.clone(),
                        parent_id: checkpoint.parent_id.clone(),
                        authors: checkpoint.authors.iter().map(|author| crate::os_store::Author { id: author.id.clone(), name: author.name.clone(), avatar: None }).collect(),
                        message: checkpoint.message.clone(),
                        timestamp: checkpoint.timestamp.clone(),
                        composition_pins: Vec::new(),
                    };
                    if self.envelope.as_mut().expect("member envelope remains retained").vcs.checkpoints.try_push(checkpoint).is_err() {
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateAlternatives;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::HydrateAlternatives => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let history = self.decoded_history.as_ref().expect("decoded history remains retained");
                if let Some(alternative) = history.alternatives.get(self.operation_index) {
                    let alternative = crate::os_store::Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() };
                    if self.envelope.as_mut().expect("member envelope remains retained").vcs.alternatives.try_push(alternative).is_err() {
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateConflicts;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::HydrateConflicts => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let history = self.decoded_history.as_ref().expect("decoded history remains retained");
                if let Some(conflict) = history.conflicts.get(self.operation_index) {
                    let conflict = match super::decode_history_conflict(conflict.clone()) {
                        Ok(conflict) => conflict,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    self.envelope.as_mut().expect("member envelope remains retained").conflicts.push(conflict);
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydratePins;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::HydratePins => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let Some(composition) = self.decoded_history.as_ref().expect("decoded history remains retained").composition.as_ref() else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::Replay;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                };
                let Some((checkpoint_id, pins)) = composition.checkpoint_pins.get(self.pin_group_index) else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::Replay;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                };
                let envelope = self.envelope.as_mut().expect("member envelope remains retained");
                let Some(checkpoint) = envelope.vcs.checkpoints.iter_mut().find(|checkpoint| checkpoint.id == *checkpoint_id) else {
                    return self.reject(MemberOpenDiagnostic::Replay);
                };
                if self.pin_index == 0 {
                    checkpoint.composition_pins.reserve(pins.len());
                }
                if let Some((child_uri, checkpoint_id)) = pins.get(self.pin_index) {
                    let child_ref = match crate::os_io::ArtifactRef::parse_uri(child_uri) {
                        Ok(reference) => reference,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    if checkpoint.composition_pins.iter().any(|pin| pin.child_ref == child_ref) {
                        return self.reject(MemberOpenDiagnostic::Replay);
                    }
                    checkpoint.composition_pins.push(CompositionPin { child_ref, checkpoint_id: checkpoint_id.clone() });
                    self.pin_index += 1;
                } else {
                    self.pin_group_index += 1;
                    self.pin_index = 0;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::Replay => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let cursor = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref()).expect("persisted member cursor remains retained");
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.record_index) {
                    let envelope = self.envelope.as_ref().expect("member envelope remains retained");
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return self.reject(MemberOpenDiagnostic::Replay) };
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let current = self.runtime.as_mut().and_then(crate::os_store::ArtifactStoreInitializationRuntime::current_mut).expect("member hydration current remains retained");
                        let next = match MutationDiff::apply(operation.diff(current).diff(), current) {
                            Ok(next) => next,
                            Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(current, next);
                        *self.active = Some(self.owners.as_ref().expect("member hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::SeedApplied;
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::SeedApplied | Phase::SeedRedo => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let envelope = self.envelope.as_ref().expect("member envelope remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted member cursor remains retained");
                let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.record_index) {
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *id) else { return self.reject(MemberOpenDiagnostic::Replay) };
                    let result = if self.phase == Phase::SeedApplied {
                        self.runtime.as_mut().expect("member hydration runtime remains retained").push_applied_edit(edit)
                    } else {
                        self.runtime.as_mut().expect("member hydration runtime remains retained").push_redo_edit(edit)
                    };
                    if result.is_err() {
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.record_index += 1;
                } else {
                    self.record_index = 0;
                    self.phase = if self.phase == Phase::SeedApplied { Phase::SeedRedo } else { Phase::RetireHistory };
                }
                cx.consume_fuel(1);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::RetireHistory => {
                if let Some(history) = self.decoded_history.take() {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(history));
                    self.phase = Phase::RetireHistoryBytes;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                self.phase = Phase::RetireHistoryBytes;
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::RetireHistoryBytes => {
                if cx.should_yield() {
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let bytes = self.history_bytes.as_mut().expect("copied history bytes remain retained");
                if bytes.is_empty() {
                    self.history_bytes.take();
                    self.phase = Phase::RetireInput;
                    cx.consume_fuel(1);
                    return MemberOpenStep::Pending(self.replay_progress());
                }
                let released = bytes.len().min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                bytes.truncate(bytes.len() - released);
                cx.consume_fuel(released.max(1) as u64);
                MemberOpenStep::Pending(self.replay_progress())
            }
            Phase::Initialize => {
                if let Err(diagnostic) = self.check_authority(cx) {
                    return self.reject(diagnostic);
                }
                if cx.should_yield() {
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Initialize, completed: 0, total: 1 });
                }
                let envelope = self.envelope.take().expect("hydrated member envelope remains retained");
                let mut runtime = self.runtime.take().expect("hydrated member runtime remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted member cursor remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                runtime.set_local_actor_id(cursor.applied_edit_ids.last().and_then(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id)).and_then(|edit| edit.actor.clone()));
                let owners = self.owners.take().expect("hydrated member owners remain retained");
                *self.member = Some(Box::new(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, owners)));
                self.phase = Phase::RetireInput;
                cx.consume_fuel(1);
                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Initialize, completed: 1, total: 1 })
            }
            Phase::RetireInput => {
                if let Err(diagnostic) = self.check_authority(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.retire-input");
                let bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
                if bytes == 0 {
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 });
                }
                let witness = self.witness.as_mut().unwrap();
                match witness.close_step(1, bytes) {
                    Ok(SnapshotRetirementStep::Complete) if witness.terminal_is_empty() => {
                        self.witness.take();
                        cx.consume_fuel(1);
                        self.phase = Phase::Ready;
                        MemberOpenStep::Ready(self.member.take().expect("initialized member handoff remains exact"))
                    }
                    Ok(SnapshotRetirementStep::Complete) => self.reject(MemberOpenDiagnostic::Initialization),
                    Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= bytes => {
                        cx.consume_fuel((released_items + released_bytes).max(1) as u64);
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Retire, completed: 0, total: 1 })
                    }
                    Ok(SnapshotRetirementStep::Pending { .. } | SnapshotRetirementStep::Blocked) | Err(_) => self.reject(MemberOpenDiagnostic::Initialization),
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
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.phase = Phase::Rejected;
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(items, bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    self.active.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open child returned false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > items || released_bytes > bytes => Err("member-open child exceeded retirement grant".into()),
                step => Ok(step),
            };
        }
        if let Some(hydration) = self.hydration.as_mut() {
            return match hydration.close_step(items.min(1), bytes)? {
                SnapshotRetirementStep::Complete if hydration.terminal_is_empty() => {
                    self.hydration.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open persisted hydration returned false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > items.min(1) || released_bytes > bytes => Err("member-open persisted hydration exceeded its close grant".into()),
                step => Ok(step),
            };
        }
        if let Some(member) = self.member.as_mut() {
            let step = crate::os_store::SpaceMember::close_owned_step(member.as_mut(), items, bytes)?;
            if matches!(step, SnapshotRetirementStep::Complete) && crate::os_store::SpaceMember::close_owned_terminal_is_empty(member.as_ref()) {
                self.member.take();
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let owners = self.owners.as_ref().ok_or("member-open runtime lost its owner catalog")?;
            return match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), items.min(1), bytes)? {
                SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    self.runtime.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("member-open runtime returned false terminal".into()),
                step => Ok(step),
            };
        }
        if let Some(edit) = self.pending_edit.take() {
            let owners = self.owners.as_ref().ok_or("member-open pending edit lost its owner catalog")?;
            *self.active = Some(owners.retire_decoded_edit(edit));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(messages) = self.pending_messages.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(messages));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(envelope) = self.envelope.take() {
            let owners = self.owners.as_ref().ok_or("member-open envelope lost its owner catalog")?;
            *self.active = Some(owners.retire_decoded_envelope(envelope));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.decoded_history.is_some() || self.history_decoder.is_some() {
            let history = self.decoded_history.take().or_else(|| self.history_decoder.as_mut().and_then(crate::os_spr::RetainedHistoryDecode::take_partial));
            let auxiliary = self.history_decoder.as_mut().map(crate::os_spr::RetainedHistoryDecode::take_auxiliary_owners);
            if let Some(decoder) = self.history_decoder.take() {
                if !decoder.terminal_is_empty() {
                    *self.history_decoder = Some(decoder);
                    return Err("member-open history decoder retained an untransferred owner".into());
                }
                drop(decoder);
            }
            *self.active = Some(crate::os_store::retirement::owned_retirement((history, auxiliary)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(history_bytes) = self.history_bytes.as_mut() {
            if history_bytes.is_empty() {
                self.history_bytes.take();
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if bytes == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released_bytes = history_bytes.len().min(bytes);
            history_bytes.truncate(history_bytes.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        macro_rules! close_field {
            ($field:ident) => {
                if let Some(owner) = self.$field.as_mut() {
                    let step = owner.close_step(items, bytes)?;
                    if matches!(step, SnapshotRetirementStep::Complete) && owner.terminal_is_empty() {
                        self.$field.take();
                        return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                    }
                    return Ok(step);
                }
            };
        }
        close_field!(witness);
        close_field!(dictionary);
        close_field!(selection);
        close_field!(history);
        close_field!(snapshot_open);
        if let Some(snapshot) = self.snapshot.take() {
            if self.owners.is_none() {
                *self.owners = Some(P::member_store_owners());
            }
            *self.active = Some(self.owners.as_ref().expect("member-open owner catalog remains retained").initial_snapshot_retirement.retire_owned(snapshot));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(owners) = self.owners.as_mut() {
            match owners.store_disposer.close_uninstalled_step(items.min(1))? {
                SnapshotRetirementStep::Complete if owners.store_disposer.uninstalled_terminal_is_empty() => {
                    self.owners.take();
                    return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
                }
                SnapshotRetirementStep::Complete => return Err("member-open uninstalled disposer returned false terminal".into()),
                step => return Ok(step),
            }
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.ownership_is_empty()
    }
}

impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    type Member = Box<ArtifactStore<P, M>>;

    fn step(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<Self::Member> {
        self.step_store(cx)
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        ErasedSnapshotRetirement::close_step(self, maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        ErasedSnapshotRetirement::terminal_is_empty(self)
    }
}

impl<F, P, M> Drop for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(
            self.ownership_is_empty(),
            "member-open operation dropped before exact member handoff or bounded close"
        );
    }
}
