//! 📥️ Bounded hydration of an exact persisted config history into a fresh config store.

use super::{
    mutation_meta_from_history_op_meta, ArtifactEnvelope, ArtifactStore, ArtifactStoreInitializationRuntime, DocumentStoreOwners, Edit, ErasedSnapshotRetirement, FromValue, Mutation, MutationDiff, OpBinary,
    OpText, SnapshotRetirementStep, ToValue,
};
use std::mem::ManuallyDrop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigStoreHydrationDiagnostic {
    Identity,
    Malformed,
    Replay,
    Capacity,
    Initialization,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConfigStoreHydrationProgress {
    pub completed: u64,
    pub total: u64,
}

pub enum ConfigStoreHydrationStep<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    Pending(ConfigStoreHydrationProgress),
    Ready(Box<ArtifactStore<P, M>>),
    Rejected(ConfigStoreHydrationDiagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Begin,
    BeginEdit,
    DecodeForward,
    DecodeInverse,
    DecodeMetadata,
    FinishEdit,
    ValidateReplay,
    RetireValidation,
    ReplayCursor,
    SeedApplied,
    SeedRedo,
    RetireHistory,
    Finish,
    Rejected,
}

pub struct RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    initial: ManuallyDrop<Option<P>>,
    validation: ManuallyDrop<Option<P>>,
    current: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<crate::os_spr::HistoryLog>>,
    source_edits: ManuallyDrop<Option<std::vec::IntoIter<crate::os_spr::HistoryEdit>>>,
    source_forwards: ManuallyDrop<Option<std::vec::IntoIter<crate::os_spr::OpPayload>>>,
    source_inverse: ManuallyDrop<Option<std::vec::IntoIter<crate::os_spr::OpPayload>>>,
    source_metadata: ManuallyDrop<Option<std::vec::IntoIter<crate::os_spr::HistoryOpMeta>>>,
    pending_payload: ManuallyDrop<Option<crate::os_spr::OpPayload>>,
    pending_metadata: ManuallyDrop<Option<crate::os_spr::HistoryOpMeta>>,
    expected_id: ManuallyDrop<Option<String>>,
    schema: ManuallyDrop<Option<String>>,
    envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pending_messages: ManuallyDrop<Option<crate::os_spr::EditMessages>>,
    edit_lookup: ManuallyDrop<Option<std::collections::BTreeMap<String, usize>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    initial_digest: [u8; 32],
    generation: u64,
    maximum_value_bytes: usize,
    phase: Phase,
    edit_index: usize,
    operation_index: usize,
    record_index: usize,
    diagnostic: Option<ConfigStoreHydrationDiagnostic>,
    terminal: bool,
}

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn from_snapshots(
        initial: P,
        validation: P,
        current: P,
        history: crate::os_spr::HistoryLog,
        expected_id: String,
        schema: String,
        initial_digest: [u8; 32],
        owners: DocumentStoreOwners<P, M>,
        generation: u64,
        maximum_value_bytes: usize,
    ) -> Self {
        Self {
            initial: ManuallyDrop::new(Some(initial)),
            validation: ManuallyDrop::new(Some(validation)),
            current: ManuallyDrop::new(Some(current)),
            history: ManuallyDrop::new(Some(history)),
            source_edits: ManuallyDrop::new(None),
            source_forwards: ManuallyDrop::new(None),
            source_inverse: ManuallyDrop::new(None),
            source_metadata: ManuallyDrop::new(None),
            pending_payload: ManuallyDrop::new(None),
            pending_metadata: ManuallyDrop::new(None),
            expected_id: ManuallyDrop::new(Some(expected_id)),
            schema: ManuallyDrop::new(Some(schema)),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(Some(owners)),
            pending_edit: ManuallyDrop::new(None),
            pending_messages: ManuallyDrop::new(None),
            edit_lookup: ManuallyDrop::new(Some(std::collections::BTreeMap::new())),
            active: ManuallyDrop::new(None),
            initial_digest,
            generation,
            maximum_value_bytes,
            phase: Phase::Begin,
            edit_index: 0,
            operation_index: 0,
            record_index: 0,
            diagnostic: None,
            terminal: false,
        }
    }

    fn decode_operation(payload: &crate::os_spr::OpPayload) -> Result<M, ConfigStoreHydrationDiagnostic> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => M::decode_op(bytes).map_err(|_| ConfigStoreHydrationDiagnostic::Replay),
            (None, Some(text)) => M::parse_op(text).map_err(|_| ConfigStoreHydrationDiagnostic::Replay),
            (None, None) => Err(ConfigStoreHydrationDiagnostic::Malformed),
        }
    }

    fn reject(&mut self, diagnostic: ConfigStoreHydrationDiagnostic) -> ConfigStoreHydrationStep<P, M> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        ConfigStoreHydrationStep::Rejected(self.diagnostic.unwrap())
    }

    fn progress(&self) -> ConfigStoreHydrationProgress {
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.changes.len() + history.checkpoints.len() + history.alternatives.len() + history.conflicts.len() + 1);
        ConfigStoreHydrationProgress { completed: self.record_index as u64, total: total as u64 }
    }

    fn retire_edit(&mut self, edit: Edit<M>) {
        *self.active = Some(self.owners.as_ref().expect("config hydration owner catalog remains retained").retire_decoded_edit(edit));
    }

    fn drive_active(&mut self, maximum_bytes: usize) -> Result<bool, ConfigStoreHydrationDiagnostic> {
        let Some(active) = self.active.as_mut() else { return Ok(false) };
        match active.close_step(1, maximum_bytes) {
            Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }) if released_items <= 1 && released_bytes <= maximum_bytes => Ok(true),
            Ok(SnapshotRetirementStep::Complete) if active.terminal_is_empty() => {
                self.active.take();
                Ok(true)
            }
            _ => Err(ConfigStoreHydrationDiagnostic::Initialization),
        }
    }

    pub fn request_cancel(&mut self) {
        self.diagnostic.get_or_insert(ConfigStoreHydrationDiagnostic::Cancelled);
        self.phase = Phase::Rejected;
    }

    pub fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> ConfigStoreHydrationStep<P, M> {
        if let Some(diagnostic) = self.diagnostic {
            return ConfigStoreHydrationStep::Rejected(diagnostic);
        }
        if maximum_items == 0 {
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        match self.drive_active(maximum_bytes) {
            Ok(true) => return ConfigStoreHydrationStep::Pending(self.progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        match self.phase {
            Phase::Begin => {
                if maximum_bytes < self.maximum_value_bytes {
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                let history = self.history.as_mut().expect("config history remains retained");
                let expected_id = self.expected_id.as_ref().expect("config identity remains retained");
                let schema = self.schema.as_ref().expect("config schema remains retained");
                if history.doc_id != *expected_id
                    || history.schema != *schema
                    || history.cursor.is_none()
                    || history.composition.is_some()
                    || !history.changes.is_empty()
                    || !history.checkpoints.is_empty()
                    || !history.alternatives.is_empty()
                    || history.active_alternative_id.is_some()
                    || !history.conflicts.is_empty()
                {
                    return self.reject(ConfigStoreHydrationDiagnostic::Identity);
                }
                let initial = self.initial.take().expect("typed config initial snapshot remains retained");
                let current = self.current.take().expect("typed config current snapshot remains retained");
                let expected_id = self.expected_id.take().expect("config identity remains retained");
                let schema = self.schema.take().expect("config schema remains retained");
                let mut envelope = crate::os_store::create_document_envelope::<P, M>(&schema, &expected_id, initial, None);
                let cursor = history.cursor.take().expect("validated persisted config cursor remains retained");
                envelope.cursor = Some(crate::os_store::ArtifactCursor::new(cursor.applied_edit_ids, cursor.redo_edit_ids, cursor.checkpoint_id));
                *self.source_edits = Some(std::mem::take(&mut history.edits).into_iter());
                *self.runtime = Some(ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, current, self.initial_digest));
                *self.envelope = Some(envelope);
                self.phase = Phase::BeginEdit;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BeginEdit => {
                let Some(source) = self.source_edits.as_mut().expect("config source edits remain retained").next() else {
                    self.source_edits.take();
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::ValidateReplay;
                    return ConfigStoreHydrationStep::Pending(self.progress());
                };
                if source.id.is_empty() || source.id.len() > self.maximum_value_bytes || maximum_bytes < source.id.len().saturating_mul(2) {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(source));
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                let Some(metadata) = source.meta else {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: None, ..source }));
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                };
                if metadata.len() != source.ops.len() {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(crate::os_spr::HistoryEdit { meta: Some(metadata), ..source }));
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                let edit = Edit {
                    id: source.id,
                    actor: source.actor,
                    forwards: Vec::new(),
                    inverse: Vec::new(),
                    mutation_meta: Vec::new(),
                    description: source.description,
                    coalesce_key: source.coalesce_key,
                    sequence_number: self.edit_index as i32 + 1,
                    started_at: source.started_at,
                    finished_at: source.finished_at,
                };
                *self.source_forwards = Some(source.ops.into_iter());
                *self.source_inverse = Some(source.inverse.into_iter());
                *self.source_metadata = Some(metadata.into_iter());
                let runtime = self.runtime.as_mut().expect("config hydration runtime remains retained");
                if runtime.seed_mutation(crate::os_spr::MutationId(edit.id.clone())).is_err() {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                runtime.observe_sequence(self.edit_index as i32 + 1);
                *self.pending_messages = Some(crate::os_spr::EditMessages { edit_id: edit.id.clone(), messages: Vec::new() });
                *self.pending_edit = Some(edit);
                self.operation_index = 0;
                self.phase = Phase::DecodeForward;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::DecodeForward | Phase::DecodeInverse => {
                let payloads = if self.phase == Phase::DecodeForward { self.source_forwards.as_mut() } else { self.source_inverse.as_mut() };
                if self.pending_payload.is_none() {
                    *self.pending_payload = payloads.expect("config operation source remains retained").next();
                }
                if let Some(payload) = self.pending_payload.as_ref() {
                    let payload_bytes = payload.binary.as_ref().map_or_else(|| payload.text.as_ref().map_or(0, String::len), Vec::len);
                    if payload_bytes == 0 || payload_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < payload_bytes.max(std::mem::size_of::<M>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    let operation = match Self::decode_operation(payload) {
                        Ok(operation) => operation,
                        Err(diagnostic) => return self.reject(diagnostic),
                    };
                    let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                    let target = if self.phase == Phase::DecodeForward { &mut edit.forwards } else { &mut edit.inverse };
                    if target.try_reserve_exact(1).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if self.phase == Phase::DecodeForward {
                        edit.forwards.push(operation);
                    } else {
                        edit.inverse.push(operation);
                    }
                    let payload = self.pending_payload.take().expect("decoded config payload remains retained");
                    *self.active = Some(crate::os_store::retirement::owned_retirement(payload));
                    self.operation_index += 1;
                } else {
                    if self.phase == Phase::DecodeForward {
                        self.source_forwards.take();
                    } else {
                        self.source_inverse.take();
                    }
                    self.operation_index = 0;
                    self.phase = if self.phase == Phase::DecodeForward { Phase::DecodeInverse } else { Phase::DecodeMetadata };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::DecodeMetadata => {
                if self.pending_metadata.is_none() {
                    *self.pending_metadata = self.source_metadata.as_mut().expect("config metadata source remains retained").next();
                }
                if let Some(source) = self.pending_metadata.as_ref() {
                    let retained_bytes = source.op_id.as_ref().map_or(0, String::len)
                        + source.dependencies.iter().map(String::len).sum::<usize>()
                        + source.author_id.as_ref().map_or(0, String::len)
                        + source.group_id.as_ref().map_or(0, String::len)
                        + source.messages.iter().map(|message| message.code.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>()).sum::<usize>();
                    if retained_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < retained_bytes.max(std::mem::size_of::<crate::os_spr::MutationMeta>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    let source = self.pending_metadata.take().expect("bounded config metadata remains retained");
                    let (meta, messages) = match mutation_meta_from_history_op_meta(source) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending config edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        if id.0 != edit_id && self.runtime.as_mut().expect("config hydration runtime remains retained").seed_mutation(id.clone()).is_err() {
                            return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                        }
                    }
                    self.runtime.as_mut().expect("config hydration runtime remains retained").observe_timestamp(meta.timestamp);
                    let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                    if edit.mutation_meta.try_reserve_exact(1).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    let pending_messages = self.pending_messages.as_mut().expect("pending message owner remains retained");
                    if pending_messages.messages.try_reserve_exact(messages.len()).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    edit.mutation_meta.push(meta);
                    pending_messages.messages.extend(messages);
                    self.operation_index += 1;
                } else {
                    self.source_metadata.take();
                    self.operation_index = 0;
                    self.phase = Phase::FinishEdit;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::FinishEdit => {
                let edit = self.pending_edit.take().expect("completed config edit remains retained");
                if edit.id.len() > self.maximum_value_bytes || maximum_bytes < edit.id.len() {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                let edit_id = edit.id.clone();
                let edit_position = self.envelope.as_ref().expect("config envelope remains retained").vcs.edits.len();
                if let Err(edit) = self.envelope.as_mut().expect("config envelope remains retained").vcs.edits.try_push(edit) {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                if self.edit_lookup.as_mut().expect("config edit lookup remains retained").insert(edit_id, edit_position).is_some() {
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                let messages = self.pending_messages.take().expect("completed message owner remains retained");
                if !messages.messages.is_empty() {
                    if let Err(messages) = self.envelope.as_mut().expect("config envelope remains retained").edit_messages.admit(messages) {
                        *self.active = Some(Box::new(super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)));
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                }
                self.edit_index += 1;
                self.record_index += 1;
                self.phase = Phase::BeginEdit;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::ValidateReplay => {
                let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                if let Some(edit) = envelope.vcs.edits.get(self.record_index) {
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let validation = self.validation.as_mut().expect("config validation projection remains retained");
                        let next = match MutationDiff::apply(operation.diff(validation).diff(), validation) {
                            Ok(next) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(validation, next);
                        *self.active = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    let validation = self.validation.take().expect("completed config validation projection remains retained");
                    *self.active = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(validation));
                    self.phase = Phase::RetireValidation;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::RetireValidation => {
                self.record_index = 0;
                self.operation_index = 0;
                self.phase = Phase::ReplayCursor;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::ReplayCursor => {
                let cursor = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref()).expect("persisted config cursor remains retained");
                if let Some(edit_id) = cursor.applied_edit_ids.get(self.record_index) {
                    let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                    let Some(position) = self.edit_lookup.as_ref().expect("config edit lookup remains retained").get(edit_id).copied() else { return self.reject(ConfigStoreHydrationDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed config edit remains retained");
                    if let Some(operation) = edit.forwards.get(self.operation_index) {
                        let current = self.runtime.as_mut().and_then(ArtifactStoreInitializationRuntime::current_mut).expect("config current remains retained");
                        let next = match MutationDiff::apply(operation.diff(current).diff(), current) {
                            Ok(next) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(current, next);
                        *self.active = Some(self.owners.as_ref().expect("config hydration owners remain retained").initial_snapshot_retirement.retire_owned(displaced));
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
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::SeedApplied | Phase::SeedRedo => {
                let envelope = self.envelope.as_ref().expect("config envelope remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted config cursor remains retained");
                let ids = if self.phase == Phase::SeedApplied { &cursor.applied_edit_ids } else { &cursor.redo_edit_ids };
                if let Some(id) = ids.get(self.record_index) {
                    let Some(position) = self.edit_lookup.as_ref().expect("config edit lookup remains retained").get(id).copied() else { return self.reject(ConfigStoreHydrationDiagnostic::Replay) };
                    let edit = envelope.vcs.edits.get(position).expect("indexed config edit remains retained");
                    let result = if self.phase == Phase::SeedApplied {
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_applied_edit(edit)
                    } else {
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_redo_edit(edit)
                    };
                    if result.is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    self.record_index += 1;
                } else {
                    self.record_index = 0;
                    self.phase = if self.phase == Phase::SeedApplied { Phase::SeedRedo } else { Phase::RetireHistory };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::RetireHistory => {
                if let Some(history) = self.history.take() {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(history));
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if let Some(index) = self.edit_lookup.take() {
                    *self.active = Some(crate::os_store::retirement::owned_retirement(index));
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                self.phase = Phase::Finish;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::Finish => {
                let envelope = self.envelope.take().expect("hydrated config envelope remains retained");
                let mut runtime = self.runtime.take().expect("hydrated config runtime remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted config cursor remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                runtime.set_local_actor_id(cursor.applied_edit_ids.last().and_then(|id| envelope.vcs.edits.iter().find(|edit| edit.id == *id)).and_then(|edit| edit.actor.clone()));
                let owners = self.owners.take().expect("config hydration owners remain retained");
                self.terminal = true;
                ConfigStoreHydrationStep::Ready(Box::new(crate::os_store::config_store_from_initialized_runtime_with_owners(envelope, runtime, self.generation, owners)))
            }
            Phase::Rejected => ConfigStoreHydrationStep::Rejected(self.diagnostic.unwrap_or(ConfigStoreHydrationDiagnostic::Cancelled)),
        }
    }
}

impl<P, M> ErasedSnapshotRetirement for RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.request_cancel();
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
                SnapshotRetirementStep::Complete => Err("config hydration nested owner reported false terminal".into()),
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes => Err("config hydration nested owner exceeded close grant".into()),
                step => Ok(step),
            };
        }
        let owners = self.owners.as_ref().ok_or("config hydration lost its owner catalog")?;
        if let Some(runtime) = self.runtime.as_mut() {
            return match runtime.close_step(owners.initial_snapshot_retirement.as_ref(), 1, maximum_bytes)? {
                SnapshotRetirementStep::Complete if runtime.terminal_is_empty() => {
                    self.runtime.take();
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                SnapshotRetirementStep::Complete => Err("config hydration runtime reported false terminal".into()),
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
        if let Some(current) = self.current.take() {
            *self.active = Some(owners.retire_initial_snapshot_owned(current));
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
        if let Some(index) = self.edit_lookup.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(index));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(expected_id) = self.expected_id.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(expected_id));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(schema) = self.schema.take() {
            *self.active = Some(crate::os_store::retirement::owned_retirement(schema));
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        let owners = self.owners.as_mut().expect("config hydration owner catalog remains retained");
        match owners.store_disposer.close_uninstalled_step(1)? {
            SnapshotRetirementStep::Complete if owners.store_disposer.uninstalled_terminal_is_empty() => {
                self.owners.take();
                self.terminal = true;
                Ok(SnapshotRetirementStep::Complete)
            }
            SnapshotRetirementStep::Complete => Err("config hydration uninstalled disposer reported false terminal".into()),
            step => Ok(step),
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.edit_lookup.is_none()
            && self.active.is_none()
    }
}

impl<P, M> Drop for RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_unbounded(), "retained config hydration reached Drop before exact handoff or bounded retirement");
    }
}

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn terminal_is_empty_unbounded(&self) -> bool {
        self.terminal
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.edit_lookup.is_none()
            && self.active.is_none()
    }
}
