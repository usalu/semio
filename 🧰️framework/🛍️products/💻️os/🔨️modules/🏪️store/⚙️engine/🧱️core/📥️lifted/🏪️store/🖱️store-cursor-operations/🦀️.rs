//! 🖱️ Store cursor operations: the cursor demands, time travel reads, one item publication and event sourced history operations of the store cursor. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreCursor<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn generation(&self, store_wire: &StoreWire<P, Mutation>) -> u64 {
        store_wire.durable_group_read_root().map_or(self.generation, |root| root.generation)
    }

    /// 🪪️ Reads the event-maintained generation at an already-owned operation boundary.
    pub fn generation_now(&self, store_wire: &StoreWire<P, Mutation>) -> u64 {
        self.generation(store_wire)
    }

    pub fn content_revision(&self, store_wire: &StoreWire<P, Mutation>) -> [u8; 32] {
        store_wire.durable_group_read_root().map_or(self.content_revision, |root| root.content_revision)
    }

    /// 🧬️ Reads the fixed-width event-maintained content revision without suspension.
    pub fn content_revision_now(&self, store_wire: &StoreWire<P, Mutation>) -> [u8; 32] {
        self.content_revision(store_wire)
    }

    /// 🎯️ Returns the exact revision plus local generation a projection result must match.
    pub fn projection_stamp(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>) -> ArtifactProjectionStamp {
        ArtifactProjectionStamp { revision: store_wire.artifact_revision(store_log), generation: self.generation(store_wire) }
    }

    /// 📣️ Returns the last successful transition through the shared projection invalidation seam.
    ///
    /// 🌀️ `projection_stamp` is async; `Option::map`'s closure is sync (R10 shape 1).
    pub fn last_projection_invalidation(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>) -> Option<ArtifactProjectionInvalidation> {
        store_wire.durable_group_read_root().map_or(self.last_projection_cause, |root| root.last_projection_cause).map(|cause| ArtifactProjectionInvalidation { cause, stamp: self.projection_stamp(store_log, store_wire) })
    }

    /// 🛂️ Accepts only a result computed for the exact current revision and generation. Accepted
    /// semantic diffs are returned to their owning strict-apply boundary, never applied here.
    ///
    /// 🌀️ A future is consumed by one `.await` (R10 shape 2) — resolved once, reused below.
    pub fn accept_projection_result<Output, Diff>(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, result: ArtifactProjectionResult<Output, Diff>) -> Result<AcceptedArtifactProjection<Output, Diff>, StaleArtifactProjection> {
        let current = self.projection_stamp(store_log, store_wire);
        if result.stamp != current {
            return Err(StaleArtifactProjection { computed_for: result.stamp, current });
        }
        Ok(AcceptedArtifactProjection { output: result.output, proposed_diff: result.proposed_diff })
    }

    pub fn seal_derived_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        ArtifactDerivedSnapshot { owner, registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire) }
    }

    pub fn admit_derived_source(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, registry: &SnapshotReadRegistryHandle, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        if !registry.ptr_eq(&store_owners.snapshot_read_leases) || generation != self.generation(store_wire) || revision != self.content_revision(store_wire) {
            return Err(VcsError::ValidationFailed("derived snapshot belongs to another Store or event frontier".into()));
        }
        Ok(())
    }

    /// 🪞️ Issues a member read only from this Store's genuine derived projection capability.
    pub fn snapshot_read_derived(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>) -> Result<ErasedSnapshotRead, VcsError>
    where
        P: Sync,
    {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        if !store_owners.snapshot_read_leases.publish_authority(source.generation, source.revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let alias = Arc::clone(&source.owner);
        let lease = store_owners.snapshot_read_leases.try_issue(Arc::clone(&alias)).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(ErasedSnapshotRead::new(alias, lease))
    }

    /// 🧩️ Derives a provisional member projection through this Store's own operation vocabulary and merge policy.
    pub fn derive_provisional_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as super::Mutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| semio_framework_os_spr::apply_diff(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as super::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(store_owners, store_wire, Arc::new(owner))))
    }

    /// ✏️ Derives one history preview input with the replay's keep-and-record application rule.
    pub fn derive_history_snapshot(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> {
        self.admit_derived_source(store_owners, store_wire, &source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as super::Mutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = semio_framework_os_spr::apply_diff(&diff, source.snapshot());
        <<Mutation as super::Mutation<P>>::Diff as MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(store_owners, store_wire, Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    /// 🕰️ Starts a preview cursor without scanning, copying or hashing the live history.
    pub fn begin_derived_history_preview(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedHistoryPreview {
            plan: Some(HistoryReadPlan::new(Some(target), drafts, store_log.applied_edit_ids.len(), false)), plan_retirement:None, drafts: protocol::HistoryInputDrafts::new(), target: None, cursor: 0, operation: 0, state: None,
            progress: ReplayProgress::default(), registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire), marker: PhantomData, finished: false,
        })
    }

    /// 🏁️ Seals a completed prefix against the same store frontier that admitted its cursor.
    pub fn finish_derived_history_preview(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: &mut Option<ArtifactDerivedHistoryPreview<P, Mutation>>) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let preview = owner.as_mut().ok_or(VcsError::HistoryReplaying)?;
        self.admit_derived_source(store_owners, store_wire, &preview.registry, preview.generation, preview.revision)?;
        Ok(self.seal_derived_snapshot(store_owners, store_wire, preview.take_finished_snapshot_owner()?))
    }

    /// ⏪️ Starts sealed replay planning; every history identity and effective-input copy happens in a later slice.
    pub fn begin_derived_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        store_wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedReplay { replay: None, plan: Some(HistoryReadPlan::new(from, drafts, store_log.applied_edit_ids.len(), true)), plan_retirement:None, registry: store_owners.snapshot_read_leases.clone(), generation: self.generation(store_wire), revision: self.content_revision(store_wire) })
    }

    /// 🏁️ Returns a finished replay and its genuine complete projection for preview and subsequent publication.
    pub fn finish_derived_report_replay(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, owner: &mut Option<ArtifactDerivedReplay<P, Mutation>>) -> Result<(EditReplayResult<P, Mutation>, Option<ArtifactDerivedSnapshot<P>>), VcsError> {
        let replay = owner.as_ref().ok_or_else(|| VcsError::ValidationFailed("derived replay owner is absent".into()))?;
        self.admit_derived_source(store_owners, store_wire, &replay.registry, replay.generation, replay.revision)?;
        if !replay.replay.as_ref().is_some_and(|replay|replay.is_finished()) { return Err(VcsError::ValidationFailed("derived replay has not finished".into())); }
        let replay = owner.take().expect("admitted finished derived replay owner is present");
        let result = replay.replay.expect("finished replay cursor").finish().expect("admitted finished replay completes without another fallible operation");
        let head = result.state().map(|owner| self.seal_derived_snapshot(store_owners, store_wire, Arc::clone(owner)));
        Ok((result, head))
    }

    pub fn revision_owner_slots(value: &CursorRevisionAccumulator) -> usize {
        usize::from(!value.applied.is_empty() || value.applied.capacity() != 0 || !value.redo.is_empty() || value.redo.capacity() != 0 || !value.mutation_positions.is_empty() || !value.indexed_edits.is_empty() || !value.unit_flags.is_empty())
    }

    pub fn push_revision_replacement_reserved(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: CursorRevisionAccumulator) {
        let previous = std::mem::replace(&mut *self.revision_accumulator, next);
        if !previous.applied.is_empty() || previous.applied.capacity() != 0 || !previous.redo.is_empty() || previous.redo.capacity() != 0 || !previous.mutation_positions.is_empty() || !previous.indexed_edits.is_empty() || !previous.unit_flags.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreRevisionAccumulatorRetirement::new(previous)));
        }
    }

    fn replace_revision_accumulator_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: CursorRevisionAccumulator) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::revision_owner_slots(&self.revision_accumulator))?;
        self.push_revision_replacement_reserved(store_owners, next);
        Ok(())
    }

    //#region 🔖️TimeTravelReads
    /// 📍️ One exact identity's current applied position, validated against the maintained revision authority.
    pub fn mutation_position(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, target: &MutationId) -> Result<(usize, usize), VcsError> { self.locate_mutation(store_log, store_wire, target) }

    /// 🔎️ The maintained applied position of `edit_id`, fenced by the exact current cursor identity.
    pub fn applied_edit_position(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, edit_id: &str) -> Option<usize> {
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit_id.as_bytes()]);
        let root = store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        let position = root.and_then(|root| root.indexed_edits.get(&digest)).or_else(|| self.revision_accumulator.indexed_edits.get(&digest))?.0;
        store_wire.applied_edit_ids(store_log).get(position).filter(|id| id.as_str() == edit_id).map(|_| position)
    }

    pub fn applied_revision<'a>(&'a self, store_wire: &'a StoreWire<P, Mutation>) -> &'a CursorRevisionAccumulator {
        store_wire.durable_group_read_root().map_or(&self.revision_accumulator, |root| root.revision_accumulator.as_ref().expect("committed group retains its exact revision"))
    }

    pub fn applied_unit_flag(&self, store_wire: &StoreWire<P, Mutation>, edit: [u8; 32], operation: usize) -> bool {
        store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref()).and_then(|root| root.unit_flags.get(&(edit, operation))).or_else(|| self.revision_accumulator.unit_flags.get(&(edit, operation))).copied().unwrap_or(false)
    }

    /// 📖️ The applied edit at `position`.
    pub fn applied_edit<'a>(&self, store_log: &'a StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, position: usize) -> Result<&'a Edit<Mutation>, VcsError> {
        let edit_id = store_wire.applied_edit_ids(store_log).get(position).ok_or_else(|| VcsError::ValidationFailed(format!("no applied edit at position {position}")))?;
        self.applied_revision(store_wire).applied.get(position).and_then(|record| record.ledger_key).and_then(|key| store_log.envelope.vcs.edits.get_key(key)).filter(|edit| edit.id == *edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))
    }

    /// 📍️ The applied position of the edit owning `mutation_id` and the operation's index within it.
    pub fn locate_mutation(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, mutation_id: &MutationId) -> Result<(usize, usize), VcsError> {
        let root = store_wire.durable_group_read_root().and_then(|root| root.revision_accumulator.as_ref());
        if let Some((position, index, digest)) = root.and_then(|root| root.mutation_positions.get(mutation_id)).or_else(|| self.revision_accumulator.mutation_positions.get(mutation_id)) {
            if store_wire.applied_edit_ids(store_log).get(*position).filter(|id| CursorRevisionAccumulator::hash_record(b"edit-id", &[id.as_bytes()]) == *digest).is_some() {
                let edit = self.applied_edit(store_log, store_wire, *position)?;
                if semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, *index).as_ref() == Some(mutation_id) { return Ok((*position, *index)); }
            }
        }
        Err(VcsError::ValidationFailed(format!("{} is not an applied operation", mutation_id.0)))
    }

    /// 🪬️ This store's part of the unit of `mutation_id`, in operation order: every operation its edit recorded under the
    /// unit's group, or the operation alone when it is its own unit ([`Self::unit_id`]).
    pub fn unit_operations(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, mutation_id: &MutationId) -> Result<Vec<MutationId>, VcsError> {
        let (position, index) = self.locate_mutation(store_log, store_wire, mutation_id)?;
        let edit = self.applied_edit(store_log, store_wire, position)?;
        let group = edit.mutation_meta.get(index).and_then(|meta| meta.group_id.as_ref());
        if group.is_none() || self.applied_edit_units(store_log, store_wire, position)?[index].is_none() {
            return Ok(vec![mutation_id.clone()]);
        }
        Ok(semio_framework_os_spr::mutation_ids_for_edit::<P, Mutation>(edit).into_iter().enumerate().filter(|(member, _)| edit.mutation_meta.get(*member).and_then(|meta| meta.group_id.as_ref()) == group).map(|(_, member)| member).collect())
    }

    /// 🫴️ Recorded unit facts captured against each original operation's actual historical prestate by publication,
    /// initialization and replay; metadata supplies the transaction origin and the gesture's current group identity.
    pub fn applied_edit_units(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, position: usize) -> Result<Vec<Option<String>>, VcsError> {
        let edit = self.applied_edit(store_log, store_wire, position)?;
        let digest = CursorRevisionAccumulator::hash_record(b"edit-id", &[edit.id.as_bytes()]);
        Ok((0..edit.forwards.len()).map(|index| {
            let meta = edit.mutation_meta.get(index);
            let spans = self.applied_unit_flag(store_wire, digest, index) || matches!(meta.map(|meta| &meta.origin), Some(semio_framework_os_spr::MutationOrigin::Transaction { .. }));
            spans.then(|| meta.and_then(|meta| meta.group_id.clone()).unwrap_or_else(|| semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded unit identity").0))
        }).collect())
    }

    //#endregion 🔖️TimeTravelReads
    //#region 📬️OneItemPublication
    /// 🎯️ Publishes exactly one durable lane item after validating the live generation.
    /// The returned lane witness is independent of the edit receipt so a retained publisher can
    /// validate, ACK, retry, and retire one semantic unit without entering a batch API.
    /// 🎟️ Admits ONE gesture's exact app-owned retained mutation preparations. No generic mutation
    /// method is called here or by `advance_apply_batch`; absence of a domain factory is explicit.
    /// A single-mutation publication is simply the `mutations.len() == 1` case of this one machine.

    /// 🎟️ Admits the original captured actor, input backing and transaction before any constructor work.
    pub fn admit_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, lane:HistoryLane,
        factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,
        transaction:&mut Option<protocol::TransactionRef>, verb:Option<&str>, outbound:bool, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        use semio_framework_value::{ValueRefusalKind,retirement::controlled::ControlledRetirement};
        let factory=factory.ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original typed preparation issuer"))?;
        let retirement=store_owners.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original mutation retirement issuer"))?;
        let original_actor=actor.as_ref().filter(|owner|owner.original_is_untouched()).and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original paid actor capture"))?;
        if mutations.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires original mutation inputs"));}
        if store_wire.durable_group_root.is_some()||store_log.envelope.open_transaction.as_ref().is_some_and(|open|transaction.as_ref().is_none_or(|transaction|transaction.id!=open.transaction.id)){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication has an unresolved original transaction or group decision"));}
        if self.generation!=expected_generation||self.content_revision!=expected_revision{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original generation or revision is stale"));}
        if lane!=HistoryLane::Document||store_wire.backbone.is_some()&&!outbound{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"batched publication requires its original document/outbound lane"));}
        if original_actor.is_empty()||original_actor.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication actor exceeds its original identity bound"));}
        let cursor=store_log.envelope.cursor.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication requires its original live cursor"))?;
        if cursor.applied_edit_ids.len()!=store_log.applied_edit_ids.len()||cursor.redo_edit_ids.len()!=store_log.redo_edit_ids.len()||cursor.checkpoint_id.as_deref()!=store_log.current_checkpoint_id.as_deref(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original cursor is stale"));}
        let next_sequence_number=store_log.edit_sequence.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original sequence is exhausted"))?;
        let mut next_clock=store_log.clock;next_clock.tick(now_ms());
        match factory.stamped_clock(){Some(stamped)if store_log.edit_sequence==0||(stamped.physical_ms,stamped.logical)>(store_log.clock.physical_ms,store_log.clock.logical)=>next_clock=stamped,Some(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original stamp does not follow the Store clock")),None=>{}}
        let stamped=factory.stamped_mutation_id().map(|identity|identity.0.as_str());let line=store_log.envelope.active_alternative_id.as_deref();
        if stamped.is_some_and(|text|text.is_empty()||text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||line.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES)||verb.is_some_and(|text|text.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"batched publication original metadata exceeds its admitted identity bound"));}
        let text_bytes=stamped.map_or(0,str::len)+line.map_or(0,str::len)+verb.map_or(0,str::len);
        let copied_bytes=size_of::<ArtifactTypedBatchItemAuthority<P,Mutation>>()+size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+size_of::<ArtifactStoreOneItemLiveAuthority>()+size_of::<ArtifactStoreBatchPublication<P,Mutation>>()+size_of::<Vec<Mutation>>()+size_of::<Option<protocol::TransactionRef>>()+size_of_val(actor)+text_bytes;
        let retained_capacity_bytes=size_of::<ArtifactStoreBatchSourceOf<P,Mutation,ArtifactTypedBatchItemAuthority<P,Mutation>>>()+semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<ArtifactStoreOneItemLiveAuthority>()+text_bytes;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_capacity_bytes<retained_capacity_bytes||grant.maximum_depth==0{return Ok(None);}
        let text=|original:&str|{let mut value=String::with_capacity(original.len());value.push_str(original);value};
        let stamped_edit_id=stamped.map(text);let line=line.map(text);let verb=verb.map(text);
        let admitted_items=mutations.len();
        let original_actor=actor.as_mut().unwrap().take_original().expect("observed untouched original captured actor");drop(actor.take());
        let source=ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority{preparation:Arc::clone(factory),retirement:Arc::clone(retirement)},std::mem::take(mutations),None,store_log.operation_wire);
        let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(store_log.mint_witness(), operation, semio_framework_job::Generation(expected_generation), expected_revision, store_log.applied_edit_ids.len(), next_sequence_number, next_clock, original_actor, line, None, stamped_edit_id));
        let publication=ArtifactStoreBatchPublication{operation,expected_generation,expected_revision,lane,footprint:ArtifactStoreOneItemFootprint{work_items:0,retained_bytes:0},source_census_completed:0,source_census_finished:false,admitted_items,authority:Some(authority),authority_retirement:None,source:Some(Box::new(source)),preparation:None,item_closing:false,stage:None,group_preparation:None,retained_checkpoint:Default::default(),receipt:None,attempts:0,published:false,cancel_requested:false,close_started:false,fault:None,refused_base:None,refused_authority:None,refused_issuers:[None,None],phase:ArtifactStoreOneItemPublicationPhase::Preparing,verb,transaction:transaction.take(),outbound,announce_from:0,announce_items:0,transaction_open:false};
        Ok(Some((publication,RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0})))
    }

    pub fn begin_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, lane, factory, false, transaction)
    }

    /// 📤️ Admits ONE gesture exactly like [`Self::begin_apply_batch`], but on a store with an attached
    /// backbone: the one edit it publishes stays unannounced until [`Self::flush_published_apply_batch`]
    /// sends it as ONE `BackboneMessage::Mutations` batch, and the publication refuses its ACK before.
    /// See `📋️tool-run-contract.md` §2.7.4 (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS).
    #[allow(clippy::too_many_arguments)]
    pub fn begin_outbound_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_typed_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, factory, true, transaction)
    }

    #[allow(clippy::too_many_arguments)]
    fn begin_typed_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
        lane: HistoryLane,
        factory: Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P, Mutation>>>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        let Some(factory) = factory else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched publication requires an explicit app-owned ArtifactStoreOneItemPreparationFactory".into(), mutations });
        };
        let Some(retirement) = store_owners.mutation_retirement_factory.as_ref() else { return Err(ArtifactStoreBatchAdmissionRejected { reason: "batched inputs require their original mutation retirement authority".into(), mutations }); };
        let source = ArtifactStoreBatchSourceOf::new(ArtifactTypedBatchItemAuthority { preparation: Arc::clone(factory), retirement: Arc::clone(retirement) }, mutations, None, store_log.operation_wire);
        self.begin_apply_batch_owned(store_log, store_wire, operation, expected_generation, expected_revision, actor, lane, None, source, outbound, transaction).map_err(|rejected| {
            let (reason, mutations) = rejected.into_owners();
            ArtifactStoreBatchAdmissionRejected { reason, mutations }
        })
    }

    /// 🎟️ Borrows the original installed catalog and publication inputs under the caller's full turn.
    pub fn admit_member_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, operation:semio_framework_job::OperationId, expected_generation:u64, expected_revision:[u8;32],
        actor:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
        mutations:&mut Vec<Mutation>, transaction:&mut Option<protocol::TransactionRef>, grant:RetainedCloneGrant,
    )->Result<Option<(ArtifactStoreBatchPublication<P,Mutation>,RetainedCloneProgress)>,ValueError>
    where P:Sync,Mutation:Send {
        self.admit_apply_batch(store_log, store_owners, store_wire, operation,expected_generation,expected_revision,actor,mutations,HistoryLane::Document,store_owners.one_item_preparation_factory.as_ref(),transaction,None,false,grant)
    }

    /// 🧩 Uses the exact typed factory installed by the member's owner catalog.
    pub fn begin_member_apply_batch(
        &self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        mutations: Vec<Mutation>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<Mutation>>
    where
        P: Sync,
        Mutation: Send,
    {
        self.begin_apply_batch(store_log, store_owners, store_wire, operation, expected_generation, expected_revision, actor, mutations, HistoryLane::Document, store_owners.one_item_preparation_factory.as_ref(), None)
    }

    /// ⏰️ A stamped clock is compared against the document's OWN history, not against the
    /// wall clock this Store happened to be constructed at. `clock` is seeded
    /// `HybridLogicalTimestamp::new(0, now_ms())` at construction, so a Store that has published
    /// no edit of its own carries a machine fact, not a document fact, and rejecting a stamp
    /// against it refused every first publication whose stamp was minted anywhere but this
    /// process's `now_ms()`. Once this Store has published an edit, `clock` IS the last edit's
    /// stamp (the commit path assigns `self.clock = next_clock`), and strictly-after is then
    /// exactly the hybrid-logical invariant the ordering depends on.
    ///
    /// 🔏 The stamped identity travels with the stamped clock: a durable committer hashes its
    /// canonical command over BOTH before the Store is reached, so an authority that minted its
    /// own content-addressed `edit_id()` published an edit that no verifier could bind back to
    /// the approval it executed. It is admitted under the same fixed identity capacity every
    /// other Store identity answers to.
    #[allow(clippy::too_many_arguments)]
    pub fn begin_apply_batch_owned<A>(
        &self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>,
        operation: semio_framework_job::OperationId,
        expected_generation: u64,
        expected_revision: [u8; 32],
        actor: semio_framework_value::SharedUtf8,
        lane: HistoryLane,
        group_id: Option<String>,
        source: ArtifactStoreBatchSourceOf<P, Mutation, A>,
        outbound: bool,
        transaction: Option<protocol::TransactionRef>,
    ) -> Result<ArtifactStoreBatchPublication<P, Mutation>, ArtifactStoreBatchAdmissionRejected<A::Input>>
    where
        P: Sync + Send,
        A: ArtifactStoreBatchItemAuthority<P, Mutation> + 'static,
        Mutation: 'static,
        P: 'static,
    {
        let reject = |reason: String, source: ArtifactStoreBatchSourceOf<P, Mutation, A>| ArtifactStoreBatchAdmissionRejected { reason, mutations: source.inputs.into() };
        if source.inputs.is_empty() {
            return Err(reject("batched publication requires at least one admitted mutation owner".into(), source));
        }
        if store_wire.durable_group_root.is_some() {
            return Err(reject("batched publication cannot interleave with an unresolved durable group decision".into(), source));
        }
        if let Some(open) = store_log.envelope.open_transaction.as_ref().filter(|open| transaction.as_ref().is_none_or(|transaction| transaction.id != open.transaction.id)) {
            return Err(reject(format!("batched publication cannot interleave with the open tool transaction {}", open.transaction.id), source));
        }
        if self.generation != expected_generation || self.content_revision != expected_revision {
            return Err(reject("batched publication base generation or revision is stale".into(), source));
        }
        if lane != HistoryLane::Document {
            return Err(reject("batched publication has no retained side-lane map preparation authority".into(), source));
        }
        if store_wire.backbone.is_some() && !outbound {
            return Err(reject("batched publication has no retained outbound backbone encoder/sender authority".into(), source));
        }
        if actor.is_empty() || actor.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(reject("batched publication actor exceeds its fixed identity capacity".into(), source));
        }
        if group_id.as_ref().is_some_and(|id| id.is_empty() || id.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err(reject("batched publication group exceeds its fixed identity capacity".into(), source));
        }
        let Some(cursor) = store_log.envelope.cursor.as_ref() else {
            return Err(reject("batched publication requires an initialized live cursor authority".into(), source));
        };
        if cursor.applied_edit_ids.len() != store_log.applied_edit_ids.len() || cursor.redo_edit_ids.len() != store_log.redo_edit_ids.len() || cursor.checkpoint_id.as_deref() != store_log.current_checkpoint_id.as_deref() {
            return Err(reject("batched publication live cursor is stale or lacks preinstalled fixed capacity".into(), source));
        }
        let next_sequence_number = match store_log.edit_sequence.checked_add(1) {
            Some(value) => value,
            None => return Err(reject("batched edit sequence exhausted".into(), source)),
        };
        let mut next_clock = store_log.clock;
        next_clock.tick(now_ms());
        match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_clock) {
            Some(stamped) if store_log.edit_sequence == 0 || (stamped.physical_ms, stamped.logical) > (store_log.clock.physical_ms, store_log.clock.logical) => next_clock = stamped,
            Some(_) => return Err(reject("stamped publication clock is not strictly after this Store's own last published edit".into(), source)),
            None => {}
        }
        let stamped_edit_id = match source.authority.as_ref().and_then(ArtifactStoreBatchItemAuthority::stamped_mutation_id) {
            Some(identity) if identity.0.is_empty() || identity.0.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES => {
                return Err(reject("stamped publication identity exceeds its fixed identity capacity".into(), source));
            }
            Some(identity) => Some(identity.0.clone()),
            None => None,
        };
        let admitted_items = source.inputs.len();
        let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint_for_store(
            store_log.mint_witness(),
            operation,
            semio_framework_job::Generation(expected_generation),
            expected_revision,
            store_log.applied_edit_ids.len(),
            next_sequence_number,
            next_clock,
            actor,
            store_log.envelope.active_alternative_id.clone(),
            group_id,
            stamped_edit_id,
        ));
        Ok(ArtifactStoreBatchPublication {
            operation,
            expected_generation,
            expected_revision,
            lane,
            footprint: ArtifactStoreOneItemFootprint { work_items: 0, retained_bytes: 0 },
            source_census_completed: 0,
            source_census_finished: false,
            admitted_items,
            authority: Some(authority),
            authority_retirement: None,
            source: Some(Box::new(source)),
            preparation: None,
            item_closing: false,
            stage: None,
            group_preparation: None,
            retained_checkpoint: ArtifactStoreOneItemCheckpoint::default(),
            receipt: None,
            attempts: 0,
            published: false,
            cancel_requested: false,
            close_started: false,
            fault: None,
            refused_base: None,
            refused_authority: None,
            refused_issuers: [None, None],
            phase: ArtifactStoreOneItemPublicationPhase::Preparing,
            verb: None,
            transaction,
            outbound,
            announce_from: 0,
            announce_items: 0,
            transaction_open: false,
        })
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🎴️ Takes what a finished Report replay publishes — its messages and reached projection — and installs its unit flags
    /// and rebased inverses; every untaken owner stays attached for [`Self::retire_replay_result`].
    pub fn take_report_replay(&mut self, store_log: &mut StoreLog<P, Mutation>, result: &mut EditReplayResult<P, Mutation>) -> Result<(Vec<semio_framework_os_spr::EditMessages>, Arc<P>), VcsError> {
        self.revision_accumulator.unit_flags.extend(std::mem::take(&mut result.unit_flags));
        let replayed = result.take_replayed();
        if let Some(entry) = replayed.iter().find(|entry| entry.edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES) {
            return Err(VcsError::ValidationFailed(format!("replayed edit id {} exceeds its fixed byte authority", entry.edit_id)));
        }
        let state = result.take_state().ok_or_else(|| VcsError::ValidationFailed("report replay lost its reached projection".into()))?;
        for (edit_id, inverse) in result.take_rebased_undo_rows() {
            match store_log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id) {
                Some(edit) => retire_scratch_operations::<P, Mutation>(std::mem::replace(&mut edit.inverse, inverse)),
                None => retire_scratch_operations::<P, Mutation>(inverse),
            }
        }
        Ok((replayed, state))
    }

    //#endregion 🔖️EventSourcedHistory
    /// 🖍️ The applied tail edit's recorded content changed outside a mutation (a group or origin stamp): the next
    /// {@link bump} rebuilds its revision record.
    pub fn restamp_applied_tail(&mut self, store_log: &mut StoreLog<P, Mutation>) {
        self.mirror_dirt.dirty_applied_from(store_log.applied_edit_ids.len().saturating_sub(1));
    }

    /// ➕️ Pushes `edit_id` onto the applied stack; its mirrors are stale from the new tail on.
    pub fn push_applied_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: String) {
        self.mirror_dirt.dirty_applied_from(store_log.applied_edit_ids.len());
        store_log.applied_edit_ids.push(edit_id);
    }

    /// ➕️ Pushes `edit_id` onto the redo stack; its mirrors are stale from the new top on.
    pub fn push_redo_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: String) {
        self.mirror_dirt.dirty_redo_from(store_log.redo_edit_ids.len());
        store_log.redo_edit_ids.push(edit_id);
    }

    /// ➖️ Pops the redo top; its mirrors are stale past the new length.
    pub fn pop_redo_edit_id(&mut self, store_log: &mut StoreLog<P, Mutation>) -> Option<String> {
        let edit_id = store_log.redo_edit_ids.pop()?;
        self.mirror_dirt.dirty_redo_from(store_log.redo_edit_ids.len());
        Some(edit_id)
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
