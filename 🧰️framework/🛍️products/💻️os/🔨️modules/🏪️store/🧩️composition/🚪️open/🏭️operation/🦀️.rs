//! 🏭️ Closed request-owned member opening: one typed decoder, one verified history, one store handoff.

use super::history::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryStep};
use super::history::factory::{MemberFactorySelection, MemberFactorySelectionStep, SelectedMemberHistoryDictionary, SelectedMemberHistoryInput, SelectedVerifiedMemberHistory};
use super::history::{MemberHistoryInputStep, MemberHistoryVerification};
use super::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep, SnapshotRetirementStep};
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

impl<P: ArtifactPack + crate::os_store::retirement::RetireOwned> MemberSnapshotOpenOperation for PackMemberSnapshotOpen<P> {
    type Snapshot = P;

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
        drop(std::mem::take(&mut self.input));
        self.expected_bytes = None;
        self.terminal = true;
        Some((snapshot, request))
    }
}

impl<P: crate::os_store::retirement::RetireOwned> ErasedSnapshotRetirement for PackMemberSnapshotOpen<P> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    drop(self.active.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("pack member decoder snapshot retirement returned false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("pack member decoder snapshot retirement exceeded its exact grant".into()),
                step => Ok(step),
            };
        }
        if let Some(snapshot) = self.snapshot.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(snapshot));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(request) = self.request.as_mut() {
            return match request.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if request.terminal_is_empty() => {
                    drop(self.request.take());
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("pack member decoder input returned false terminal".into()),
                step => Ok(step),
            };
        }
        if !self.input.is_empty() {
            let released_bytes = maximum_bytes.min(self.input.len());
            self.input.truncate(self.input.len() - released_bytes);
            if self.input.is_empty() {
                drop(std::mem::take(&mut self.input));
                self.expected_bytes = None;
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        self.expected_bytes = None;
        self.terminal = true;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none() && self.input.is_empty() && self.expected_bytes.is_none()
    }
}

impl<P> Drop for PackMemberSnapshotOpen<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.request.is_none() && self.snapshot.is_none() && self.active.is_none()), "pack member decoder dropped before exact handoff or bounded retirement");
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
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    snapshot_open: ManuallyDrop<Option<P::SnapshotOpen>>,
    snapshot: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<MemberHistoryVerification>>,
    selection: ManuallyDrop<Option<MemberFactorySelection<F>>>,
    /// The selected history input awaiting its dictionary owner: kept across steps when the step budget
    /// is spent between selection and dictionary admission, instead of failing the open as stale.
    selected: ManuallyDrop<Option<SelectedMemberHistoryInput<F>>>,
    dictionary: ManuallyDrop<Option<SelectedMemberHistoryDictionary<F>>>,
    witness: ManuallyDrop<Option<SelectedVerifiedMemberHistory<F>>>,
    history_bytes: ManuallyDrop<Option<Vec<u8>>>,
    history_page: Box<[u8; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]>,
    history_decoder: ManuallyDrop<Option<crate::os_spr::RetainedHistoryDecode>>,
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
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + crate::os_schema_composition::ArtifactCompositionFields,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn ownership_is_empty(&self) -> bool {
        self.snapshot_open.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.selection.is_none()
            && self.selected.is_none()
            && self.dictionary.is_none()
            && self.witness.is_none()
            && self.history_bytes.is_none()
            && self.history_decoder.is_none()
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
            selected: ManuallyDrop::new(None),
            dictionary: ManuallyDrop::new(None),
            witness: ManuallyDrop::new(None),
            history_bytes: ManuallyDrop::new(None),
            history_page: Box::new([0; crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES]),
            history_decoder: ManuallyDrop::new(None),
            decoded_history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(None),
            member: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            operation,
            generation,
            expires_at_us,
            completed_history_bytes: 0,
            completed_history_records: 0,
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

    fn replay_progress(&self) -> MemberOpenProgress {
        let total = self.decoded_history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: total as u64 }
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
                *self.active = self.selected.take().map(|selected| Box::new(selected) as Box<dyn ErasedSnapshotRetirement>);
                self.reject(diagnostic)
            }
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
        close_field!(selected);
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
        // A panic already unwinding through a live open must not become a double panic that aborts the
        // whole process; the drop bomb still fires for every non-unwinding drop.
        assert!(
            std::thread::panicking() || self.ownership_is_empty(),
            "member-open operation dropped before exact member handoff or bounded close"
        );
    }
}
