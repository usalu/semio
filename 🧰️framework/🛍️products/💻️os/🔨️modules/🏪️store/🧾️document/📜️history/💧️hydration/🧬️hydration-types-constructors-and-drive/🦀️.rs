//! 🧬️ Hydration types, constructors and drive: the hydration grants, the retirement and runtime drives, the target, output, progress and step types, the phase, the replayed edits and the hydration constructors with the active drive. See [`super`].

use super::*;

pub fn hydration_fold_byte_grant(logical_bytes: usize, demand: usize) -> Option<usize> {
    let grant = logical_bytes.max(demand);
    (grant <= semio_framework_os_store_owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES).then_some(grant)
}

/// 🎫️ Debits nested depth and clips logical work without enlarging physical authority.
pub(super) fn hydration_child_grant(grant: RetainedCloneGrant, fuel: u64) -> Result<RetainedCloneGrant, MemberOpenDiagnostic> {
    let depth = grant.maximum_depth.checked_sub(1).ok_or(MemberOpenDiagnostic::Capacity)?;
    let bytes = usize::try_from(fuel).map_err(|_| MemberOpenDiagnostic::Capacity)?.min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES);
    Ok(RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_copy_bytes: grant.maximum_copy_bytes.min(bytes), maximum_depth: depth, ..grant })
}

/// 🧮️ Prices one granted turn as fuel from its copied items and bytes.
fn progress_fuel(progress: RetainedCloneProgress) -> Result<u64, MemberOpenDiagnostic> {
    progress.copied_items.checked_add(progress.copied_bytes).and_then(|fuel| u64::try_from(fuel).ok()).ok_or(MemberOpenDiagnostic::Capacity)
}

/// 🎫️ True when the grant funds the exact erased frame of one typed original owner.
pub(super) fn owned_frame_funded<T: semio_framework_value::retirement::RetireOwned>(grant: RetainedCloneGrant) -> bool {
    grant.maximum_items != 0 && grant.maximum_depth >= 2 && grant.maximum_capacity_bytes >= semio_framework_value::retirement::owned_retirement_birth_bytes::<T>()
}

/// 📦️ Forwards the supplied currencies to the current retained child.
pub fn drive_hydration_retirement(active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<bool, MemberOpenDiagnostic> {
    if active.is_none() { return Ok(false); }
    if cx.is_cancelled() || cx.should_yield() { return Ok(true); }
    let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
    let step = semio_framework_os_store_read::artifact_retirement_box_close_step(active, grant).map_err(|_| MemberOpenDiagnostic::Initialization)?;
    let progress = step.progress();
    semio_framework_os_store_member_open::record_member_step(cx, progress)?;
    Ok(true)
}

/// 🧾️ Closes the original initialization runtime under the supplied current authority.
pub fn drive_hydration_runtime<P: Send + Sync + 'static>(runtime: &mut ArtifactStoreInitializationRuntime<P>, factory: &std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<P>>, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<bool, MemberOpenDiagnostic> {
    if cx.is_cancelled() || cx.should_yield() { return Ok(false); }
    let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
    let step = runtime.close_step(factory, grant).map_err(|_| MemberOpenDiagnostic::Initialization)?;
    let terminal = runtime.terminal_is_empty();
    semio_framework_value::retained_clone::admit_retained_clone_close(grant, step, terminal, "hydration original initialization").map_err(|_| MemberOpenDiagnostic::Initialization)?;
    let progress = step.progress();
    semio_framework_os_store_member_open::record_member_step(cx, progress)?;
    Ok(terminal)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersistedDocumentHydrationTarget {
    Store { generation: u64 },
    Envelope,
}

pub enum PersistedDocumentHydrationOutput<P, M>
where
    P: ToValue + Clone + FromValue,
    M: ToValue + FromValue + Mutation<P> + Clone + semio_framework_os_spr::ColdMutation<P>,
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
    P: ToValue + Clone + FromValue,
    M: ToValue + FromValue + Mutation<P> + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    Pending(PersistedDocumentHydrationProgress),
    Ready(PersistedDocumentHydrationOutput<P, M>),
    Rejected(MemberOpenDiagnostic),
}

/// 🏪️ A Store-only hydration handoff preserves its declared output authority.
pub enum PersistedDocumentStoreHydrationStep<P, M>
where
    P: ToValue + Clone + FromValue,
    M: ToValue + FromValue + Mutation<P> + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    Pending(PersistedDocumentHydrationProgress),
    Ready(Box<ArtifactStore<P, M>>),
    Rejected(MemberOpenDiagnostic),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    ScanPack,
    DecodePack,
    Begin,
    Fold,
    AdmitInitializationCatalog,
    BindGenesis,
    AdmitEnvelope,
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

pub(super) struct HydratedReplayEdits<'a, M> {
    pub edits: &'a semio_framework_os_vcs::ArtifactHistoryLedger<Edit<M>>,
    pub positions: &'a std::collections::BTreeMap<String, usize>,
}

impl<M> semio_framework_os_store_replay::ReplayEdits<M> for HydratedReplayEdits<'_, M> {
    fn replay_edit(&self, id: &str) -> Option<&Edit<M>> {
        self.edits.get(*self.positions.get(id)?).filter(|edit| edit.id == id)
    }
}

pub struct RetainedPersistedDocumentHydration<P, M>
where
    P: ToValue + Clone + FromValue,
    M: ToValue + FromValue + Mutation<P> + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    pub actor: ManuallyDrop<semio_framework_os_spr::ActorId>,
    pub pack: ManuallyDrop<Option<Vec<u8>>>,
    pub initial: ManuallyDrop<Option<P>>,
    pub history: ManuallyDrop<Option<std::sync::Arc<semio_framework_os_spr::HistoryLog>>>,
    pub fold_job: ManuallyDrop<Option<semio_framework_os_spr::HistoryFoldJob<'static, semio_framework_os_spr::history::RetainedHistoryFold>>>,
    pub normalized_transitions: ManuallyDrop<Option<Vec<semio_framework_os_spr::MutationEnvelope>>>,
    pub normalized_conflicts: ManuallyDrop<Option<Vec<semio_framework_os_spr::Conflict>>>,
    pub replay_ids: ManuallyDrop<Option<Vec<String>>>,
    pub replay_order: ManuallyDrop<Option<semio_framework_os_vcs::HistoryPageStack<String>>>,
    pub replay_total: u32,
    pub pin_refs: ManuallyDrop<Option<std::collections::HashSet<ArtifactRef>>>,
    pub loaded_result: ManuallyDrop<Option<semio_framework_os_store_replay::EditReplayResult<P, M>>>,
    pub edit_lookup: ManuallyDrop<Option<std::collections::BTreeMap<String, usize>>>,
    pub mutation_lookup: ManuallyDrop<Option<std::collections::BTreeMap<semio_framework_os_spr::MutationId, (usize, usize)>>>,
    pub target_source: ManuallyDrop<Option<std::sync::Arc<Vec<semio_framework_os_spr::MutationEnvelope>>>>,
    pub target_decoder: ManuallyDrop<Option<semio_framework_os_spr::HistoryFoldJob<'static, semio_framework_os_spr::HistoryTransition>>>,
    pub pending_target: ManuallyDrop<Option<semio_framework_os_spr::HistoryTransition>>,
    pub target_address: ManuallyDrop<Option<Vec<String>>>,
    pub fold_completed: u64,
    pub progress_high_water: std::cell::Cell<u64>,
    pub fold: ManuallyDrop<Option<semio_framework_os_spr::HistoryFold>>,
    pub expected: ManuallyDrop<Option<ArtifactRef>>,
    pub owner: ManuallyDrop<Option<OwnerRef>>,
    pub schema: ManuallyDrop<Option<String>>,
    pub envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    pub runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    pub initialization_catalog: ManuallyDrop<Option<semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog>>,
    pub replay: ManuallyDrop<Option<EditReplay<P, M>>>,
    pub owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pub pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pub pending_messages: ManuallyDrop<Option<semio_framework_os_spr::EditMessages>>,
    pub active: ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub operation: OperationId,
    pub generation: Generation,
    pub expires_at_us: u64,
    pub target: PersistedDocumentHydrationTarget,
    pub phase: Phase,
    pub edit_index: usize,
    pub operation_index: usize,
    pub record_index: usize,
    pub pin_group_index: usize,
    pub pin_index: usize,
    pub pack_scanned: usize,
    pub pack_hasher: semio_framework_hash::Hasher,
    pub pack_digest: Option<[u8; 32]>,
    pub diagnostic: Option<MemberOpenDiagnostic>,
    pub terminal: bool,
}

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    pub fn from_decoded_pack(
        initial: P,
        pack: Vec<u8>,
        digest: [u8; 32],
        history: semio_framework_os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: semio_framework_os_spr::ActorId,
    ) -> Self {
        let mut hydration = Self::new(Some(pack), Some(initial), history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::Begin, actor);
        hydration.pack_digest = Some(digest);
        hydration
    }

    pub fn from_pack(
        pack: Vec<u8>,
        history: semio_framework_os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: semio_framework_os_spr::ActorId,
    ) -> Self {
        Self::new(Some(pack), None, history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::ScanPack, actor)
    }

    pub fn new(
        pack: Option<Vec<u8>>,
        initial: Option<P>,
        history: semio_framework_os_spr::HistoryLog,
        expected: ArtifactRef,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        phase: Phase,
        actor: semio_framework_os_spr::ActorId,
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
            replay_order: ManuallyDrop::new(Some(semio_framework_os_vcs::HistoryPageStack::new())),
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
            initialization_catalog: ManuallyDrop::new(None),
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

    pub fn decode_operation(payload: &semio_framework_os_spr::OpPayload) -> Result<M, MemberOpenDiagnostic> {
        match (&payload.binary, &payload.text) {
            (Some(bytes), _) => M::decode_op(bytes).map_err(|_| MemberOpenDiagnostic::Replay),
            (None, Some(text)) => M::parse_op(text).map_err(|_| MemberOpenDiagnostic::Replay),
            (None, None) => Err(MemberOpenDiagnostic::Malformed),
        }
    }

    pub fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> PersistedDocumentHydrationStep<P, M> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap())
    }

    pub fn progress(&self) -> PersistedDocumentHydrationProgress {
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        let completed = self.progress_high_water.get().max(self.fold_completed.saturating_add(self.record_index as u64));
        self.progress_high_water.set(completed);
        PersistedDocumentHydrationProgress { completed, total: completed.max(self.fold_completed.saturating_add(total as u64)) }
    }

    pub fn drive_active(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<bool, MemberOpenDiagnostic> {
        drive_hydration_retirement(&mut self.active, cx, grant)
    }

    pub(super) fn admit_active<T: semio_framework_value::retirement::RetireOwned>(&mut self, value: T, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<(), MemberOpenDiagnostic> {
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let (owner, progress) = semio_framework_value::retirement::admit_owned_retirement(value, child).map_err(|_| MemberOpenDiagnostic::Initialization)?;
        *self.active = Some(owner);
        semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "hydration owned retirement").map_err(|_| MemberOpenDiagnostic::Initialization)?;
        semio_framework_os_store_member_open::record_member_step(cx, progress)?;
        Ok(())
    }

    pub(super) fn admitted_step(&mut self, step: Result<RetainedCloneStep, ValueError>, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        match step.map_err(|_| MemberOpenDiagnostic::Initialization).and_then(|step| semio_framework_os_store_member_open::record_member_step(cx, step.progress())) {
            Ok(()) => PersistedDocumentHydrationStep::Pending(self.progress()),
            Err(diagnostic) => self.reject(diagnostic),
        }
    }

    pub(super) fn loaded_replay_retirement(&self, owners: &DocumentStoreOwners<P, M>) -> semio_framework_os_store_read::ArtifactHistoryReadRetirement<P, M> {
        semio_framework_os_store_read::ArtifactHistoryReadRetirement {derived:ManuallyDrop::new(None),derived_alias:ManuallyDrop::new(None), preview: ManuallyDrop::new(None), replay: ManuallyDrop::new(None), loaded_replay: ManuallyDrop::new(None), finished: ManuallyDrop::new(None), active: ManuallyDrop::new(None), plan_retirement: ManuallyDrop::new(None), draft_retirement: ManuallyDrop::new(None), registry_retirement: ManuallyDrop::new(None), snapshots: ManuallyDrop::new(Some(owners.initial_snapshot_retirement.clone())), mutations: ManuallyDrop::new(Some(owners.mutation_retirement.clone())), factory_retirement:std::array::from_fn(|_|None) }
    }

    /// 🏪️ Refuses a foreign immutable target before consuming any retained hydration input.
    pub fn step_store(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentStoreHydrationStep<P, M> {
        if !matches!(self.target, PersistedDocumentHydrationTarget::Store { .. }) {
            self.reject(MemberOpenDiagnostic::Initialization);
            return PersistedDocumentStoreHydrationStep::Rejected(self.diagnostic.unwrap());
        }
        match self.step(cx, grant) {
            PersistedDocumentHydrationStep::Pending(progress) => PersistedDocumentStoreHydrationStep::Pending(progress),
            PersistedDocumentHydrationStep::Rejected(diagnostic) => PersistedDocumentStoreHydrationStep::Rejected(diagnostic),
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Store(store)) => PersistedDocumentStoreHydrationStep::Ready(store),
            PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(envelope)) => {
                *self.envelope = Some(envelope);
                self.terminal = false;
                self.reject(MemberOpenDiagnostic::Initialization);
                PersistedDocumentStoreHydrationStep::Rejected(self.diagnostic.unwrap())
            }
        }
    }

}
