//! 🏭️ Closed request-owned member opening: one typed decoder, one verified history, one store handoff.

use super::history::dictionary::{MemberHistoryDictionaryLimits, MemberHistoryDictionaryStep};
use super::history::factory::{MemberFactorySelection, MemberFactorySelectionStep, SelectedMemberHistoryDictionary, SelectedVerifiedMemberHistory};
use super::history::{MemberHistoryInputStep, MemberHistoryVerification};
use super::{ErasedSnapshotRetirement, MemberOpenAdmissionError, MemberOpenDiagnostic, MemberOpenOperation, MemberOpenPhase, MemberOpenProgress, MemberOpenRequest, MemberOpenStep, SnapshotRetirementStep};
use crate::os_spr::format::retained::RetainedSprLimits;
use crate::os_store::{ArtifactPack, ArtifactStore, MemberFactory, MemberStoreOwner, SpaceMember};
use crate::{FromValue, Mutation, OpBinary, OpText, ToValue};
use semio_framework_job::StepContext;
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
    Replay,
    Initialize,
    RetireInput,
    Ready,
    Rejected,
}

pub struct InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M>,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    snapshot_open: ManuallyDrop<Option<P::SnapshotOpen>>,
    snapshot: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<MemberHistoryVerification>>,
    selection: ManuallyDrop<Option<MemberFactorySelection<F>>>,
    dictionary: ManuallyDrop<Option<SelectedMemberHistoryDictionary<F>>>,
    witness: ManuallyDrop<Option<SelectedVerifiedMemberHistory<F>>>,
    member: ManuallyDrop<Option<ArtifactStore<P, M>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    phase: Phase,
    diagnostic: Option<MemberOpenDiagnostic>,
}

impl<F, P, M> InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn begin(request: MemberOpenRequest) -> Result<Self, MemberOpenAdmissionError> {
        let snapshot_open = P::SnapshotOpen::begin(request)?;
        Ok(Self {
            snapshot_open: ManuallyDrop::new(Some(snapshot_open)),
            snapshot: ManuallyDrop::new(None),
            history: ManuallyDrop::new(None),
            selection: ManuallyDrop::new(None),
            dictionary: ManuallyDrop::new(None),
            witness: ManuallyDrop::new(None),
            member: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            phase: Phase::Snapshot,
            diagnostic: None,
        })
    }

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> MemberOpenStep<ArtifactStore<P, M>> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        MemberOpenStep::Rejected(self.diagnostic.unwrap())
    }

    fn check_witness(&mut self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        self.witness.as_mut().ok_or(MemberOpenDiagnostic::Stale)?.check_step_authority(cx)
    }

    pub fn step_store(&mut self, cx: &mut StepContext<'_>) -> MemberOpenStep<ArtifactStore<P, M>> {
        if let Some(diagnostic) = self.diagnostic {
            return MemberOpenStep::Rejected(diagnostic);
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
                        self.phase = Phase::Replay;
                        MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: 1 })
                    }
                    Ok(None) => MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::History, completed: 0, total: 1 }),
                    Err(diagnostic) => self.reject(diagnostic),
                },
            },
            Phase::Replay => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.history.initial");
                if cx.should_yield() {
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 0, total: 1 });
                }
                cx.consume_fuel(1);
                if !self.witness.as_ref().unwrap().initial_history_is_exact() {
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
                self.phase = Phase::Initialize;
                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Replay, completed: 1, total: 1 })
            }
            Phase::Initialize => {
                if let Err(diagnostic) = self.check_witness(cx) {
                    return self.reject(diagnostic);
                }
                cx.set_stage("member-open.initialize");
                if cx.should_yield() {
                    return MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Initialize, completed: 0, total: 1 });
                }
                let (expected, owner, schema) = match self.witness.as_mut().unwrap().clone_initial_identity(cx) {
                    Ok(identity) => identity,
                    Err(diagnostic) => return self.reject(diagnostic),
                };
                let snapshot = match self.snapshot.take() {
                    Some(snapshot) => snapshot,
                    None => return self.reject(MemberOpenDiagnostic::Stale),
                };
                cx.consume_fuel(1);
                let initial_digest = *semio_framework_hash::hash(&snapshot.encode_pack()).as_bytes();
                let mut envelope = crate::os_store::create_document_envelope::<P, M>(schema, &expected.artifact_id, snapshot, None);
                envelope.dialect = Some(expected.dialect);
                envelope.owner = owner;
                let current = envelope.vcs.initial_snapshot.clone();
                let runtime = crate::os_store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, current, initial_digest);
                *self.member = Some(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, 0, P::member_store_owners()));
                self.phase = Phase::RetireInput;
                MemberOpenStep::Pending(MemberOpenProgress { phase: MemberOpenPhase::Initialize, completed: 1, total: 1 })
            }
            Phase::RetireInput => {
                if let Err(diagnostic) = self.check_witness(cx) {
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
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + Send + Sync + 'static,
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
        if let Some(member) = self.member.as_mut() {
            let step = member.close_owned_step(items, bytes)?;
            if matches!(step, SnapshotRetirementStep::Complete) && member.close_owned_terminal_is_empty() {
                self.member.take();
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            return Ok(step);
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
            *self.active = Some(P::member_store_owners().initial_snapshot_retirement.retire_owned(snapshot));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.snapshot_open.is_none() && self.snapshot.is_none() && self.history.is_none() && self.selection.is_none() && self.dictionary.is_none() && self.witness.is_none() && self.member.is_none() && self.active.is_none()
    }
}

impl<F, P, M> MemberOpenOperation for InitialMemberStoreOpen<F, P, M>
where
    F: MemberFactory + 'static,
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M> + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    type Member = ArtifactStore<P, M>;

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
    P: Clone + ToValue + FromValue + ArtifactPack + MemberStoreOwner<M>,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(
            self.snapshot_open.is_none() && self.snapshot.is_none() && self.history.is_none() && self.selection.is_none() && self.dictionary.is_none() && self.witness.is_none() && self.member.is_none() && self.active.is_none(),
            "member-open operation dropped before exact member handoff or bounded close"
        );
    }
}
