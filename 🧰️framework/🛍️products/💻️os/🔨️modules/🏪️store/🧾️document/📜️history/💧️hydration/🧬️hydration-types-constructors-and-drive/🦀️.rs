//! 🧬️ Hydration types, constructors and drive: the hydration grants, the retirement and runtime drives, the target, output, progress and step types, the phase, the replayed edits and the hydration constructors with the active drive. See [`super`].



use super::*;

/// 🪪️ A constructor declares the exact identity and optional artifact dialect; absence is compared, never inferred.
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub struct PersistedDocumentHydrationIdentity{pub id:String,pub dialect:Option<semio_framework_artifact_reference::ArtifactDialect>}
impl PersistedDocumentHydrationIdentity{
 pub fn matches(&self,id:&str,dialect:Option<(&str,&str,&str)>)->bool{self.id==id&&match(self.dialect.as_ref(),dialect){(None,None)=>true,(Some(expected),Some((kind,standard,subset)))=>expected.artifact_kind==kind&&expected.standard==standard&&expected.subset==subset,_=>false}}
}


pub fn hydration_fold_byte_grant(logical_bytes: usize, demand: usize) -> Option<usize> {
    let grant = logical_bytes.max(demand);
    (grant <= semio_os_store_owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES).then_some(grant)
}

/// 🎫️ Debits nested depth and clips logical work without enlarging physical authority.
pub(super) fn hydration_child_grant(grant: RetainedCloneGrant, fuel: u64) -> Result<RetainedCloneGrant, MemberOpenDiagnostic> {
    let depth = grant.maximum_depth.checked_sub(1).ok_or(MemberOpenDiagnostic::Capacity)?;
    let bytes = usize::try_from(fuel).map_err(|_| MemberOpenDiagnostic::Capacity)?.min(semio_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES);
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
    let step = semio_os_store_read::artifact_retirement_box_close_step(active, grant).result.map_err(|_| MemberOpenDiagnostic::Initialization)?;
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
    P: semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
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
    P: semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    Pending(PersistedDocumentHydrationProgress),
    Ready(PersistedDocumentHydrationOutput<P, M>),
    Rejected(MemberOpenDiagnostic),
}

/// 🏪️ A Store-only hydration handoff preserves its declared output authority.
pub enum PersistedDocumentStoreHydrationStep<P, M>
where
    P: semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
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
    pub edits: &'a semio_os_vcs::ArtifactHistoryLedger<Edit<M>>,
    pub positions: &'a std::collections::BTreeMap<String, usize>,
}

impl<M> semio_os_store_replay::ReplayEdits<M> for HydratedReplayEdits<'_, M> {
    fn replay_edit(&self, id: &str) -> Option<&Edit<M>> {
        self.edits.get(*self.positions.get(id)?).filter(|edit| edit.id == id)
    }
}

pub struct RetainedPersistedDocumentHydration<P, M>
where
    P: semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    pub actor: ManuallyDrop<semio_os_spr::ActorId>,
    pub(super) original_source:Option<semio_framework_job::OriginalSourceBinding>,
    pub(super) original_meta:OwnedSlot<semio_os_spr::MutationMeta>,
    pub(super) empty_messages:OwnedSlot<semio_os_spr::EditMessages>,
    pub(super) fold_output:OwnedSlot<semio_os_spr::history::RetainedHistoryFold>,
    pub(super) fold_protocol:OwnedSlot<protocol::ProtocolError>,
    pub(super) mutation_domain:OwnedSlot<M::DomainFailure>,
    pub(super) metadata_message:usize,
    pub(super) semantic_index:usize,
    pub(super) mutation_receiving:ManuallyDrop<Option<Box<M::Receiving>>>,
    pub(super) mutation_output:OwnedSlot<M>,
    pub original_failure:ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    pub original_context_failure:ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    pub pack: ManuallyDrop<Option<Vec<u8>>>,
    pub initial: ManuallyDrop<Option<P>>,
    pub history: ManuallyDrop<Option<std::sync::Arc<semio_os_spr::HistoryLog>>>,
    pub fold_job: ManuallyDrop<Option<semio_os_spr::HistoryFoldJob<'static, semio_os_spr::history::RetainedHistoryFold>>>,
    pub normalized_transitions: ManuallyDrop<Option<std::collections::VecDeque<semio_os_spr::MutationEnvelope>>>,
    pub transition_ledger:ManuallyDrop<Option<semio_os_vcs::ArtifactHistoryLedger<semio_os_spr::MutationEnvelope>>>,
    pub normalized_conflicts: ManuallyDrop<Option<Vec<semio_os_spr::Conflict>>>,
    pub replay_ids: ManuallyDrop<Option<Vec<String>>>,
    pub replay_order: ManuallyDrop<Option<semio_os_vcs::HistoryPageStack<String>>>,
    pub replay_total: u32,
    pub pin_refs: ManuallyDrop<Option<std::collections::HashSet<ArtifactRef>>>,
    pub loaded_result: ManuallyDrop<Option<semio_os_store_replay::EditReplayResult<P, M>>>,
    pub edit_lookup: ManuallyDrop<Option<std::collections::BTreeMap<String, usize>>>,
    pub mutation_lookup: ManuallyDrop<Option<std::collections::BTreeMap<semio_os_spr::MutationId, (usize, usize)>>>,
    pub target_source: ManuallyDrop<Option<std::sync::Arc<semio_os_vcs::ArtifactHistoryLedger<semio_os_spr::MutationEnvelope>>>>,
    pub target_decoder: ManuallyDrop<Option<semio_os_spr::HistoryFoldJob<'static, semio_os_spr::HistoryTransition>>>,
    pub pending_target: ManuallyDrop<Option<semio_os_spr::HistoryTransition>>,
    pub target_address: ManuallyDrop<Option<Vec<String>>>,
    pub fold_completed: u64,
    pub progress_high_water: std::cell::Cell<u64>,
    pub fold: ManuallyDrop<Option<semio_os_spr::HistoryFold>>,
    pub expected: ManuallyDrop<Option<PersistedDocumentHydrationIdentity>>,
    pub owner: ManuallyDrop<Option<OwnerRef>>,
    pub schema: ManuallyDrop<Option<String>>,
    pub envelope: ManuallyDrop<Option<ArtifactEnvelope<P, M>>>,
    pub runtime: ManuallyDrop<Option<ArtifactStoreInitializationRuntime<P>>>,
    pub initialization_catalog: ManuallyDrop<Option<semio_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog>>,
    pub replay: ManuallyDrop<Option<EditReplay<P, M>>>,
    pub owners: ManuallyDrop<Option<DocumentStoreOwners<P, M>>>,
    pub pending_edit: ManuallyDrop<Option<Edit<M>>>,
    pub pending_messages: ManuallyDrop<Option<semio_os_spr::EditMessages>>,
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
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    pub fn from_decoded_pack(
        initial: P,
        pack: Vec<u8>,
        digest: [u8; 32],
        history: semio_os_spr::HistoryLog,
        expected: PersistedDocumentHydrationIdentity,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: semio_os_spr::ActorId,
    ) -> Self where M:protocol::OpBinaryReceiving {
        let mut hydration = Self::new(Some(pack), Some(initial), history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::Begin, actor);
        hydration.pack_digest = Some(digest);
        hydration
    }

    pub fn from_pack(
        pack: Vec<u8>,
        history: semio_os_spr::HistoryLog,
        expected: PersistedDocumentHydrationIdentity,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        actor: semio_os_spr::ActorId,
    ) -> Self where M:protocol::OpBinaryReceiving {
        Self::new(Some(pack), None, history, expected, owner, schema, owners, operation, generation, expires_at_us, target, Phase::ScanPack, actor)
    }

    pub fn new(
        pack: Option<Vec<u8>>,
        initial: Option<P>,
        history: semio_os_spr::HistoryLog,
        expected: PersistedDocumentHydrationIdentity,
        owner: Option<OwnerRef>,
        schema: String,
        owners: DocumentStoreOwners<P, M>,
        operation: OperationId,
        generation: Generation,
        expires_at_us: u64,
        target: PersistedDocumentHydrationTarget,
        phase: Phase,
        actor: semio_os_spr::ActorId,
    ) -> Self where M:protocol::OpBinaryReceiving {
        Self {
            actor: ManuallyDrop::new(actor),
            original_source:None,original_meta:OwnedSlot::default(),empty_messages:OwnedSlot::default(),fold_output:OwnedSlot::default(),fold_protocol:OwnedSlot::default(),mutation_domain:OwnedSlot::default(),metadata_message:0,semantic_index:0,
            mutation_receiving:ManuallyDrop::new(None),mutation_output:OwnedSlot::default(),original_failure:ManuallyDrop::new(None),original_context_failure:ManuallyDrop::new(None),
            pack: ManuallyDrop::new(pack),
            initial: ManuallyDrop::new(initial),
            history: ManuallyDrop::new(Some(std::sync::Arc::new(history))),
            fold_job: ManuallyDrop::new(None),
            normalized_transitions: ManuallyDrop::new(None),
            transition_ledger:ManuallyDrop::new(Some(semio_os_vcs::ArtifactHistoryLedger::empty())),
            normalized_conflicts: ManuallyDrop::new(None),
            replay_ids: ManuallyDrop::new(None),
            replay_order: ManuallyDrop::new(Some(semio_os_vcs::HistoryPageStack::new())),
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


    pub(super) fn bind_original_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if let Some(binding)=self.original_source{return if binding.matches(cx){Ok(true)}else{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hydration changed its actual original Source").into())}}
        let bytes=std::mem::size_of_val(&self.original_source);let grant=cx.retained_grant();
        if cx.should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        self.original_source=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;Ok(false)
    }
    /// 🧾️ The actual Source receives the untouched mutation-domain cause into its own retained recipient.
    pub fn receive_original_mutation_domain_step(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut Option<M::DomainFailure>,&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if cx.operation()!=self.operation||cx.generation()!=self.generation||!self.original_source.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mutation domain receiver changed its original Source").into())}
        if self.mutation_domain.value.is_none(){
            if let Some(cursor)=self.mutation_receiving.as_mut(){
                if cursor.domain_failure().is_some(){
                    let demand=cursor.domain_failure_demand()?;let work=cursor.domain_failure_work_demand()?;let grant=cx.retained_grant();
                    if cx.retirement_should_yield()||cx.fuel_remaining()<work||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(false)}
                    let turn=cursor.take_domain_failure_into(&mut self.mutation_domain.value,grant);cx.consume_fuel(turn.work);
                    match turn.result{Ok(progress)=>cx.consume_retained(progress)?,Err(error)=>{let progress=error.retained_progress();let receipt=cx.consume_retained(progress);let mut failure=semio_framework_job::OriginalTurnFailure::from(error);failure.effects=progress;failure.receipt_refusal=receipt.err();return Err(failure)}}
                    if turn.work>work{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"mutation domain transfer exceeded its original work quote").into())}
                }
            }
        }
        if self.mutation_domain.value.is_some(){receive(&mut self.mutation_domain.value,cx)?}
        Ok(self.mutation_domain.terminal_is_empty()&&!self.mutation_receiving.as_ref().is_some_and(|cursor|cursor.domain_failure().is_some()))
    }

    /// 🪢️ The caller takes the complete actual fold protocol cause before its constructor can retire.
    pub fn receive_original_fold_protocol_step(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut Option<protocol::ProtocolError>,&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if cx.operation()!=self.operation||cx.generation()!=self.generation||!self.original_source.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"fold protocol receiver changed its actual original Source").into())}
        if self.fold_protocol.value.is_some(){receive(&mut self.fold_protocol.value,cx)?}
        Ok(self.fold_protocol.terminal_is_empty())
    }

    /// 🧯️ An actual receiving callback takes complete producer and context failures before their owner can close.
    pub fn receive_original_failures_step(&mut self,cx:&mut StepContext<'_>,receive:impl FnOnce(&mut Option<semio_framework_job::OriginalTurnFailure>,&mut Option<semio_framework_job::OriginalTurnFailure>,&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if cx.operation()!=self.operation||cx.generation()!=self.generation{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hydration failure receiver changed its original operation").into())}
        receive(&mut self.original_failure,&mut self.original_context_failure,cx)?;
        Ok(self.original_failure.is_none()&&self.original_context_failure.is_none())
    }
    pub fn check_authority(&self, cx: &StepContext<'_>) -> Result<(), MemberOpenDiagnostic> {
        if self.original_source.is_some_and(|source|!source.matches(cx)){return Err(MemberOpenDiagnostic::Stale)}
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

    pub fn reject(&mut self, diagnostic: MemberOpenDiagnostic) -> PersistedDocumentHydrationStep<P, M> {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = Phase::Rejected;
        PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap())
    }

    pub fn progress(&self) -> PersistedDocumentHydrationProgress {
        let total = self.history.as_ref().map_or(1, |history| history.edits.len() + history.transitions.len() + history.conflicts.len() + 1);
        let fold_completed=self.fold_job.as_ref().map_or(self.fold_completed,|job|job.completed());
        let completed = self.progress_high_water.get().max(fold_completed.saturating_add(self.record_index as u64));
        self.progress_high_water.set(completed);
        PersistedDocumentHydrationProgress { completed, total: completed.max(fold_completed.saturating_add(total as u64)) }
    }

    pub fn drive_active(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<bool, MemberOpenDiagnostic> {
        drive_hydration_retirement(&mut self.active, cx, grant)
    }

    pub(super) fn admit_active<T: semio_framework_value::retirement::RetireOwned>(&mut self, value: T, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> Result<(), MemberOpenDiagnostic> {
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let (owner, progress) = semio_framework_value::retirement::admit_owned_retirement(value, child).result.map_err(|_| MemberOpenDiagnostic::Initialization)?;
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

impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
M: protocol::OpBinaryReceiving + Clone + semio_os_spr::ColdMutation<P> + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static {
    pub(super) fn loaded_replay_retirement(&self, owners: &DocumentStoreOwners<P, M>) -> semio_os_store_read::ArtifactHistoryReadRetirement<P, M> {
        semio_os_store_read::ArtifactHistoryReadRetirement {derived:ManuallyDrop::new(None),derived_alias:ManuallyDrop::new(None), preview: ManuallyDrop::new(None), replay: ManuallyDrop::new(None), loaded_replay: ManuallyDrop::new(None), finished: ManuallyDrop::new(None), active: ManuallyDrop::new(None), plan_retirement: ManuallyDrop::new(None), draft_retirement: ManuallyDrop::new(None), registry_retirement: ManuallyDrop::new(None), snapshots: ManuallyDrop::new(Some(owners.initial_snapshot_retirement.clone())), mutations: ManuallyDrop::new(Some(owners.mutation_retirement.clone())), factory_retirement:std::array::from_fn(|_|None) }
    }

}

#[cfg(test)]
mod identity_source_laws{
 use super::*;
#[test]
fn declared_optional_hydration_identity_matches_the_portable_corpus(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../identity.json")).unwrap();
    let dialect=|value:&serde_json::Value|if value.is_null(){None}else{Some(semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()})};
    for row in law["cases"].as_array().unwrap(){let expected=PersistedDocumentHydrationIdentity{id:row["expectedId"].as_str().unwrap().into(),dialect:dialect(&row["expected"])};let actual=dialect(&row["actual"]);assert_eq!(expected.matches(row["actualId"].as_str().unwrap(),actual.as_ref().map(|value|(value.artifact_kind.as_str(),value.standard.as_str(),value.subset.as_str()))),row["accepted"].as_bool().unwrap());}
}

}
