//! 🧬️ Hydration types, constructor and drive: the hydration diagnostic, progress and step, the phase, the retained config store hydration, its constructor from snapshots and the active drive with cancellation. See [`super`].

use super::*;

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
    M: ToValue + FromValue + Mutation<P>,
{
    Pending(ConfigStoreHydrationProgress),
    Ready(Box<ArtifactStore<P, M>>),
    Rejected(ConfigStoreHydrationDiagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Phase {
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

pub struct ReferenceConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: ToValue + FromValue + Mutation<P>,
{
    pub initial: ManuallyDrop<Option<semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>>>,
    pub(super) validation: ManuallyDrop<Option<P>>,
    pub current: ManuallyDrop<Option<P>>,
    pub history: ManuallyDrop<Option<std::sync::Arc<semio_framework_os_spr::HistoryLog>>>,
    pub fold_job: ManuallyDrop<Option<semio_framework_os_spr::HistoryFoldJob<'static, semio_framework_os_spr::history::RetainedHistoryFold>>>,
    pub fold: ManuallyDrop<Option<semio_framework_os_spr::HistoryFold>>,
    pub normalized_transitions: ManuallyDrop<Option<Vec<semio_framework_os_spr::MutationEnvelope>>>,
    pub(super) fold_auxiliary: ManuallyDrop<Option<(Vec<String>, Vec<semio_framework_os_spr::Conflict>)>>,
    pub fold_completed: u64,
    pub progress_high_water: std::cell::Cell<u64>,
    pub(super) source_edits: ManuallyDrop<Option<semio_framework_os_store_retirement::resident_backing::PagedSourceIterator<semio_framework_os_spr::HistoryEdit>>>,
    pub(super) source_forwards: ManuallyDrop<Option<semio_framework_os_store_retirement::resident_backing::SourceIterator<semio_framework_os_spr::OpPayload>>>,
    pub(super) source_inverse: ManuallyDrop<Option<semio_framework_os_store_retirement::resident_backing::SourceIterator<semio_framework_os_spr::OpPayload>>>,
    pub(super) source_metadata: ManuallyDrop<Option<semio_framework_os_store_retirement::resident_backing::SourceIterator<semio_framework_os_spr::HistoryOpMeta>>>,
    pub(super) pending_source_lane: ManuallyDrop<Option<String>>,
    pub(super) pending_snapshot: ManuallyDrop<Option<P>>,
    pub(super) pending_source_edit: ManuallyDrop<Option<semio_framework_os_spr::HistoryEdit>>,
    pub(super) pending_retired_payload: ManuallyDrop<Option<semio_framework_os_spr::OpPayload>>,
    pub(super) pending_payload: ManuallyDrop<Option<semio_framework_os_spr::OpPayload>>,
    pub(super) pending_metadata: ManuallyDrop<Option<semio_framework_os_spr::HistoryOpMeta>>,
    pub(super) expected_id: ManuallyDrop<Option<String>>,
    pub schema: ManuallyDrop<Option<String>>,
    pub envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    pub runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    pub owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pub pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pub pending_messages: ManuallyDrop<Option<semio_framework_os_spr::EditMessages>>,
    pub(super) retired_messages: ManuallyDrop<Option<semio_framework_os_spr::EditMessages>>,
    pub edit_lookup: ManuallyDrop<Option<protocol::HistoryFoldIndex<String, usize>>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub actor: ManuallyDrop<semio_framework_os_spr::ActorId>,
    pub initial_digest: [u8; 32],
    pub generation: u64,
    pub(super) maximum_value_bytes: usize,
    pub phase: Phase,
    pub edit_index: usize,
    pub operation_index: usize,
    pub record_index: usize,
    pub diagnostic: Option<ConfigStoreHydrationDiagnostic>,
    pub(super) last_retirement_progress: RetainedCloneProgress,
    pub terminal: bool,
}

impl<P, M> ReferenceConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn from_snapshots(
        initial: semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>,
        validation: P,
        current: P,
        history: semio_framework_os_spr::HistoryLog,
        expected_id: String,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        generation: u64,
        maximum_value_bytes: usize,
        actor: semio_framework_os_spr::ActorId,
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

    pub(super) fn metadata_retained_bytes(source: &semio_framework_os_spr::HistoryOpMeta) -> usize {
        source.op_id.as_ref().map_or(0, String::len)
            + source.dependencies.iter().map(String::len).sum::<usize>()
            + source.author_id.as_ref().map_or(0, semio_framework_value::SharedUtf8::len)
            + source.group_id.as_ref().map_or(0, String::len)
            + source.messages.iter().map(|message| message.code.len() + message.message.len() + message.target.iter().map(String::len).sum::<usize>()).sum::<usize>()
    }

    pub fn payload_bytes(payload: &semio_framework_os_spr::OpPayload) -> usize {
        payload.binary.as_ref().map_or_else(|| payload.text.as_ref().map_or(0, String::len), Vec::len)
    }

    pub fn decode_operation(payload: &semio_framework_os_spr::OpPayload) -> Result<M, ConfigStoreHydrationDiagnostic> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => M::decode_op(bytes).map_err(|_| ConfigStoreHydrationDiagnostic::Replay),
            (None, Some(text)) => M::parse_op(text).map_err(|_| ConfigStoreHydrationDiagnostic::Replay),
            (None, None) => Err(ConfigStoreHydrationDiagnostic::Malformed),
        }
    }

    pub(super) fn vacant_history_edit()->semio_framework_os_spr::HistoryEdit{semio_framework_os_spr::HistoryEdit{id:String::new(),actor:None,started_at:String::new(),finished_at:None,verb:None,line:None,ops:Vec::new(),inverse:Vec::new(),meta:None,lane:None}}

    pub fn reject(&mut self, diagnostic: ConfigStoreHydrationDiagnostic) -> ConfigStoreHydrationStep<P, M> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        ConfigStoreHydrationStep::Rejected(self.diagnostic.unwrap())
    }

    pub fn progress(&self) -> ConfigStoreHydrationProgress {
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        let completed = self.progress_high_water.get().max(self.fold_completed.saturating_add(self.record_index as u64));
        self.progress_high_water.set(completed);
        ConfigStoreHydrationProgress { completed, total: completed.max(self.fold_completed.saturating_add(total as u64)) }
    }

    pub(super) fn retire_edit(&mut self, edit: Edit<M>) {
        assert!(self.pending_edit.is_none(), "rejected original config edit retains its sole custody slot");
        *self.pending_edit = Some(edit);
    }

    pub fn drive_active(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneProgress>, ConfigStoreHydrationDiagnostic> {
        if self.active.is_none() { return Ok(None); }
        semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| Some(step.progress())).map_err(|_| ConfigStoreHydrationDiagnostic::Initialization)
    }

    pub(super) fn admit_input<T: semio_framework_value::retirement::RetireOwned>(source: &mut Option<T>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress)>, ValueError> {
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

    pub(super) fn close_source<R:ErasedSnapshotRetirement>(source:&mut Option<R>,grant:RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
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
            Phase::BeginEdit => self.source_edits.as_ref().and_then(|edits| edits.peek()).map_or(0,|edit|edit.id.len().saturating_mul(2).max(size_of::<Option<semio_framework_os_spr::HistoryEdit>>()+size_of::<usize>())),
            Phase::DecodeForward | Phase::DecodeInverse => self.pending_payload.as_ref().map_or(0, Self::payload_bytes).max(size_of::<M>()),
            Phase::DecodeMetadata => size_of::<semio_framework_os_spr::MutationMeta>().max(self.pending_metadata.as_ref().map_or(0, Self::metadata_retained_bytes)),
            Phase::FinishEdit => self.pending_edit.as_ref().map_or(0, |edit| edit.id.len()),
            _ => 0,
        })
    }

}
