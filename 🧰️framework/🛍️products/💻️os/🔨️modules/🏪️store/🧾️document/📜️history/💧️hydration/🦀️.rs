//! 💧️ Retained typed hydration of one persisted Pack and SPR history into an exact document owner.

use super::{conflict_from_history_conflict, mutation_meta_from_history_op_meta, ErasedSnapshotRetirement, MemberOpenDiagnostic, SnapshotRetirementStep};
use crate::os_io::ArtifactRef;
use crate::os_store::{
    ArtifactEnvelope, ArtifactPack, ArtifactStore, ArtifactStoreInitializationRuntime, DocumentStoreOwners, OwnerRef,
};
use crate::{CompositionPin, Edit, FromValue, Mutation, MutationDiff, OpBinary, OpText, ToValue};
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
    ValidateReplay,
    RetireValidation,
    ReplayCursor,
    SeedApplied,
    SeedRedo,
    RetireHistory,
    RetirePack,
    CloseEnvelopeRuntime,
    CloseEnvelopeOwners,
    Finish,
    Rejected,
}

pub struct RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    pack: ManuallyDrop<Option<Vec<u8>>>,
    initial: ManuallyDrop<Option<P>>,
    validation: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<crate::os_spr::HistoryLog>>,
    expected: ManuallyDrop<Option<ArtifactRef>>,
    owner: ManuallyDrop<Option<OwnerRef>>,
    schema: ManuallyDrop<Option<String>>,
    envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
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
    diagnostic: Option<MemberOpenDiagnostic>,
    terminal: bool,
}

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn from_initial(
        initial: P,
        history: crate::os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
    ) -> Self {
        Self::new(None, Some(initial), history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::Begin)
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
    ) -> Self {
        Self::new(Some(pack), None, history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::ScanPack)
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
    ) -> Self {
        Self {
            pack: ManuallyDrop::new(pack),
            initial: ManuallyDrop::new(initial),
            validation: ManuallyDrop::new(None),
            history: ManuallyDrop::new(Some(history)),
            expected: ManuallyDrop::new(Some(expected)),
            owner: ManuallyDrop::new(owner),
            schema: ManuallyDrop::new(Some(schema)),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
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
        let total = self.history.as_ref().map_or(1, |history| {
            history.edits.len()
                + history.changes.len()
                + history.checkpoints.len()
                + history.alternatives.len()
                + history.conflicts.len()
                + history.composition.as_ref().map_or(0, |composition| composition.checkpoint_pins.len())
                + 1
        });
        PersistedDocumentHydrationProgress { completed: self.record_index as u64, total: total as u64 }
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

    fn retire_edit(&mut self, edit: Edit<M>) {
        *self.active = Some(self.owners.as_ref().expect("hydration owner catalog remains retained").retire_decoded_edit(edit));
    }

    pub fn step(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
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
                let history = self.history.as_ref().expect("decoded history remains retained");
                let expected = self.expected.as_ref().expect("document identity remains retained");
                let schema = self.schema.as_ref().expect("document schema remains retained");
                let Some(composition) = history.composition.as_ref() else { return self.reject(MemberOpenDiagnostic::Identity) };
                let dialect_matches = composition.dialect.as_ref().is_some_and(|(artifact_kind, standard, subset)| {
                    artifact_kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset
                });
                let history_owner = match composition.owner.as_ref() {
                    Some((parent, slot, child_id)) => match ArtifactRef::parse_uri(parent) {
                        Ok(parent) => Some(OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() }),
                        Err(_) => return self.reject(MemberOpenDiagnostic::Owner),
                    },
                    None => None,
                };
                if history.doc_id != expected.artifact_id || history.schema != *schema || history.cursor.is_none() || !dialect_matches || history_owner.as_ref() != self.owner.as_ref() {
                    return self.reject(MemberOpenDiagnostic::Identity);
                }
                let initial = self.initial.take().expect("typed initial snapshot remains retained");
                let validation = initial.clone();
                let initial_digest = *semio_framework_hash::hash(&initial.encode_pack()).as_bytes();
                let expected = self.expected.take().expect("document identity remains retained");
                let schema = self.schema.take().expect("document schema remains retained");
                let mut envelope = crate::os_store::create_document_envelope::<P, M>(&schema, &expected.artifact_id, initial, None);
                envelope.dialect = Some(expected.dialect);
                self.owner.take();
                envelope.owner = history_owner;
                envelope.active_alternative_id = history.active_alternative_id.clone();
                envelope.cursor = history.cursor.as_ref().map(|cursor| crate::os_store::ArtifactCursor::new(cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone(), cursor.checkpoint_id.clone()));
                if envelope.conflicts.try_reserve_exact(history.conflicts.len()).is_err() {
                    *self.validation = Some(validation);
                    *self.envelope = Some(envelope);
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                let current = envelope.vcs.initial_snapshot.clone();
                *self.validation = Some(validation);
                *self.runtime = Some(ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, current, initial_digest));
                *self.envelope = Some(envelope);
                self.phase = Phase::BeginEdit;
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
                let mut forwards = Vec::new();
                let mut inverse = Vec::new();
                let mut mutation_meta = Vec::new();
                if forwards.try_reserve_exact(source.ops.len()).is_err()
                    || inverse.try_reserve_exact(source.inverse.len()).is_err()
                    || mutation_meta.try_reserve_exact(source.meta.as_ref().map_or(0, Vec::len)).is_err()
                {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                let edit = Edit {
                    id: source.id.clone(),
                    actor: source.actor.clone(),
                    forwards,
                    inverse,
                    mutation_meta,
                    description: source.description.clone(),
                    coalesce_key: source.coalesce_key.clone(),
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
                    let (meta, messages) = match mutation_meta_from_history_op_meta(meta.clone()) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending typed edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        if id.0 != edit_id && self.runtime.as_mut().expect("hydration runtime remains retained").seed_mutation(id.clone()).is_err() {
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
                let edit = self.pending_edit.take().expect("completed typed edit remains retained");
                if let Err(edit) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.edits.try_push(edit) {
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
                let history = self.history.as_ref().expect("decoded history remains retained");
                if let Some(change) = history.changes.get(self.operation_index) {
                    let change = crate::os_store::Change { id: change.id.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone(), saved_at: change.saved_at.clone() };
                    if let Err(change) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes.try_push(change) {
                        *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::change(change)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateCheckpoints;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateCheckpoints => {
                let history = self.history.as_ref().expect("decoded history remains retained");
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
                    if let Err(checkpoint) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.checkpoints.try_push(checkpoint) {
                        *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateAlternatives;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateAlternatives => {
                let history = self.history.as_ref().expect("decoded history remains retained");
                if let Some(alternative) = history.alternatives.get(self.operation_index) {
                    let alternative = crate::os_store::Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() };
                    if let Err(alternative) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives.try_push(alternative) {
                        *self.active = Some(Box::new(super::ArtifactStoreHistoryMetadataRetirement::alternative(alternative)));
                        return self.reject(MemberOpenDiagnostic::Capacity);
                    }
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydrateConflicts;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydrateConflicts => {
                let history = self.history.as_ref().expect("decoded history remains retained");
                if let Some(conflict) = history.conflicts.get(self.operation_index) {
                    let conflict = match conflict_from_history_conflict(conflict.clone()) {
                        Ok(conflict) => conflict,
                        Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                    };
                    self.envelope.as_mut().expect("hydrated envelope remains retained").conflicts.push(conflict);
                    self.operation_index += 1;
                    self.record_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = Phase::HydratePins;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::HydratePins => {
                let Some(composition) = self.history.as_ref().expect("decoded history remains retained").composition.as_ref() else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::ValidateReplay;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                };
                let Some((checkpoint_id, pins)) = composition.checkpoint_pins.get(self.pin_group_index) else {
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::ValidateReplay;
                    cx.consume_fuel(1);
                    return PersistedDocumentHydrationStep::Pending(self.progress());
                };
                let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
                let Some(checkpoint) = envelope.vcs.checkpoints.iter_mut().find(|checkpoint| checkpoint.id == *checkpoint_id) else {
                    return self.reject(MemberOpenDiagnostic::Replay);
                };
                if self.pin_index == 0 && checkpoint.composition_pins.try_reserve_exact(pins.len()).is_err() {
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                if let Some((child_uri, checkpoint_id)) = pins.get(self.pin_index) {
                    let child_ref = match ArtifactRef::parse_uri(child_uri) {
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
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::ValidateReplay => {
                let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                if let Some(edit) = envelope.vcs.edits.get(self.record_index) {
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let validation = self.validation.as_mut().expect("validation projection remains retained");
                        let next = match MutationDiff::apply(operation.diff(validation).diff(), validation) {
                            Ok(next) => next,
                            Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(validation, next);
                        *self.active = Some(self.owners.as_ref().expect("hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    let validation = self.validation.take().expect("completed validation projection remains retained");
                    *self.active = Some(self.owners.as_ref().expect("hydration owners remain retained").initial_snapshot_retirement.retire_owned(validation));
                    self.phase = Phase::RetireValidation;
                }
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::RetireValidation => {
                self.record_index = 0;
                self.operation_index = 0;
                self.phase = Phase::ReplayCursor;
                cx.consume_fuel(1);
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::ReplayCursor => {
                let cursor = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref()).expect("persisted cursor remains retained");
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.record_index) {
                    let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *edit_id) else { return self.reject(MemberOpenDiagnostic::Replay) };
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let current = self.runtime.as_mut().and_then(ArtifactStoreInitializationRuntime::current_mut).expect("hydrated current remains retained");
                        let next = match MutationDiff::apply(operation.diff(current).diff(), current) {
                            Ok(next) => next,
                            Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(current, next);
                        *self.active = Some(self.owners.as_ref().expect("hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
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
                PersistedDocumentHydrationStep::Pending(self.progress())
            }
            Phase::SeedApplied | Phase::SeedRedo => {
                let envelope = self.envelope.as_ref().expect("hydrated envelope remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
                let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.record_index) {
                    let Some(edit) = envelope.vcs.edits.iter().find(|edit| edit.id == *id) else { return self.reject(MemberOpenDiagnostic::Replay) };
                    let result = if self.phase == Phase::SeedApplied {
                        self.runtime.as_mut().expect("hydration runtime remains retained").push_applied_edit(edit)
                    } else {
                        self.runtime.as_mut().expect("hydration runtime remains retained").push_redo_edit(edit)
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
                    *self.active = Some(crate::os_store::retirement::owned_retirement(history));
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
                        runtime.set_local_actor_id(cursor.applied_edit_ids.last().and_then(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id)).and_then(|edit| edit.actor.clone()));
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
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
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
                SnapshotRetirementStep::Complete => Err("persisted hydration nested owner reported false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("persisted hydration nested owner exceeded close grant".into()),
                step => Ok(step),
            };
        }
        let owners = self.owners.as_ref().ok_or("persisted hydration lost its owner catalog")?;
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), 1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    self.runtime.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("persisted hydration runtime reported false terminal".into()),
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
        if let Some(validation) = self.validation.take() {
            *self.active = Some(owners.initial_snapshot_retirement.retire_owned(validation));
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
            *self.active = Some(crate::os_store::retirement::owned_retirement(history));
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
            *self.active = Some(crate::os_store::retirement::owned_retirement(expected));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(owner));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(schema) = self.schema.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(schema));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let owners = self.owners.as_mut().expect("persisted hydration owner catalog remains retained");
        match owners.store_disposer.close_uninstalled_step(1)? {
            SnapshotRetirementStep::Complete if owners.store_disposer.uninstalled_terminal_is_empty() => {
                self.owners.take();
                self.terminal = true;
                Ok(SnapshotRetirementStep::Complete)
            }
            SnapshotRetirementStep::Complete => Err("persisted hydration uninstalled disposer reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.pack.is_none()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.history.is_none()
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
            && self.pack.is_none()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.history.is_none()
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
