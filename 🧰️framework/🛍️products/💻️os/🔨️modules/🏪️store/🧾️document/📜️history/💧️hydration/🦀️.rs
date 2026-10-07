//! 💧️ Retained typed hydration of one persisted Pack and SPR history into an exact document owner.

use super::{ErasedSnapshotRetirement, MemberOpenDiagnostic, SnapshotRetirementStep, conflict_from_history_conflict, mutation_meta_from_history_op_meta};
use {semio_framework_artifact_reference::ArtifactRef};
use crate::os_store::{ArtifactEnvelope, ArtifactPack, ArtifactStore, ArtifactStoreInitializationRuntime, DocumentStoreOwners, EditReplay, OwnerRef, ReplayStep};
use crate::{CompositionPin, Edit, FromValue, Mutation, OpBinary, OpText, ToValue};
use semio_framework_job::{Generation, OperationId, StepContext};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistedDocumentHydrationTarget {
    Store { generation: u64 },
    Envelope,
}

pub enum PersistedDocumentHydrationOutput<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    Store(Box<ArtifactStore<P, M>>),
    Envelope(ArtifactEnvelope<P, M>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PersistedDocumentHydrationProgress {
    pub completed: u64,
    pub total: u64,
}

pub enum PersistedDocumentHydrationStep<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    Pending(PersistedDocumentHydrationProgress),
    Ready(PersistedDocumentHydrationOutput<P, M>),
    Rejected(MemberOpenDiagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    ScanPack,
    DecodePack,
    Begin,
    Fold,
    BindGenesis,
    BuildAppliedCursor,
    BuildRedoCursor,
    BeginEdit,
    DecodeForward,
    DecodeInverse,
    DecodeMetadata,
    FinishEdit,
    HydrateChanges,
    HydrateCheckpoints,
    HydrateAlternatives,
    HydratePins,
    PrepareReplay,
    ReplayCursor,
    AdoptInverse,
    AdoptMessages,
    SettleReplay,
    BeginTargets,
    DecodeTarget,
    TargetInputs,
    SeedApplied,
    SeedRedo,
    RetireHistory,
    RetirePack,
    CloseEnvelopeRuntime,
    CloseEnvelopeOwners,
    Finish,
    Rejected,
}

struct HydratedReplayEdits<'a, M> {
    edits: &'a crate::os_vcs::ArtifactHistoryLedger<Edit<M>>,
    positions: &'a std::collections::BTreeMap<String, usize>,
}

impl<M> crate::os_store::ReplayEdits<M> for HydratedReplayEdits<'_, M> {
    fn replay_edit(&self, id: &str) -> Option<&Edit<M>> {
        self.edits.get(*self.positions.get(id)?).filter(|edit| edit.id == id)
    }
}

pub struct RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    actor: ManuallyDrop<crate::os_spr::ActorId>,
    pack: ManuallyDrop<Option<Vec<u8>>>,
    initial: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<std::sync::Arc<crate::os_spr::HistoryLog>>>,
    fold_job: ManuallyDrop<Option<crate::os_spr::HistoryFoldJob<'static, crate::os_spr::history::RetainedHistoryFold>>>,
    normalized_transitions: ManuallyDrop<Option<Vec<crate::os_spr::MutationEnvelope>>>,
    normalized_conflicts: ManuallyDrop<Option<Vec<crate::os_spr::Conflict>>>,
    replay_ids: ManuallyDrop<Option<Vec<String>>>,
    replay_order: ManuallyDrop<Option<crate::os_vcs::HistoryPageStack<String>>>,
    replay_total: u32,
    pin_refs: ManuallyDrop<Option<std::collections::HashSet<ArtifactRef>>>,
    loaded_result: ManuallyDrop<Option<super::EditReplayResult<P, M>>>,
    edit_lookup: ManuallyDrop<Option<std::collections::BTreeMap<String, usize>>>,
    mutation_lookup: ManuallyDrop<Option<std::collections::BTreeMap<crate::os_spr::MutationId, (usize, usize)>>>,
    target_source: ManuallyDrop<Option<std::sync::Arc<Vec<crate::os_spr::MutationEnvelope>>>>,
    target_decoder: ManuallyDrop<Option<crate::os_spr::HistoryFoldJob<'static, crate::os_spr::HistoryTransition>>>,
    pending_target: ManuallyDrop<Option<crate::os_spr::TransitionSupersede>>,
    target_address: ManuallyDrop<Option<Vec<String>>>,
    fold_completed: u64,
    progress_high_water: std::cell::Cell<u64>,
    fold: ManuallyDrop<Option<crate::os_spr::HistoryFold>>,
    expected: ManuallyDrop<Option<ArtifactRef>>,
    owner: ManuallyDrop<Option<OwnerRef>>,
    schema: ManuallyDrop<Option<String>>,
    envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    replay: ManuallyDrop<Option<EditReplay<P, M>>>,
    owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pending_messages: ManuallyDrop<Option<crate::os_spr::EditMessages>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    operation: OperationId,
    generation: Generation,
    expires_at_us: u64,
    target: PersistedDocumentHydrationTarget,
    phase: Phase,
    edit_index: usize,
    operation_index: usize,
    record_index: usize,
    pin_group_index: usize,
    pin_index: usize,
    pack_scanned: usize,
    pack_hasher: semio_framework_hash::Hasher,
    pack_digest: Option<[u8; 32]>,
    diagnostic: Option<MemberOpenDiagnostic>,
    terminal: bool,
}

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn from_decoded_pack(
        initial: P,
        pack: Vec<u8>,
        digest: [u8; 32],
        history: crate::os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: crate::os_spr::ActorId,
    ) -> Self {
        let mut hydration = Self::new(Some(pack), Some(initial), history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::Begin, actor);
        hydration.pack_digest = Some(digest);
        hydration
    }

    pub fn from_pack(
        pack: Vec<u8>,
        history: crate::os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: crate::os_spr::ActorId,
    ) -> Self {
        Self::new(Some(pack), None, history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::ScanPack, actor)
    }

    fn new(
        pack: Option<Vec<u8>>,
        initial: Option<P>,
        history: crate::os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        phase: Phase,
        actor: crate::os_spr::ActorId,
    ) -> Self {
        Self {
            actor: ManuallyDrop::new(actor),
            pack: ManuallyDrop::new(pack),
            initial: ManuallyDrop::new(initial),
            history: ManuallyDrop::new(Some(std::sync::Arc::new(history))),
            fold_job: ManuallyDrop::new(None),
            normalized_transitions: ManuallyDrop::new(None),
            normalized_conflicts: ManuallyDrop::new(None),
            replay_ids: ManuallyDrop::new(None),
            replay_order: ManuallyDrop::new(Some(crate::os_vcs::HistoryPageStack::new())),
            replay_total: 0,
            pin_refs: ManuallyDrop::new(Some(std::collections::HashSet::new())),
            loaded_result: ManuallyDrop::new(None),
            edit_lookup: ManuallyDrop::new(Some(std::collections::BTreeMap::new())),
            mutation_lookup: ManuallyDrop::new(Some(std::collections::BTreeMap::new())),
            target_source: ManuallyDrop::new(None),
            target_decoder: ManuallyDrop::new(None),
            pending_target: ManuallyDrop::new(None),
            target_address: ManuallyDrop::new(None),
            fold_completed: 0,
            progress_high_water: std::cell::Cell::new(0),
            fold: ManuallyDrop::new(None),
            expected: ManuallyDrop::new(Some(expected)),
            owner: ManuallyDrop::new(owner),
            schema: ManuallyDrop::new(Some(schema)),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            replay: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(Some(owners)),
            pending_edit: ManuallyDrop::new(None),
            pending_messages: ManuallyDrop::new(None),
            active: ManuallyDrop::new(None),
            operation,
            generation,
            expires_at_us,
            target,
            phase,
            edit_index: 0,
            operation_index: 0,
            record_index: 0,
            pin_group_index: 0,
            pin_index: 0,
            pack_scanned: 0,
            pack_hasher: semio_framework_hash::Hasher::new(),
            pack_digest: None,
            diagnostic: None,
            terminal: false,
        }
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

    fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> PersistedDocumentHydrationStep<P, M> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap())
    }

    fn progress(&self) -> PersistedDocumentHydrationProgress {
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        let completed = self.progress_high_water.get().max(self.fold_completed.saturating_add(self.record_index as u64));
        self.progress_high_water.set(completed);
        PersistedDocumentHydrationProgress { completed, total: completed.max(self.fold_completed.saturating_add(total as u64)) }
    }

    fn drive_active(&mut self, cx: &mut StepContext<'_>) -> Result<bool, MemberOpenDiagnostic> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        if cx.should_yield() {
            return Ok(true);
        }
        let maximum_bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
        match active.close_step(1, maximum_bytes) {
            Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= maximum_bytes => {
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

    fn loaded_replay_retirement(&self, owners: &DocumentStoreOwners<P, M>) -> super::ArtifactHistoryReadRetirement<P, M> {
        super::ArtifactHistoryReadRetirement { preview: ManuallyDrop::new(None), replay: ManuallyDrop::new(None), loaded_replay: ManuallyDrop::new(None), finished: ManuallyDrop::new(None), active: ManuallyDrop::new(None), snapshots: owners.initial_snapshot_retirement.clone(), mutations: owners.mutation_retirement.clone() }
    }

    fn retire_edit(&mut self, edit: Edit<M>) {
        *self.active = Some(self.owners.as_ref().expect("hydration owner catalog remains retained").retire_decoded_edit(edit));
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        if let Some(diagnostic) = self.diagnostic {
            return PersistedDocumentHydrationStep::Rejected(diagnostic);
        }
        if let Err(diagnostic) = self.check_authority(cx) {
            return self.reject(diagnostic);
        }
        match self.drive_active(cx) {
            Ok(true) => return PersistedDocumentHydrationStep::Pending(self.progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if cx.should_yield() {
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        match self.phase {
            Phase::ScanPack => {
                let pack = self.pack.as_ref().expect("persisted Pack remains retained through scan");
                let remaining = pack.len().saturating_sub(self.pack_scanned);
                if remaining == 0 {
                    self.phase = Phase::DecodePack;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                let scanned = remaining.min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                if scanned == 0 {
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                self.pack_hasher.update(&self.pack.as_ref().expect("retained Pack remains present")[self.pack_scanned..self.pack_scanned + scanned]);
                self.pack_scanned += scanned;
                cx.consume_fuel(scanned as u64);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::DecodePack => {
                let initial = match P::decode_pack(self.pack.as_ref().expect("scanned Pack remains retained")) {
                    Ok(initial) => initial,
                    Err(_) => return self.reject(MemberOpenDiagnostic::Decode),
                };
                *self.initial = Some(initial);
                self.phase = Phase::Begin;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Begin => {
                if self.actor.0.trim().is_empty() { return self.reject(MemberOpenDiagnostic::Initialization); }
                let history = self.history.as_ref().expect("decoded history remains retained");
                let expected = self.expected.as_ref().expect("document identity remains retained");
                let schema = self.schema.as_ref().expect("document schema remains retained");
                let Some(composition) = history.composition.as_ref() else { return self.reject(MemberOpenDiagnostic::Identity) };
                let dialect_matches = composition.dialect.as_ref().is_some_and(|(artifact_kind, standard, subset)| artifact_kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset);
                let history_owner = match composition.owner.as_ref() {
                    Some((parent, slot, child_id)) => match ArtifactRef::parse_uri(parent) {
                        Ok(parent) => Some(OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() }),
                        Err(_) => return self.reject(MemberOpenDiagnostic::Owner),
                    },
                    None => None,
                };
                if history.doc_id != expected.artifact_id || history.schema != *schema || !dialect_matches || history_owner.as_ref() != self.owner.as_ref() {
                    return self.reject(MemberOpenDiagnostic::Identity);
                }
                if P::publish_native_snapshot().is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
                *self.fold_job = Some(crate::os_spr::HistoryLog::fold_job(history.clone(), crate::os_spr::HistoryShape::Document));
                self.phase = Phase::Fold;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Fold => {
                let maximum_bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
                let job = self.fold_job.as_mut().expect("history fold job remains retained");
                let result = job.step(1, maximum_bytes, &mut || cx.should_yield());
                self.fold_completed = job.completed();
                cx.consume_fuel(1);
                match result {
                    Ok(crate::os_spr::HistoryFoldJobStep::Pending { .. }) => {}
                    Ok(crate::os_spr::HistoryFoldJobStep::Ready((fold, transitions, replay_order, conflicts))) => {
                        self.fold_job.take();
                        *self.replay_ids = Some(replay_order);
                        *self.fold = Some(fold);
                        *self.normalized_transitions = Some(transitions);
                        *self.normalized_conflicts = Some(conflicts);
                        self.phase = Phase::BindGenesis;
                    }
                    _ => return self.reject(MemberOpenDiagnostic::Replay),
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BindGenesis => {
                let history = std::sync::Arc::get_mut(self.history.as_mut().expect("decoded history remains retained")).expect("fold aliases close before genesis binding");
                let fold = self.fold.as_mut().expect("derived history remains retained");
                let initial = self.initial.take().expect("typed initial snapshot remains retained");
                let initial_digest = self.pack_digest.unwrap_or_else(|| *self.pack_hasher.finalize().as_bytes());
                let pack = self.pack.take().expect("stored genesis Pack remains retained");
                let genesis = crate::os_store::ArtifactGenesis::from_verified_pack(initial, pack, initial_digest);
                let expected = self.expected.take().expect("document identity remains retained");
                let schema = self.schema.take().expect("document schema remains retained");
                let mut envelope = crate::os_store::create_document_envelope_from_genesis::<P, M>(&schema, &expected.artifact_id, genesis, None);
                envelope.dialect = Some(expected.dialect);
                envelope.owner = self.owner.take();
                envelope.active_alternative_id = fold.alternative.take();
                envelope.viewer_checkpoint_id = history.viewer_checkpoint.take();
                envelope.cursor = Some(crate::os_store::ArtifactCursor::new(Vec::new(), Vec::new(), fold.checkpoint.take()));
                envelope.transitions = self.normalized_transitions.take().expect("normalized transition owners remain retained");
                let supersessions = std::mem::take(&mut fold.supersessions);
                envelope.conflicts = self.normalized_conflicts.take().expect("normalized conflict owners remain retained");
                let current = envelope.vcs.genesis.share_snapshot();
                let mut runtime = ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, current, initial_digest, crate::os_spr::ActorId(std::mem::take(&mut self.actor.0)));
                let installed = runtime.set_supersessions(supersessions);
                *self.runtime = Some(runtime);
                *self.envelope = Some(envelope);
                if installed.is_err() {
                    return self.reject(MemberOpenDiagnostic::Initialization);
                }
                self.phase = Phase::BuildAppliedCursor;
                self.operation_index = 0;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BuildAppliedCursor | Phase::BuildRedoCursor => {
                let fold = self.fold.as_mut().expect("derived history remains retained");
                let applied = self.phase == Phase::BuildAppliedCursor;
                let ids = if applied { &mut fold.applied } else { &mut fold.redo };
                if let Some(id) = ids.get_mut(self.operation_index) {
                    let cursor = self.envelope.as_mut().expect("hydrated envelope remains retained").cursor.as_mut().expect("hydrated cursor remains retained");
                    if applied { cursor.applied_edit_ids.push(std::mem::take(id)); } else { cursor.redo_edit_ids.push(std::mem::take(id)); }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = if applied { Phase::BuildRedoCursor } else { Phase::BeginEdit };
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BeginEdit => {
                let history = self.history.as_ref().expect("decoded history remains retained");
                let Some(source) = history.edits.get(self.edit_index) else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateChanges;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                };
                if source.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
                let mut forwards = Vec::new();
                let mut inverse = Vec::new();
                let mut mutation_meta = Vec::new();
                if forwards.try_reserve_exact(source.ops.len()).is_err() || inverse.try_reserve_exact(source.inverse.len()).is_err() || mutation_meta.try_reserve_exact(source.meta.as_ref().map_or(0, Vec::len)).is_err() {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                let edit = Edit {
                    line: source.line.clone(),
                    id: source.id.clone(),
                    actor: source.actor.clone(),
                    forwards,
                    inverse: inverse.into(),
                    mutation_meta,
                    verb: source.verb.clone(),
                    sequence_number: self.edit_index as i32 + 1,
                    started_at: source.started_at.clone(),
                    finished_at: source.finished_at.clone(),
                };
                let runtime = self.runtime.as_mut().expect("hydration runtime remains retained");
                if runtime.seed_mutation(crate::os_spr::MutationId(edit.id.clone())).is_err() {
                    self.retire_edit(edit);
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
                runtime.observe_sequence(self.edit_index as i32 + 1);
                *self.pending_messages = Some(crate::os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
                *self.pending_edit = Some(edit);
                self.operation_index = 0;
                self.phase = Phase::DecodeForward;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::DecodeForward | Phase::DecodeInverse => {
                let source = &self.history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
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
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::DecodeMetadata => {
                let source = &self.history.as_ref().expect("decoded history remains retained").edits[self.edit_index];
                let Some(metadata) = source.meta.as_ref() else { return self.reject(MemberOpenDiagnostic::Replay) };
                if metadata.len() != source.ops.len() {
                    return self.reject(MemberOpenDiagnostic::Replay);
                }
                if let Some(meta) = metadata.get(self.operation_index) {
                    if meta.author_id.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(MemberOpenDiagnostic::Replay); }
                    let (meta, messages) = match mutation_meta_from_history_op_meta(meta.clone()) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending typed edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        self.mutation_lookup.as_mut().expect("hydrated mutation index remains retained").insert(id.clone(), (self.edit_index, self.operation_index));
                        if self.runtime.as_mut().expect("hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {
                            return self.reject(MemberOpenDiagnostic::Replay);
                        }
                    }
                    self.runtime.as_mut().expect("hydration runtime remains retained").observe_timestamp(meta.timestamp);
                    self.pending_edit.as_mut().expect("pending typed edit remains retained").mutation_meta.push(meta);
                    self.pending_messages.as_mut().expect("pending message owner remains retained").messages.extend(messages);
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::FinishEdit;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::FinishEdit => {
                let mut edit = self.pending_edit.take().expect("completed typed edit remains retained");
                let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                super::stamp_edit_semantics::<P, M>(&mut edit, &envelope.schema);
                self.edit_lookup.as_mut().expect("hydrated edit index remains retained").insert(edit.id.clone(), envelope.vcs.edits.len());
                if let Err(edit) = envelope.vcs.edits.try_push(edit) {
                    self.retire_edit(edit);
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                let messages = self.pending_messages.take().expect("completed message owner remains retained");
                if !messages.messages.is_empty() {
                    if let Err(messages) = self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages.admit(messages) {
                        *self.active = Some(Box::new(super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                }
                self.edit_index += 1;
                self.record_index += 1;
                self.phase = Phase::BeginEdit;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateChanges => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(change) = fold.changes.get_mut(self.operation_index) {
                    let change = crate::os_store::Change { id: std::mem::take(&mut change.id), edit_ids: std::mem::take(&mut change.edit_ids), description: change.description.take(), saved_at: std::mem::take(&mut change.saved_at) };
                    if let Err(change) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes.try_push(change) {
                        *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::change(change)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateCheckpoints;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateCheckpoints => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(source) = fold.checkpoints.get_mut(self.operation_index) {
                    let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                    if self.pin_index == 0 {
                        let checkpoint = crate::os_store::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.change_ids), parent_id: source.parent_id.take(), authors: Vec::with_capacity(source.authors.len()), message: source.message.take(), timestamp: std::mem::take(&mut source.timestamp), composition_pins: Vec::new() };
                        if let Err(checkpoint) = envelope.vcs.checkpoints.try_push(checkpoint) {
                            *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint)));
                            return self.reject(MemberOpenDiagnostic::Capacity);
                        }
                        self.pin_index = 1;
                    } else if let Some(author) = source.authors.get_mut(self.pin_index - 1) {
                        envelope.vcs.checkpoints.get_mut(self.operation_index).expect("hydrated checkpoint remains indexed").authors.push(crate::os_store::Author { id: std::mem::take(&mut author.id), name: std::mem::take(&mut author.name), avatar: author.avatar.take() });
                        self.pin_index += 1;
                    } else {
                        self.operation_index += 1;
                        self.pin_index = 0;
                    }
                } else {
                    self.operation_index = 0;
                    self.pin_index = 0;
                    self.phase = Phase::HydrateAlternatives;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateAlternatives => {
                let fold = self.fold.as_mut().expect("history fold remains retained");
                if let Some(alternative) = fold.alternatives.get_mut(self.operation_index) {
                    let alternative = crate::os_store::Alternative { id: std::mem::take(&mut alternative.id), name: std::mem::take(&mut alternative.name), checkpoint_ids: std::mem::take(&mut alternative.checkpoint_ids) };
                    if let Err(alternative) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives.try_push(alternative) {
                        *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::alternative(alternative)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydratePins;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydratePins => {
                let Some(source) = self.fold.as_ref().expect("history fold remains retained").checkpoints.get(self.pin_group_index) else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::PrepareReplay;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                };
                let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                let Some(checkpoint) = envelope.vcs.checkpoints.get_mut(self.pin_group_index) else {
                    return self.reject(MemberOpenDiagnostic::Replay);
                };
                if self.pin_index == 0 && checkpoint.composition_pins.try_reserve_exact(source.pins.len()).is_err() {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                if let Some(pin) = source.pins.get(self.pin_index) {
                    let child_ref = match ArtifactRef::parse_uri(&pin.child_uri) {
                        Ok(reference) => reference,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    if !self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained").insert(child_ref.clone()) {
                        return self.reject(MemberOpenDiagnostic::Replay);
                    }
                    checkpoint.composition_pins.push(CompositionPin { child_ref, checkpoint_id: pin.checkpoint_id.clone() });
                    self.pin_index += 1;
                } else {
                    let previous = std::mem::take(self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained"));
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(previous));
                    self.pin_group_index += 1;
                    self.pin_index = 0;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::PrepareReplay => {
                let ids = self.replay_ids.as_mut().expect("prepared replay identity owners remain retained");
                if let Some(id) = ids.get_mut(self.record_index) {
                    let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                    let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed hydrated edit remains retained");
                    self.replay_total = self.replay_total.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
                    self.replay_order.as_mut().expect("prepared replay order remains retained").push(std::mem::take(id));
                    self.record_index += 1;
                } else {
                    self.replay_ids.take();
                    let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                    let supersessions = std::mem::take(&mut *self.runtime.as_mut().expect("hydration runtime remains retained").supersessions);
                    match EditReplay::new_counted(super::ReplayMode::Report, envelope.vcs.genesis.share_snapshot(), self.replay_order.take().expect("prepared replay order remains retained"), 0, &envelope.schema, supersessions, self.replay_total) {
                        Ok(replay) => { *self.replay = Some(replay); self.phase = Phase::ReplayCursor; },
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    }
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::ReplayCursor => {
                let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                let replay = self.replay.as_mut().expect("hydration replay remains retained");
                let edits = HydratedReplayEdits { edits: &envelope.vcs.edits, positions: self.edit_lookup.as_ref().expect("hydrated replay index remains retained") };
                let stepped = replay.step(&edits, &mut || {
                    cx.consume_fuel(1);
                    cx.should_yield()
                });
                match stepped {
                    Ok(ReplayStep::Pending(_)) => return PersistedDocumentHydrationStep::Pending(self.progress()),
                    Ok(ReplayStep::Finished(_)) => {}
                    Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                }
                let replay = self.replay.take().expect("finished hydration replay remains retained");
                match replay.finish() { Ok(result) => *self.loaded_result = Some(result), Err(_) => return self.reject(MemberOpenDiagnostic::Replay) }
                self.phase = Phase::AdoptInverse;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::AdoptInverse => {
                let result = self.loaded_result.as_mut().expect("completed hydration replay remains retained");
                if let Some((id, inverse)) = result.rebased_inverse.last_mut() {
                    let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
                    let edit = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.edits.get_mut(position).expect("indexed hydrated edit remains retained");
                    let displaced = std::mem::replace(&mut edit.inverse, std::mem::take(inverse));
                    let (id, _) = result.rebased_inverse.pop().expect("adopted inverse identity remains retained");
                    *self.active = Some(Box::new(super::ArtifactStoreOperationRowsRetirement::new(displaced, self.owners.as_ref().expect("hydration owner catalog remains retained").mutation_retirement.clone()).with_identity(id)));
                } else { self.phase = Phase::AdoptMessages; }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::AdoptMessages => {
                let result = self.loaded_result.as_mut().expect("completed hydration replay remains retained");
                if let Some(entry) = result.replayed.pop() {
                    let ledger = &mut self.envelope.as_mut().expect("hydrated envelope remains retained").edit_messages;
                    if entry.messages.is_empty() {
                        let previous = ledger.remove_id(&entry.edit_id);
                        *self.active = Some(semio_framework_value::retirement::owned_retirement((entry, previous)));
                    } else if let Some(previous) = ledger.get_mut_by_id(&entry.edit_id) {
                        let previous = std::mem::replace(previous, entry);
                        *self.active = Some(Box::new(super::ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
                    } else if let Err(entry) = ledger.admit(entry) {
                        *self.active = Some(Box::new(super::ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                } else {
                    let runtime = self.runtime.as_mut().expect("hydration runtime remains retained");
                    runtime.set_supersessions(std::mem::take(&mut result.supersessions)).expect("replay returns the original effective-input owner");
                    let state = result.take_state().expect("completed replay retains its reached projection");
                    let factory = self.owners.as_ref().expect("hydration owner catalog remains retained").initial_snapshot_retirement.clone();
                    *self.active = Some(Box::new(super::ReturnedSnapshotReadRetirement::new(runtime.replace_current_alias(state), factory)));
                    self.phase = Phase::SettleReplay;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::SettleReplay => {
                if let Some(result) = self.loaded_result.take() {
                    let owners = self.owners.as_ref().expect("hydration owner catalog remains retained");
                    let mut retirement = self.loaded_replay_retirement(owners);
                    *retirement.finished = Some(result);
                    *self.active = Some(Box::new(retirement));
                } else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::BeginTargets;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::BeginTargets => {
                if self.target_source.is_none() {
                    *self.target_source = Some(std::sync::Arc::new(std::mem::take(&mut self.envelope.as_mut().expect("hydrated envelope remains retained").transitions)));
                    self.record_index = 0;
                }
                let source = self.target_source.as_ref().expect("immutable transition source remains retained");
                if let Some(transition) = source.get(self.record_index) {
                    let mut at = 0;
                    let kind = crate::os_spr::wire::read_varint_u64(&transition.diff.payload, &mut at);
                    if !transition.target.is_empty() || !matches!(kind, Ok(6)) { self.record_index += 1; }
                    else {
                        let source = source.clone();
                        let index = self.record_index;
                        *self.target_decoder = Some(crate::os_spr::HistoryFoldJob::new(move |control| async move { crate::os_spr::decode_history_transition_controlled(&source[index].diff.payload, &control).await }));
                        self.phase = Phase::DecodeTarget;
                    }
                } else {
                    let transitions = std::sync::Arc::into_inner(self.target_source.take().expect("immutable transition source remains retained")).expect("target decoder aliases close before transition handoff");
                    self.envelope.as_mut().expect("hydrated envelope remains retained").transitions = transitions;
                    self.record_index = 0;
                    self.phase = Phase::SeedApplied;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::DecodeTarget => {
                let bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
                let job = self.target_decoder.as_mut().expect("transition target decoder remains retained");
                match job.step(1, bytes, &mut || cx.should_yield()) {
                    Ok(crate::os_spr::HistoryFoldJobStep::Pending { .. }) => {},
                    Ok(crate::os_spr::HistoryFoldJobStep::Ready(crate::os_spr::HistoryTransition::Supersede(target))) => {
                        self.target_decoder.take();
                        *self.pending_target = Some(target);
                        self.operation_index = 0;
                        self.phase = Phase::TargetInputs;
                    }
                    _ => return self.reject(MemberOpenDiagnostic::Replay),
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::TargetInputs => {
                let target = self.pending_target.as_ref().expect("decoded target inputs remain retained");
                if let Some(input) = target.inputs.get(self.operation_index) {
                    let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                    let operation = self.mutation_lookup.as_ref().expect("hydrated mutation index remains retained").get(&input.target).and_then(|(edit, index)| envelope.vcs.edits.get(*edit).and_then(|edit| edit.forwards.get(*index)));
                    let original = operation.map(<M as Mutation<P>>::conflict_target).unwrap_or_default();
                    let replacement = operation.and_then(|operation| super::admit_replacement::<P, M>(operation, &input.replacement, &envelope.schema).ok().flatten());
                    let replacement_target = replacement.as_ref().map(<M as Mutation<P>>::conflict_target);
                    let mut written = match &replacement_target { Some(target) => super::common_address(&original, target), None => super::common_address(&original, &original) };
                    let previous = self.target_address.take();
                    let next = match &previous { Some(known) => super::common_address(known, &written), None => std::mem::take(&mut written) };
                    *self.target_address = Some(next);
                    let retired_strings = (original, replacement_target, previous, written);
                    if let Some(replacement) = replacement {
                        let owners = self.owners.as_ref().expect("hydration owner catalog remains retained");
                        *self.active = Some(Box::new(super::ArtifactStoreOperationRowsRetirement::new(vec![replacement], owners.mutation_retirement.clone()).with_strings(retired_strings)));
                    } else { *self.active = Some(semio_framework_value::retirement::owned_retirement(retired_strings)); }
                    self.operation_index += 1;
                } else {
                    let source = std::sync::Arc::get_mut(self.target_source.as_mut().expect("immutable transition source remains retained")).expect("decoder aliases close before target mutation");
                    source[self.record_index].target = self.target_address.take().unwrap_or_default();
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(self.pending_target.take().expect("finished decoded target remains retained")));
                    self.record_index += 1;
                    self.phase = Phase::BeginTargets;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::SeedApplied | Phase::SeedRedo => {
                let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
                let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.record_index) {
                    let Some(position) = self.edit_lookup.as_ref().expect("hydrated edit index remains retained").get(id).copied() else { return self.reject(MemberOpenDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed hydrated edit remains retained");
                    let ledger_key = envelope.vcs.edits.key_at(position).expect("indexed hydrated ledger key remains retained");
                    let result = if self.phase == Phase::SeedApplied {
                        self.runtime.as_mut().expect("hydration runtime remains retained").push_applied_edit(edit, ledger_key)
                    } else {
                        self.runtime.as_mut().expect("hydration runtime remains retained").push_redo_edit(edit, ledger_key)
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
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::RetireHistory => {
                if let Some(history) = self.history.take() {
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold aliases close before history retirement")));
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                if let Some(index) = self.mutation_lookup.take() {
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                if let Some(pins) = self.pin_refs.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(pins)); cx.consume_fuel(1); return PersistedDocumentHydrationStep::Pending(self.progress()); }
                if let Some(index) = self.edit_lookup.take() {
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                if let Some(fold) = self.fold.take() {
                    *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                self.phase = Phase::RetirePack;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::RetirePack => {
                if let Some(pack) = self.pack.as_mut() {
                    if pack.is_empty() {
                        self.pack.take();
                        cx.consume_fuel(1);
                        return PersistedDocumentHydrationStep::Pending(self.progress());
                    }
                    let released = pack.len().min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
                    if released == 0 {
                        return PersistedDocumentHydrationStep::Pending(self.progress());
                    }
                    pack.truncate(pack.len() - released);
                    cx.consume_fuel(released as u64);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                }
                self.phase = if self.target == PersistedDocumentHydrationTarget::Envelope { Phase::CloseEnvelopeRuntime } else { Phase::Finish };
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::CloseEnvelopeRuntime => {
                let runtime = self.runtime.as_mut().expect("envelope hydration runtime remains retained");
                let owners = self.owners.as_ref().expect("envelope hydration owner catalog remains retained");
                let maximum_bytes = usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(crate::os_store::OWNED_SCHEMA_DECODE_PAGE_BYTES);
                match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), 1, maximum_bytes) {
                    Ok(SnapshotRetirementStep::Complete) if runtime.terminal_is_empty() => {
                        self.runtime.take();
                        self.phase = Phase::CloseEnvelopeOwners;
                        cx.consume_fuel(1);
                    }
                    Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= maximum_bytes => {
                        cx.consume_fuel((released_items + released_bytes).max(1) as u64);
                    }
                    _ => return self.reject(MemberOpenDiagnostic::Initialization),
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::CloseEnvelopeOwners => {
                let owners = self.owners.as_mut().expect("envelope hydration owner catalog remains retained");
                match owners.store_disposer.close_uninstalled_step(1) {
                    Ok(SnapshotRetirementStep::Complete) if owners.store_disposer.uninstalled_terminal_is_empty() => {
                        self.owners.take();
                        let envelope = self.envelope.take().expect("hydrated envelope remains retained until owner-catalog close");
                        assert!(self.runtime.is_none(), "envelope hydration runtime closes before handoff");
                        self.terminal = true;
                        return PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(envelope));
                    }
                    Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes == 0 => cx.consume_fuel(released_items.max(1) as u64),
                    _ => return self.reject(MemberOpenDiagnostic::Initialization),
                }
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::Finish => {
                let envelope = self.envelope.take().expect("hydrated envelope remains retained");
                let output = match self.target {
                    PersistedDocumentHydrationTarget::Store { generation } => {
                        let mut runtime = self.runtime.take().expect("hydrated runtime remains retained");
                        let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
                        runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                        let owners = self.owners.take().expect("store hydration owners remain retained");
                        PersistedDocumentHydrationOutput::Store(Box::new(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, generation, owners)))
                    }
                    PersistedDocumentHydrationTarget::Envelope => unreachable!("envelope hydration hands off while closing its uninstalled owner catalog"),
                };
                self.terminal = true;
                PersistedDocumentHydrationStep::Ready(output)
            }
            Phase::Rejected => PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap_or(MemberOpenDiagnostic::Stale)),
        }
    }
}

impl<P, M> ErasedSnapshotRetirement for RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, semio_framework_value::ValueError> {
        self.phase = Phase::Rejected;
        self.diagnostic.get_or_insert(MemberOpenDiagnostic::Cancelled);
        if self.terminal {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if active.terminal_is_empty() => {
                    self.active.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "persisted hydration nested owner reported false terminal")),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => {
                    Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "persisted hydration nested owner exceeded close grant"))
                }
                step => Ok(step),
            };
        }
        if let Some(job) = self.fold_job.as_mut() {
            match job.close_step(1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if job.terminal_is_empty() => { self.fold_job.take(); }
                step => return Ok(step),
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(conflicts) = self.normalized_conflicts.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(conflicts)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(transitions) = self.normalized_transitions.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(transitions));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(job) = self.target_decoder.as_mut() {
            match job.close_step(1, maximum_bytes)? { SnapshotRetirementStep::Complete if job.terminal_is_empty() => { self.target_decoder.take(); }, step => return Ok(step) }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(target) = self.pending_target.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(target)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(address) = self.target_address.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(address)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(source) = self.target_source.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(source).expect("target decoder closes before its source"))); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(ids) = self.replay_ids.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(ids));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(order) = self.replay_order.as_mut() {
            if let Some(id) = order.pop() { *self.active = Some(semio_framework_value::retirement::owned_retirement(id)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
            if order.release_empty_page() { return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }); }
            self.replay_order.take();
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let owners = self.owners.as_ref().ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "persisted hydration lost its owner catalog"))?;
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), 1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    self.runtime.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "persisted hydration runtime reported false terminal")),
                step => Ok(step),
            };
        }
        if let Some(edit) = self.pending_edit.take() {
            *self.active = Some(owners.retire_decoded_edit(edit));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(messages) = self.pending_messages.take() {
            *self.active = Some(Box::new(super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.replay.is_some() || self.loaded_result.is_some() {
            let mut retirement = self.loaded_replay_retirement(owners);
            *retirement.loaded_replay = self.replay.take();
            *retirement.finished = self.loaded_result.take();
            *self.active = Some(Box::new(retirement));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(envelope) = self.envelope.take() {
            *self.active = Some(owners.retire_decoded_envelope(envelope));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(initial) = self.initial.take() {
            *self.active = Some(owners.initial_snapshot_retirement.retire_owned(initial));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(history) = self.history.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(std::sync::Arc::into_inner(history).expect("fold aliases close before history retirement")));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(index) = self.mutation_lookup.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(pins) = self.pin_refs.take() { *self.active = Some(semio_framework_value::retirement::owned_retirement(pins)); return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }); }
        if let Some(index) = self.edit_lookup.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(index));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(fold) = self.fold.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(fold));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(pack) = self.pack.as_mut() {
            if pack.is_empty() {
                self.pack.take();
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            if maximum_bytes == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            let released = pack.len().min(maximum_bytes);
            pack.truncate(pack.len() - released);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: released });
        }
        if let Some(expected) = self.expected.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(expected));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(owner));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if !self.actor.0.is_empty() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(std::mem::take(&mut self.actor.0)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(schema) = self.schema.take() {
            *self.active = Some(semio_framework_value::retirement::owned_retirement(schema));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let owners = self.owners.as_mut().expect("persisted hydration owner catalog remains retained");
        match owners.store_disposer.close_uninstalled_step(1)? {
            SnapshotRetirementStep::Complete if owners.store_disposer.uninstalled_terminal_is_empty() => {
                self.owners.take();
                self.terminal = true;
                Ok(SnapshotRetirementStep::Complete)
            }
            SnapshotRetirementStep::Complete => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "persisted hydration uninstalled disposer reported false terminal")),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.actor.0.is_empty()
            && self.pack.is_none()
            && self.initial.is_none()
            && self.replay.is_none()
            && self.loaded_result.is_none()
            && self.pin_refs.is_none()
            && self.edit_lookup.is_none()
            && self.mutation_lookup.is_none()
            && self.target_source.is_none()
            && self.target_decoder.is_none()
            && self.pending_target.is_none()
            && self.target_address.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.replay_ids.is_none()
            && self.replay_order.is_none()
            && self.normalized_transitions.is_none()
            && self.normalized_conflicts.is_none()
            && self.fold.is_none()
            && self.expected.is_none()
            && self.owner.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.active.is_none()
    }
}

impl<P, M> Drop for RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_unbounded(), "persisted document hydration reached Drop before exact handoff or bounded retirement");
    }
}

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn terminal_is_empty_unbounded(&self) -> bool {
        self.terminal
            && self.actor.0.is_empty()
            && self.pack.is_none()
            && self.initial.is_none()
            && self.replay.is_none()
            && self.loaded_result.is_none()
            && self.pin_refs.is_none()
            && self.edit_lookup.is_none()
            && self.mutation_lookup.is_none()
            && self.target_source.is_none()
            && self.target_decoder.is_none()
            && self.pending_target.is_none()
            && self.target_address.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.replay_ids.is_none()
            && self.replay_order.is_none()
            && self.normalized_transitions.is_none()
            && self.normalized_conflicts.is_none()
            && self.fold.is_none()
            && self.expected.is_none()
            && self.owner.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.active.is_none()
    }
}
