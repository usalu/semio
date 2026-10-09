//! 📥️ Bounded hydration of an exact persisted config history into a fresh config store.

use super::{ArtifactEnvelope, ArtifactStore, ArtifactStoreInitializationRuntime, DocumentStoreOwners, Edit, ErasedSnapshotRetirement, FromValue, Mutation, ToValue, mutation_meta_from_history_op_meta};
use crate::os_spr::io::{binary::OpBinary, text::OpText};
use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

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
    Fold,
    BindGenesis,
    BuildAppliedCursor,
    BuildRedoCursor,
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
    initial: ManuallyDrop<Option<crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>>>,
    validation: ManuallyDrop<Option<P>>,
    current: ManuallyDrop<Option<P>>,
    history: ManuallyDrop<Option<std::sync::Arc<crate::os_spr::HistoryLog>>>,
    fold_job: ManuallyDrop<Option<crate::os_spr::HistoryFoldJob<'static, crate::os_spr::history::RetainedHistoryFold>>>,
    fold: ManuallyDrop<Option<crate::os_spr::HistoryFold>>,
    normalized_transitions: ManuallyDrop<Option<Vec<crate::os_spr::MutationEnvelope>>>,
    fold_auxiliary: ManuallyDrop<Option<(Vec<String>, Vec<crate::os_spr::Conflict>)>>,
    fold_completed: u64,
    progress_high_water: std::cell::Cell<u64>,
    source_edits: ManuallyDrop<Option<super::resident_backing::SourceIterator<crate::os_spr::HistoryEdit>>>,
    source_forwards: ManuallyDrop<Option<super::resident_backing::SourceIterator<crate::os_spr::OpPayload>>>,
    source_inverse: ManuallyDrop<Option<super::resident_backing::SourceIterator<crate::os_spr::OpPayload>>>,
    source_metadata: ManuallyDrop<Option<super::resident_backing::SourceIterator<crate::os_spr::HistoryOpMeta>>>,
    pending_source_lane: ManuallyDrop<Option<String>>,
    pending_snapshot: ManuallyDrop<Option<P>>,
    pending_source_edit: ManuallyDrop<Option<crate::os_spr::HistoryEdit>>,
    pending_retired_payload: ManuallyDrop<Option<crate::os_spr::OpPayload>>,
    pending_payload: ManuallyDrop<Option<crate::os_spr::OpPayload>>,
    pending_metadata: ManuallyDrop<Option<crate::os_spr::HistoryOpMeta>>,
    expected_id: ManuallyDrop<Option<String>>,
    schema: ManuallyDrop<Option<String>>,
    envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pending_messages: ManuallyDrop<Option<crate::os_spr::EditMessages>>,
    retired_messages: ManuallyDrop<Option<crate::os_spr::EditMessages>>,
    edit_lookup: ManuallyDrop<Option<protocol::HistoryFoldIndex<String, usize>>>,
    active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    actor: ManuallyDrop<crate::os_spr::ActorId>,
    initial_digest: [u8; 32],
    generation: u64,
    maximum_value_bytes: usize,
    phase: Phase,
    edit_index: usize,
    operation_index: usize,
    record_index: usize,
    diagnostic: Option<ConfigStoreHydrationDiagnostic>,
    last_retirement_progress: RetainedCloneProgress,
    terminal: bool,
}

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn from_snapshots(
        initial: crate::os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>,
        validation: P,
        current: P,
        history: crate::os_spr::HistoryLog,
        expected_id: String,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        generation: u64,
        maximum_value_bytes: usize,
        actor: crate::os_spr::ActorId,
    ) -> Self {
        let initial_digest = initial.facts().digest();
        Self {
            actor: ManuallyDrop::new(actor),
            initial: ManuallyDrop::new(Some(initial)),
            validation: ManuallyDrop::new(Some(validation)),
            current: ManuallyDrop::new(Some(current)),
            history: ManuallyDrop::new(Some(std::sync::Arc::new(history))),
            fold_job: ManuallyDrop::new(None),
            fold: ManuallyDrop::new(None),
            normalized_transitions: ManuallyDrop::new(None),
            fold_auxiliary: ManuallyDrop::new(None),
            fold_completed: 0,
            progress_high_water: std::cell::Cell::new(0),
            source_edits: ManuallyDrop::new(None),
            source_forwards: ManuallyDrop::new(None),
            source_inverse: ManuallyDrop::new(None),
            source_metadata: ManuallyDrop::new(None),
            pending_source_lane: ManuallyDrop::new(None),
            pending_snapshot: ManuallyDrop::new(None),
            pending_source_edit: ManuallyDrop::new(None),
            pending_retired_payload: ManuallyDrop::new(None),
            pending_payload: ManuallyDrop::new(None),
            pending_metadata: ManuallyDrop::new(None),
            expected_id: ManuallyDrop::new(Some(expected_id)),
            schema: ManuallyDrop::new(Some(schema)),
            envelope: ManuallyDrop::new(None),
            runtime: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(Some(owners)),
            pending_edit: ManuallyDrop::new(None),
            pending_messages: ManuallyDrop::new(None),
            retired_messages: ManuallyDrop::new(None),
            edit_lookup: ManuallyDrop::new(Some(protocol::HistoryFoldIndex::new())),
            active: ManuallyDrop::new(None),
            initial_digest,
            generation,
            maximum_value_bytes,
            phase: Phase::Begin,
            edit_index: 0,
            operation_index: 0,
            record_index: 0,
            diagnostic: None,
            last_retirement_progress: Default::default(),
            terminal: false,
        }
    }

    fn metadata_retained_bytes(source: &crate::os_spr::HistoryOpMeta) -> usize {
        source.op_id.as_ref().map_or(0, String::len)
            + source.dependencies.iter().map(String::len).sum::<usize>()
            + source.author_id.as_ref().map_or(0, semio_framework_value::SharedUtf8::len)
            + source.group_id.as_ref().map_or(0, String::len)
            + source.messages.iter().map(|message| message.code.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>()).sum::<usize>()
    }

    fn payload_bytes(payload: &crate::os_spr::OpPayload) -> usize {
        payload.binary.as_ref().map_or_else(|| payload.text.as_ref().map_or(0, String::len), Vec::len)
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
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        let completed = self.progress_high_water.get().max(self.fold_completed.saturating_add(self.record_index as u64));
        self.progress_high_water.set(completed);
        ConfigStoreHydrationProgress { completed, total: completed.max(self.fold_completed.saturating_add(total as u64)) }
    }

    fn retire_edit(&mut self, edit: Edit<M>) {
        assert!(self.pending_edit.is_none(), "rejected original config edit retains its sole custody slot");
        *self.pending_edit = Some(edit);
    }

    fn drive_active(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneProgress>, ConfigStoreHydrationDiagnostic> {
        if self.active.is_none() { return Ok(None); }
        super::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| Some(step.progress())).map_err(|_| ConfigStoreHydrationDiagnostic::Initialization)
    }

    fn admit_input<T: semio_framework_value::retirement::RetireOwned>(source: &mut Option<T>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>, ValueError> {
        if source.is_none() || grant.maximum_items == 0 { return Ok(None); }
        if grant.maximum_depth < 2 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "config retained input requires its original cursor depth")); }
        let capacity = semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<T>();
        if grant.maximum_capacity_bytes < capacity { return Ok(None); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let value = source.take().expect("observed original config input");
        match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(value, child) {
            Ok((owner, progress)) => Ok(Some((owner, progress))),
            Err((error, value)) => { *source = Some(value); Err(error) }
        }
    }

    pub fn last_retirement_progress(&self) -> RetainedCloneProgress { self.last_retirement_progress }

    fn close_source<T: semio_framework_value::retirement::RetireOwned>(source: &mut Option<super::resident_backing::SourceIterator<T>>, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
        let Some(owner) = source.as_mut() else { return Ok((true, Default::default())); };
        let depth = owner.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "config source parent depth overflow"))?;
        if grant.maximum_items == 0 { return Ok((false, Default::default())); }
        if grant.maximum_depth < depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "config source depth refused")); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.close_step(child)?;
        let terminal = owner.terminal_is_empty();
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, terminal, "config original source iterator")?;
        if terminal { drop(source.take()); }
        Ok((terminal, step.progress()))
    }

    pub fn request_cancel(&mut self) {
        self.diagnostic.get_or_insert(ConfigStoreHydrationDiagnostic::Cancelled);
        self.phase = Phase::Rejected;
    }

    /// 🎟️ The exact byte grant the next `advance` turn gates on: the schema-bounded value ceiling to begin, then each
    /// retained record's own admission; zero once no turn gates on bytes.
    pub fn demand_bytes(&self) -> Result<usize, ValueError> {
        if self.phase == Phase::Fold { return self.fold_job.as_ref().map_or(Ok(0), |job| job.next_copy_byte_demand()); }
        Ok(match self.phase {
            Phase::BeginEdit => self.source_edits.as_ref().and_then(|edits| edits.as_slice().first()).map_or(0, |edit| edit.id.len().saturating_mul(2)),
            Phase::DecodeForward | Phase::DecodeInverse => self.pending_payload.as_ref().map_or(0, Self::payload_bytes).max(size_of::<M>()),
            Phase::DecodeMetadata => size_of::<crate::os_spr::MutationMeta>().max(self.pending_metadata.as_ref().map_or(0, Self::metadata_retained_bytes)),
            Phase::FinishEdit => self.pending_edit.as_ref().map_or(0, |edit| edit.id.len()),
            _ => 0,
        })
    }

    pub fn advance(&mut self, grant: RetainedCloneGrant) -> ConfigStoreHydrationStep<P, M> {
        self.last_retirement_progress = Default::default();
        let maximum_items = grant.maximum_items;
        let maximum_bytes = grant.maximum_copy_bytes;
        if let Some(diagnostic) = self.diagnostic {
            return ConfigStoreHydrationStep::Rejected(diagnostic);
        }
        if maximum_items == 0 {
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        match self.drive_active(grant) {
            Ok(Some(progress)) => { self.last_retirement_progress = progress; return ConfigStoreHydrationStep::Pending(self.progress()); }
            Ok(None) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if self.fold_auxiliary.is_some() {
            match Self::admit_input(&mut self.fold_auxiliary, grant) {
                Ok(Some((owner, progress))) => {
                    *self.active = Some(owner);
                    if !progress.fits(grant) { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                    self.last_retirement_progress = progress;
                }
                Ok(None) => {}
                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.retired_messages.is_some() {
            let capacity = std::mem::size_of::<super::ArtifactStoreMessageLedgerRetirement>();
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            if grant.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let messages = self.retired_messages.take().expect("original exhausted messages retain their edit identity");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match super::admit_artifact_retirement(messages, child, |messages| super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Err((_, messages)) => { *self.retired_messages = Some(messages); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.pending_source_lane.is_some() {
            match Self::admit_input(&mut self.pending_source_lane, grant) {
                Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Ok(None) => {}
                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if let Some(snapshot) = self.pending_snapshot.as_ref() {
            let owners = self.owners.as_ref().expect("config pending snapshot retains its exact factory");
            let capacity = owners.initial_snapshot_retirement.retirement_birth_bytes(snapshot);
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            if grant.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let snapshot = self.pending_snapshot.take().expect("observed exact pending config snapshot");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match owners.retire_initial_snapshot_owned(snapshot, child) {
                Ok((owner, progress)) if progress.fits(child) && progress.retained_capacity_bytes == capacity => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                Ok((owner, _)) => { *self.active = Some(owner); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                Err((_, snapshot)) => { *self.pending_snapshot = Some(snapshot); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
            return ConfigStoreHydrationStep::Pending(self.progress());
        }
        if self.pending_retired_payload.is_some() {
            if grant.maximum_depth < 2 { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let capacity = semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<crate::os_spr::OpPayload>();
            if child.maximum_capacity_bytes < capacity { return ConfigStoreHydrationStep::Pending(self.progress()); }
            let payload = self.pending_retired_payload.take().expect("decoded original payload retains its custody slot");
            match semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(payload, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); self.last_retirement_progress = progress; return ConfigStoreHydrationStep::Pending(self.progress()); }
                Err((_, payload)) => { *self.pending_retired_payload = Some(payload); return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
            }
        }
        match self.phase {
            Phase::Begin => {
                if self.actor.0.trim().is_empty() { return self.reject(ConfigStoreHydrationDiagnostic::Initialization); }
                let history = self.history.as_ref().expect("config history remains retained");
                let expected_id = self.expected_id.as_ref().expect("config identity remains retained");
                let schema = self.schema.as_ref().expect("config schema remains retained");
                if history.doc_id != *expected_id || history.schema != *schema || history.composition.is_some() || !history.conflicts.is_empty() {
                    return self.reject(ConfigStoreHydrationDiagnostic::Identity);
                }
                *self.fold_job = Some(crate::os_spr::HistoryLog::fold_job(history.clone(), crate::os_spr::HistoryShape::Config));
                self.phase = Phase::Fold;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::Fold => {
                let job = self.fold_job.as_mut().expect("config fold job remains retained");
                let result = job.step(grant, &mut || false);
                self.fold_completed = job.completed();
                match result {
                    Ok(crate::os_spr::HistoryFoldJobStep::Pending { progress, .. }) if progress.fits(grant) => { self.last_retirement_progress = progress; }
                    Ok(crate::os_spr::HistoryFoldJobStep::Ready { value: (fold, transitions, replay_order, conflicts), progress }) => {
                        let valid_auxiliary = replay_order.is_empty() && conflicts.is_empty();
                        *self.fold = Some(fold);
                        *self.normalized_transitions = Some(transitions);
                        if replay_order.capacity() != 0 || conflicts.capacity() != 0 { *self.fold_auxiliary = Some((replay_order, conflicts)); }
                        if !progress.fits(grant) || !valid_auxiliary || !self.fold_job.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty) { return self.reject(ConfigStoreHydrationDiagnostic::Replay); }
                        self.last_retirement_progress = progress;
                        self.fold_job.take();
                        self.phase = Phase::BindGenesis;
                    }
                    _ => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BindGenesis => {
                let history = std::sync::Arc::get_mut(self.history.as_mut().expect("config history remains retained")).expect("config fold aliases close before genesis binding");
                let fold = self.fold.as_mut().expect("derived config history remains retained");
                let initial = self.initial.take().expect("typed config initial snapshot remains retained");
                let current = self.current.take().expect("typed config current snapshot remains retained");
                let expected_id = self.expected_id.take().expect("config identity remains retained");
                let schema = self.schema.take().expect("config schema remains retained");
                let mut envelope = crate::os_store::create_document_envelope_from_genesis::<P, M>(&schema, &expected_id, initial, None);
                envelope.history_shape = crate::os_spr::HistoryShape::Config;
                envelope.cursor = Some(crate::os_store::ArtifactCursor::new(Vec::new(), Vec::new(), fold.checkpoint.take()));
                envelope.transitions = self.normalized_transitions.take().expect("normalized config transitions remain retained");
                *self.source_edits = Some(super::resident_backing::SourceIterator::new(std::mem::take(&mut history.edits)));
                *self.runtime = Some(ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, std::sync::Arc::new(current), self.initial_digest, crate::os_spr::ActorId(std::mem::take(&mut self.actor.0))));
                *self.expected_id = Some(expected_id);
                *self.schema = Some(schema);
                *self.envelope = Some(envelope);
                self.operation_index = 0;
                self.phase = Phase::BuildAppliedCursor;
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BuildAppliedCursor | Phase::BuildRedoCursor => {
                let fold = self.fold.as_mut().expect("derived config history remains retained");
                let applied = self.phase == Phase::BuildAppliedCursor;
                let ids = if applied { &mut fold.applied } else { &mut fold.redo };
                if let Some(id) = ids.get_mut(self.operation_index) {
                    let cursor = self.envelope.as_mut().expect("config envelope remains retained").cursor.as_mut().expect("config cursor remains retained");
                    if applied { cursor.applied_edit_ids.push(std::mem::take(id)); } else { cursor.redo_edit_ids.push(std::mem::take(id)); }
                    self.operation_index += 1;
                } else {
                    self.operation_index = 0;
                    self.phase = if applied { Phase::BuildRedoCursor } else { Phase::BeginEdit };
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::BeginEdit => {
                let Some(source) = self.source_edits.as_mut().expect("config source edits remain retained").next() else {
                    match Self::close_source(&mut self.source_edits, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    self.record_index = 0;
                    self.operation_index = 0;
                    self.phase = Phase::ValidateReplay;
                    return ConfigStoreHydrationStep::Pending(self.progress());
                };
                if source.actor.as_deref().is_none_or(|actor| actor.trim().is_empty()) {
                    *self.pending_source_edit = Some(source);
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                if source.id.is_empty() || source.id.len() > self.maximum_value_bytes || maximum_bytes < source.id.len().saturating_mul(2) {
                    *self.pending_source_edit = Some(source);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                let Some(metadata) = source.meta else {
                    *self.pending_source_edit = Some(crate::os_spr::HistoryEdit { meta: None, ..source });
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                };
                if metadata.len() != source.ops.len() {
                    *self.pending_source_edit = Some(crate::os_spr::HistoryEdit { meta: Some(metadata), ..source });
                    return self.reject(ConfigStoreHydrationDiagnostic::Replay);
                }
                *self.pending_source_lane = source.lane;
                let edit = Edit {
                    line: source.line,
                    id: source.id,
                    actor: source.actor,
                    forwards: Vec::new(),
                    inverse: Vec::new().into(),
                    mutation_meta: Vec::new(),
                    verb: source.verb,
                    sequence_number: self.edit_index as i32 + 1,
                    started_at: source.started_at,
                    finished_at: source.finished_at,
                };
                *self.source_forwards = Some(super::resident_backing::SourceIterator::new(source.ops));
                *self.source_inverse = Some(super::resident_backing::SourceIterator::new(source.inverse));
                *self.source_metadata = Some(super::resident_backing::SourceIterator::new(metadata));
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
                    let payload_bytes = Self::payload_bytes(payload);
                    if payload_bytes == 0 || payload_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < payload_bytes.max(size_of::<M>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    if self.phase == Phase::DecodeInverse {
                        let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                        if !edit.inverse.has_reserved_slot() {
                            match edit.inverse.reserve_one(maximum_bytes) {
                                Ok(_) => return ConfigStoreHydrationStep::Pending(self.progress()),
                                Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Capacity),
                            }
                        }
                    }
                    let operation = match Self::decode_operation(payload) {
                        Ok(operation) => operation,
                        Err(diagnostic) => return self.reject(diagnostic),
                    };
                    let edit = self.pending_edit.as_mut().expect("pending config edit remains retained");
                    if self.phase == Phase::DecodeForward && edit.forwards.try_reserve_exact(1).is_err() {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if self.phase == Phase::DecodeForward {
                        edit.forwards.push(operation);
                    } else {
                        edit.inverse.push_reserved(operation).unwrap_or_else(|_| panic!("funded config inverse slot"));
                    }
                    let payload = self.pending_payload.take().expect("decoded config payload remains retained");
                    *self.pending_retired_payload = Some(payload);
                    self.operation_index += 1;
                } else {
                    if self.phase == Phase::DecodeForward {
                        match Self::close_source(&mut self.source_forwards, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    } else {
                        match Self::close_source(&mut self.source_inverse, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
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
                    if source.author_id.as_deref().is_none_or(|actor| actor.trim().is_empty()) { return self.reject(ConfigStoreHydrationDiagnostic::Replay); }
                    let retained_bytes = Self::metadata_retained_bytes(source);
                    if retained_bytes > self.maximum_value_bytes {
                        return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                    }
                    if maximum_bytes < retained_bytes.max(size_of::<crate::os_spr::MutationMeta>()) {
                        return ConfigStoreHydrationStep::Pending(self.progress());
                    }
                    let source = self.pending_metadata.take().expect("bounded config metadata remains retained");
                    let (meta, messages) = match mutation_meta_from_history_op_meta(source) {
                        Ok(decoded) => decoded,
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                    };
                    let edit_id = self.pending_edit.as_ref().expect("pending config edit remains retained").id.as_str();
                    if let Some(id) = &meta.mutation_id {
                        if self.runtime.as_mut().expect("config hydration runtime remains retained").seed_edit_operation(edit_id, id.clone()).is_err() {
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
                    match Self::close_source(&mut self.source_metadata, grant) {
                        Ok((terminal, progress)) => { self.last_retirement_progress = progress; if !terminal { return ConfigStoreHydrationStep::Pending(self.progress()); } }
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    self.operation_index = 0;
                    self.phase = Phase::FinishEdit;
                }
                ConfigStoreHydrationStep::Pending(self.progress())
            }
            Phase::FinishEdit => {
                let mut edit = self.pending_edit.take().expect("completed config edit remains retained");
                if edit.id.len() > self.maximum_value_bytes || maximum_bytes < edit.id.len() {
                    self.retire_edit(edit);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
                }
                super::stamp_edit_semantics::<P, M>(&mut edit, &self.envelope.as_ref().expect("config envelope remains retained").schema);
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
                if messages.messages.is_empty() {
                    *self.retired_messages = Some(messages);
                } else if let Err(messages) = self.envelope.as_mut().expect("config envelope remains retained").edit_messages.admit(messages) {
                    *self.pending_messages = Some(messages);
                    return self.reject(ConfigStoreHydrationDiagnostic::Capacity);
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
                        let next = match crate::os_vcs::apply_mutation(validation, operation) {
                            Ok((next, _)) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(validation, next);
                        *self.pending_snapshot = Some(displaced);
                        self.operation_index += 1;
                    } else {
                        self.record_index += 1;
                        self.operation_index = 0;
                    }
                } else {
                    let validation = self.validation.take().expect("completed config validation projection remains retained");
                    *self.pending_snapshot = Some(validation);
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
                        let next = match crate::os_vcs::apply_mutation(current, operation) {
                            Ok((next, _)) => next,
                            Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Replay),
                        };
                        let displaced = std::mem::replace(current, next);
                        *self.pending_snapshot = Some(displaced);
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
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_applied_edit(edit, envelope.vcs.edits.key_at(position).expect("indexed config edit generation remains retained"))
                    } else {
                        self.runtime.as_mut().expect("config hydration runtime remains retained").push_redo_edit(edit, envelope.vcs.edits.key_at(position).expect("indexed config edit generation remains retained"))
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
                if self.history.is_some() {
                    match Self::admit_input(&mut self.history, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.fold.is_some() {
                    match Self::admit_input(&mut self.fold, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.edit_lookup.is_some() {
                    match Self::admit_input(&mut self.edit_lookup, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.expected_id.is_some() {
                    match Self::admit_input(&mut self.expected_id, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
                    return ConfigStoreHydrationStep::Pending(self.progress());
                }
                if self.schema.is_some() {
                    match Self::admit_input(&mut self.schema, grant) {
                        Ok(Some((owner, progress))) => { *self.active = Some(owner); self.last_retirement_progress = progress; }
                        Ok(None) => {}
                        Err(_) => return self.reject(ConfigStoreHydrationDiagnostic::Initialization),
                    }
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
                let owners = self.owners.take().expect("config hydration owners remain retained");
                self.terminal = true;
                ConfigStoreHydrationStep::Ready(Box::new(crate::os_store::config_store_from_initialized_runtime_with_owners(envelope, runtime, self.generation, owners)))
            }
            Phase::Rejected => ConfigStoreHydrationStep::Rejected(self.diagnostic.unwrap_or(ConfigStoreHydrationDiagnostic::Cancelled)),
        }
    }
}

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn retirement_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        use semio_framework_value::retirement::RetireOwned;
        if self.terminal_is_empty_unbounded() { return Ok(Default::default()); }
        fn controlled<T: RetireOwned>(_: &T) -> Result<RetirementDemand, ValueError> {
            if !T::controlled_retirement_supported() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original config retained value has no controlled retirement declaration")); }
            Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
        }
        fn child(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "original config owner depth overflow"))?;
            Ok(demand)
        }
        if let Some(owner) = self.active.as_ref() { return super::artifact_retirement_box_demands(owner, copy); }
        if let Some(owner) = self.fold_job.as_ref() { return child(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? }); }
        if let Some(value) = self.normalized_transitions.as_ref() { return controlled(value); }
        if let Some(value) = self.fold.as_ref() { return controlled(value); }
        if let Some(value) = self.fold_auxiliary.as_ref() { return controlled(value); }
        if let Some(value) = self.source_edits.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_forwards.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_inverse.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_metadata.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.pending_source_lane.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_source_edit.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_retired_payload.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_payload.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_metadata.as_ref() { return controlled(value); }
        if let Some(owner) = self.runtime.as_ref() { return child(owner.initialization_retirement_demands(copy)?); }
        if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if self.retired_messages.is_some() || self.pending_messages.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactStoreMessageLedgerRetirement>(), depth: 2, ..Default::default() }); }
        if let Some(value) = self.pending_snapshot.as_ref().or_else(|| self.validation.as_ref()).or_else(|| self.current.as_ref()) { let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?; return Ok(RetirementDemand { capacity_bytes: owners.initial_snapshot_retirement.retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.envelope.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactStoreEnvelopeRetirement<P, M>>(), depth: 2, ..Default::default() }); }
        if self.initial.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<super::ArtifactGenesisRetirement<P>>(), depth: 2, ..Default::default() }); }
        if let Some(value) = self.history.as_ref() { return controlled(value); }
        if let Some(value) = self.edit_lookup.as_ref() { return controlled(value); }
        if let Some(value) = self.expected_id.as_ref() { return controlled(value); }
        if self.actor.0.has_owner() { return controlled(&self.actor.0); }
        if let Some(value) = self.schema.as_ref() { return controlled(value); }
        let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
        child(owners.uninstalled_owners_demands(copy)?)
    }
}

impl<P, M> ErasedSnapshotRetirement for RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.retirement_demands(0).map(|demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, semio_framework_value::ValueError> { self.retirement_demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.retirement_demands(0).map(|demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.retirement_demands(0).map(|demand| demand.depth) }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close}};
        use semio_framework_value::retirement::controlled::admit_typed_controlled_retirement;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "original config hydration depth refused")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.request_cancel();
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return super::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.fold_job.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config fold")?;
            if terminal { drop(self.fold_job.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        macro_rules! controlled {
            ($field:ident) => {
                if let Some(value) = self.$field.take() {
                    match admit_typed_controlled_retirement(value, child) {
                        Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                        Err((error, value)) => { *self.$field = Some(value); return Err(error); }
                    }
                }
            };
        }
        controlled!(normalized_transitions);
        controlled!(fold);
        controlled!(fold_auxiliary);
        if let Some(owner) = self.source_edits.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_edits")?;
            if terminal { drop(self.source_edits.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_forwards.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_forwards")?;
            if terminal { drop(self.source_forwards.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_inverse.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_inverse")?;
            if terminal { drop(self.source_inverse.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_metadata.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_metadata")?;
            if terminal { drop(self.source_metadata.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        controlled!(pending_source_lane);
        controlled!(pending_source_edit);
        controlled!(pending_retired_payload);
        controlled!(pending_payload);
        controlled!(pending_metadata);
        if let Some(owner) = self.runtime.as_mut() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let step = owner.close_step(&owners.initial_snapshot_retirement, child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config initialization")?;
            if terminal { drop(self.runtime.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending_edit.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let edit = self.pending_edit.take().expect("observed original config pending_edit");
            match owners.retire_decoded_edit(edit, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, edit)) => { *self.pending_edit = Some(edit); return Err(error); }
            }
        }
        if let Some(messages) = self.retired_messages.take() {
            match super::admit_artifact_retirement(messages, child, |messages| super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, messages)) => { *self.retired_messages = Some(messages); return Err(error); }
            }
        }
        if let Some(messages) = self.pending_messages.take() {
            match super::admit_artifact_retirement(messages, child, |messages| super::ArtifactStoreMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, messages)) => { *self.pending_messages = Some(messages); return Err(error); }
            }
        }
        if self.pending_snapshot.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.pending_snapshot.take().expect("observed original config pending_snapshot");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.pending_snapshot = Some(value); return Err(error); }
            }
        }
        if self.validation.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.validation.take().expect("observed original config validation");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.validation = Some(value); return Err(error); }
            }
        }
        if self.current.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.current.take().expect("observed original config current");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.current = Some(value); return Err(error); }
            }
        }
        if self.envelope.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.envelope.take().expect("observed original config envelope");
            match owners.retire_decoded_envelope(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.envelope = Some(value); return Err(error); }
            }
        }
        if self.initial.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.initial.take().expect("observed original config initial");
            match owners.retire_genesis_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.initial = Some(value); return Err(error); }
            }
        }
        controlled!(history);
        controlled!(edit_lookup);
        controlled!(expected_id);
        if self.actor.0.has_owner() {
            let value = std::mem::take(&mut self.actor.0);
            match admit_typed_controlled_retirement(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { self.actor.0 = value; return Err(error); }
            }
        }
        controlled!(schema);
        let owners = self.owners.as_mut().expect("original config catalog remains retained");
        let step = owners.close_uninstalled_owners_step(child)?;
        let terminal = owners.uninstalled_owners_terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "original config catalog")?;
        if terminal { drop(self.owners.take()); self.terminal = true; }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && !self.actor.0.has_owner()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.fold.is_none()
            && self.normalized_transitions.is_none()
            && self.fold_auxiliary.is_none()
            && self.source_edits.is_none()
            && self.source_forwards.is_none()
            && self.source_inverse.is_none()
            && self.source_metadata.is_none()
            && self.pending_source_lane.is_none()
            && self.pending_snapshot.is_none()
            && self.pending_source_edit.is_none()
            && self.pending_retired_payload.is_none()
            && self.pending_payload.is_none()
            && self.pending_metadata.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.retired_messages.is_none()
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
            && !self.actor.0.has_owner()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.fold.is_none()
            && self.normalized_transitions.is_none()
            && self.fold_auxiliary.is_none()
            && self.source_edits.is_none()
            && self.source_forwards.is_none()
            && self.source_inverse.is_none()
            && self.source_metadata.is_none()
            && self.pending_source_lane.is_none()
            && self.pending_snapshot.is_none()
            && self.pending_source_edit.is_none()
            && self.pending_retired_payload.is_none()
            && self.pending_payload.is_none()
            && self.pending_metadata.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.retired_messages.is_none()
            && self.edit_lookup.is_none()
            && self.active.is_none()
    }
}
