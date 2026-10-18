//! 📸️ Snapshot reads and retained replacements: the projection event, the derived snapshots and history previews, the snapshot reads and the retained replacement of roots, edit ids, checkpoints and the tail undo cache. See [`super`].

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
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 📬️ Captures one immutable projection or inference input from the current reconciled snapshot.
    pub fn projection_event<Previous, Policy>(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<P, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: store_cursor.projection_stamp(store_log, store_wire), cause, state: self.snapshot(store_wire)?, previous, cache_mode, policy })
    }

    /// ⚡️ The live snapshot: the incrementally-maintained `current` fold, as-is. Always `Ok`
    /// in practice (kept as `Result` for API stability); O(1) instead of a full replay. See the
    /// `current` field doc for the maintenance invariant.
    pub fn snapshot(&self, store_wire: &StoreWire<P, Mutation>) -> Result<P, VcsError> {
        Ok(store_wire.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref()).clone())
    }

    /// 🔎️ Borrows the current immutable fold without issuing or cloning a snapshot owner.
    pub fn snapshot_ref<'a>(&'a self, store_wire: &'a StoreWire<P, Mutation>) -> &'a P {
        store_wire.durable_group_read_root().map_or(self.current.as_ref(), |root| root.current.as_ref().expect("unadopted durable group root retains its current snapshot").as_ref())
    }

    /// 🌱️ Captures the current Store projection as the base of a provisional member fold.
    pub fn derived_snapshot_head(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> ArtifactDerivedSnapshot<P> {
        store_cursor.seal_derived_snapshot(store_owners, store_wire, self.snapshot_owner(store_wire))
    }

    /// 🎞️ Derives a history-edit preview base from this Store's actual history and accepted draft inputs.
    pub fn derived_snapshot_before(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError> {
        let owner = self.state_before(store_log, store_owners, store_wire, store_cursor, target, drafts)?;
        Ok(store_cursor.seal_derived_snapshot(store_owners, store_wire, owner))
    }

    /// ⏭️ Resolves identities or folds at most one operation between deadline checks; downstream operations are excluded.
    pub fn step_derived_history_preview(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        store_cursor.admit_derived_source(store_owners, store_wire, &preview.registry, preview.generation, preview.revision)?;
        if preview.plan_retirement.is_some() {
            preview.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if let Some(plan) = preview.plan.as_mut() {
            if !self.step_history_read_plan(store_log, store_wire, store_cursor, plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let live_until=preview.retain_completed_plan();
            let (start, base) = self.live_base(store_log, store_cursor, live_until);
            preview.cursor = start;
            preview.state = Some(base);
            return Ok(ReplayStep::Pending(preview.progress));
        }
        let (position, op_index) = preview.target.ok_or_else(|| VcsError::ValidationFailed("history preview lost its target".into()))?;
        while !preview.finished {
            if preview.cursor == position && preview.operation == op_index {
                preview.finished = true;
                break;
            }
            let edit = store_cursor.applied_edit(store_log, store_wire, preview.cursor)?;
            if preview.operation == edit.forwards.len() {
                preview.cursor += 1;
                preview.operation = 0;
                continue;
            }
            let index = preview.operation;
            if index == 0 {
                let count = if preview.cursor == position { op_index } else { edit.forwards.len() };
                preview.progress.total = preview.progress.total.saturating_add(u32::try_from(count).unwrap_or(u32::MAX));
            }
            let id = semio_framework_os_spr::mutation_id_for_edit_operation::<P, Mutation>(edit, index).expect("recorded operation identity");
            let draft = preview.drafts.get(&id).map(|replacement| protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() });
            let effective = effective_operation_with_supersession::<P, Mutation>(&edit.forwards[index], index as u32, id.clone(), &store_log.envelope.schema, draft.as_ref().or_else(|| self.supersessions.get(&id)));
            if let Some(operation) = effective.operation() {
                let running = preview.state.as_mut().expect("preview running projection");
                if let (Some(next), _) = fold_operation::<P, Mutation>(running, operation, index as u32) {
                    <Arc<P> as FoldProjection<P, Mutation>>::advance(running, next);
                }
            }
            preview.operation += 1;
            preview.progress.done = preview.progress.done.saturating_add(1);
            if deadline() {
                return Ok(ReplayStep::Pending(preview.progress));
            }
        }
        Ok(ReplayStep::Finished(preview.progress))
    }

    /// ⏭️ Steps sealed planning and replay against this store's actual immutable frontier.
    pub fn step_derived_report_replay(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        store_cursor.admit_derived_source(store_owners, store_wire, &owner.registry, owner.generation, owner.revision)?;
        if owner.plan_retirement.is_some() {
            owner.retire_completed_plan(retirement_grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
            return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())));
        }
        if let Some(plan) = owner.plan.as_mut() {
            if !self.step_history_read_plan(store_log, store_wire, store_cursor, plan, deadline)? {
                return Ok(ReplayStep::Pending(plan.progress));
            }
            let generation=owner.generation;let revision=owner.revision;
            let plan=owner.retain_completed_plan();
            let from = plan.first_input;
            let horizon = plan.last_draft;
            let (start, base) = self.live_base(store_log, store_cursor, from);
            let total = plan.operations.saturating_sub(plan.counts[from]);
            let mut replay = EditReplay::new_counted(ReplayMode::Report, base, std::mem::replace(&mut plan.order,semio_framework_os_vcs::HistoryPageStack::empty()), from, &store_log.envelope.schema, std::mem::take(&mut plan.supersessions), total)?;
            replay.cursor = start;
            replay.prefix_work_progress = true;
            replay.progress.total = replay.progress.total.saturating_add(plan.counts[from].saturating_sub(plan.counts[start]));
            replay.progress.done = plan.progress.done;
            replay.progress.total = replay.progress.total.saturating_add(plan.progress.done);
            replay = replay.with_base(generation, revision).with_drafts(std::mem::take(&mut plan.drafts));
            replay.operation_preparation_factory = (*store_owners.replay_preparation_factory).clone();
            if let (Some(snapshots), Some(mutations)) = ((*store_owners.initial_snapshot_retirement_factory).clone(), (*store_owners.mutation_retirement_factory).clone()) {
                replay = replay.with_retirement_factories(snapshots, mutations);
            } else if replay.operation_preparation_factory.is_some() {
                let _ = replay.retire_unscheduled();
                return Err(VcsError::ValidationFailed("interactive operation preparation requires exact replay retirement factories".into()));
            }
            if let (Some(horizon), Some(equal)) = (horizon, self.snapshot_equality) {
                match self.convergence_checkpoints(store_log, store_cursor, horizon, equal) {
                    Ok(checkpoints) => replay = replay.with_convergence(Some(checkpoints)),
                    Err(error) => {
                        let _ = replay.retire_unscheduled();
                        return Err(error);
                    }
                }
            }
            owner.replay = Some(replay);
            return Ok(ReplayStep::Pending(owner.replay.as_ref().expect("planned replay").progress()));
        }
        owner.replay.as_mut().ok_or_else(|| VcsError::ValidationFailed("derived replay lost its cursor".into()))?.step(&IndexedAppliedReplayEdits { edits: &store_log.envelope.vcs.edits, revision: &store_cursor.revision_accumulator, ids: &store_log.applied_edit_ids }, deadline)
    }

    fn step_history_read_plan(&self, store_log: &StoreLog<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>, plan: &mut HistoryReadPlan, deadline: &mut dyn FnMut() -> bool) -> Result<bool, VcsError> {
        use std::ops::Bound::{Excluded, Unbounded};
        if plan.progress.total == 0 {
            let work = usize::from(plan.target.is_some()).saturating_add(plan.drafts.len()).saturating_add(if plan.report { store_log.applied_edit_ids.len().saturating_add(self.supersessions.len()).saturating_add(plan.drafts.len()) } else { 0 });
            plan.progress.total = u32::try_from(work).unwrap_or(u32::MAX);
        }
        while !plan.finished {
            if !plan.located_target {
                plan.located_target = true;
                if let Some(target) = plan.target.as_ref() {
                    let position = store_cursor.locate_mutation(store_log, store_wire, target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.positions.insert(target.clone(), position);
                } else { continue; }
            } else if !plan.located_inputs {
                let next = match plan.located_draft.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }.map(|(target, _)| target.clone());
                if let Some(target) = next {
                    let position = store_cursor.locate_mutation(store_log, store_wire, &target)?;
                    plan.first_input = plan.first_input.min(position.0);
                    plan.last_draft = Some(plan.last_draft.unwrap_or(0).max(position.0 + 1));
                    plan.positions.insert(target.clone(), position);
                    plan.located_draft = Some(target);
                } else {
                    plan.located_inputs = true;
                    if !plan.report { plan.finished = true; }
                    continue;
                }
            } else if plan.cursor < store_log.applied_edit_ids.len() {
                let edit = store_cursor.applied_edit(store_log, store_wire, plan.cursor)?;
                plan.counts.push(plan.operations);
                plan.operations = plan.operations.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));
                plan.order.push(edit.id.clone());
                plan.cursor += 1;
            } else {
                let next = if plan.copying_drafts {
                    match plan.copied.as_ref() { Some(key) => plan.drafts.range((Excluded(key), Unbounded)).next(), None => plan.drafts.iter().next() }
                        .map(|(target, replacement)| (target.clone(), protocol::EffectiveSupersession { transition_id: DRAFT_SUPERSESSION_ID.into(), actor: semio_framework_value::SharedUtf8::default(), timestamp: HybridLogicalTimestamp::new(0, 0), scope: None, replacement: replacement.clone() }))
                } else {
                    match plan.copied.as_ref() { Some(key) => self.supersessions.first_entry_after(key), None => self.supersessions.iter().next() }.map(|(target, supersession)| (target.clone(), supersession.clone()))
                };
                if let Some((target, supersession)) = next {
                    plan.copied = Some(target.clone());
                    plan.supersessions.insert(target, supersession);
                } else if !plan.copying_drafts {
                    plan.copying_drafts = true;
                    plan.copied = None;
                    continue;
                } else { plan.finished = true; continue; }
            }
            plan.progress.done = plan.progress.done.saturating_add(1);
            if deadline() { return Ok(false); }
        }
        if plan.report && plan.counts.len() == store_log.applied_edit_ids.len() { plan.counts.push(plan.operations); }
        Ok(true)
    }

    /// 🧵️ Immutable O(1) snapshot capability for worker and composition boundaries.
    pub fn snapshot_read(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        let generation = store_cursor.generation(store_wire);
        let content_revision = store_cursor.content_revision(store_wire);
        if !store_owners.snapshot_read_leases.publish_authority(generation, content_revision) {
            return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()));
        }
        let owner = self.snapshot_owner(store_wire);
        let lease = store_owners.snapshot_read_leases.try_issue(owner.clone()).map_err(|_| VcsError::ValidationFailed("snapshot read lease registry is busy, saturated, or exhausted".into()))?;
        Ok(SnapshotRead::new(owner, lease))
    }

    pub fn snapshot_owner(&self, store_wire: &StoreWire<P, Mutation>) -> Arc<P> {
        store_wire.durable_group_read_root().map_or_else(|| Arc::clone(&*self.current), |root| Arc::clone(root.current.as_ref().expect("unadopted durable group root retains its current snapshot")))
    }

    /// 🧵️ Captures the immutable event-maintained snapshot root in O(1).
    pub fn snapshot_root(&self, store_wire: &StoreWire<P, Mutation>) -> Arc<P> {
        self.snapshot_owner(store_wire)
    }

    pub fn replace_current_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Arc<P>) -> Result<(), VcsError> {
        if Arc::ptr_eq(&self.current, &next) {
            return Ok(());
        }
        let next = ScratchOwner::new(next, retire_shared_projection::<P, Mutation>);
        let previous_is_retained = !self.current_detached && (self.tail_undo_cache.as_ref().is_some_and(|(_, snapshot)| Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(self.current.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()));
        let factory = if previous_is_retained {
            None
        } else {
            store_owners.displaced_retirements.reserve(1)?;
            Some((*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store current replacement requires its exact snapshot retirement factory".into()))?)
        };
        let previous = std::mem::replace(&mut *self.current, next.adopt());
        if previous_is_retained {
            drop(previous);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous, factory.expect("non-tail current factory was validated")));
        }
        Ok(())
    }

    fn replace_document_roots_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next_envelope: ArtifactEnvelope<P, Mutation>, next_current: Arc<P>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(2)?;
        let initial_snapshot_factory = (*store_owners.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact initial-snapshot retirement factory".into()))?;
        let mutation_factory = (*store_owners.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact mutation retirement factory".into()))?;
        let snapshot_factory = (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root replacement requires its exact snapshot retirement factory".into()))?;
        let previous_envelope = std::mem::replace(&mut *store_log.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, initial_snapshot_factory, mutation_factory)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, snapshot_factory));
        }
        Ok(())
    }

    pub fn prepare_document_root_commit(&mut self, store_owners: &mut StoreOwners<P, Mutation>, additional_displaced_owners: usize) -> Result<ArtifactStoreDocumentRootCommitAuthority<P, Mutation>, VcsError> {
        let tail_slots = self.tail_undo_cache.as_ref().map_or(0, |(edit_id, snapshot)| usize::from(!edit_id.is_empty()) + usize::from(self.current_detached || !Arc::ptr_eq(snapshot, &self.current)));
        store_owners.displaced_retirements
            .reserve(3usize.checked_add(tail_slots).and_then(|slots| slots.checked_add(additional_displaced_owners)).ok_or_else(|| VcsError::ValidationFailed("artifact store document-root retirement reservation overflowed".into()))?)?;
        Ok(ArtifactStoreDocumentRootCommitAuthority {
            snapshot: (*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact snapshot retirement factory".into()))?,
            initial_snapshot: (*store_owners.initial_snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact initial-snapshot retirement factory".into()))?,
            mutation: (*store_owners.mutation_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store document-root transaction requires its exact mutation retirement factory".into()))?,
        })
    }

    pub fn commit_document_roots_retained(
        &mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>,
        next_envelope: ArtifactEnvelope<P, Mutation>,
        next_current: Arc<P>,
        next_dag: semio_framework_os_spr::MutationDag,
        next_tail: Option<(String, Arc<P>)>,
        authority: ArtifactStoreDocumentRootCommitAuthority<P, Mutation>,
    ) {
        if let Some((edit_id, snapshot)) = self.tail_undo_cache.take() {
            if !edit_id.is_empty() {
                store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
            }
            if (!self.current_detached && Arc::ptr_eq(&snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()) {
                drop(snapshot);
            } else {
                store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, Arc::clone(&authority.snapshot)));
            }
        }
        let previous_envelope = std::mem::replace(&mut *store_log.envelope, next_envelope);
        let previous_current = std::mem::replace(&mut *self.current, next_current);
        let previous_dag = std::mem::replace(&mut *store_log.dag, next_dag);
        let current_is_genesis = std::ptr::eq(previous_current.as_ref(), previous_envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreEnvelopeRetirement::new(previous_envelope, authority.initial_snapshot, authority.mutation)));
        if current_is_genesis {
            drop(previous_current);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(previous_current, Arc::clone(&authority.snapshot)));
        }
        store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(previous_dag)));
        *self.tail_undo_cache = next_tail;
    }

    pub fn string_vector_owner_slots(values: &semio_framework_os_vcs::HistoryPageStack<String>) -> usize {
        usize::from(!values.is_empty() || values.capacity() != 0)
    }

    pub fn string_owner_slots(value: &Option<String>) -> usize {
        usize::from(value.is_some())
    }

    pub fn take_string_vector_replacement(target: &mut std::mem::ManuallyDrop<semio_framework_os_vcs::HistoryPageStack<String>>, next: semio_framework_os_vcs::HistoryPageStack<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        (!previous.is_empty() || previous.capacity() != 0).then(|| Box::new(ArtifactStoreStringVectorRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    pub fn take_string_replacement(target: &mut std::mem::ManuallyDrop<Option<String>>, next: Option<String>) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        let previous = std::mem::replace(&mut **target, next);
        previous.map(|previous| Box::new(ArtifactStoreStringRetirement::new(previous)) as Box<dyn ErasedSnapshotRetirement>)
    }

    /// 🔁️ Replaces the applied stack. From the first position whose id changed, the persisted cursor and the revision
    /// records are stale ([`StackMirrorDirt`]), the prefix ring too, and the live cursor is no longer a proven fold.
    pub fn replace_applied_edit_ids_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&store_log.applied_edit_ids, &next);
        let changed = common != store_log.applied_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        store_owners.displaced_retirements.reserve(Self::string_vector_owner_slots(&store_log.applied_edit_ids))?;
        if changed {
            store_cursor.mirror_dirt.dirty_applied_from(common);
            self.prefix_ring_dirty_from = self.prefix_ring_dirty_from.min(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut store_log.applied_edit_ids, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    /// 🔂️ Replaces the redo stack; its mirrors are stale from the first position whose id changed.
    pub fn replace_redo_edit_ids_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, next: Vec<String>) -> Result<(), VcsError> {
        let common = shared_prefix_len(&store_log.redo_edit_ids, &next);
        let changed = common != store_log.redo_edit_ids.len() || common != next.len();
        let next = ArtifactStoreInitializationOwnerCatalog::retain_id_capacity(next);
        store_owners.displaced_retirements.reserve(Self::string_vector_owner_slots(&store_log.redo_edit_ids))?;
        if changed {
            store_cursor.mirror_dirt.dirty_redo_from(common);
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_vector_replacement(&mut store_log.redo_edit_ids, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    pub(super) fn replace_current_checkpoint_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Option<String>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(Self::string_owner_slots(&store_log.current_checkpoint_id))?;
        if next != *store_log.current_checkpoint_id {
            self.fold_frontier = None;
        }
        if let Some(owner) = Self::take_string_replacement(&mut store_log.current_checkpoint_id, next) {
            store_owners.displaced_retirements.push_reserved(owner);
        }
        Ok(())
    }

    pub fn insert_reserved_edit_history(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, mut reservation: ArtifactStoreHistoryCommitReservation, edit: Edit<Mutation>) -> Result<ArtifactHistoryKey, VcsError> {
        let factory = (*store_owners.mutation_retirement_factory).clone().expect("edit history admission validated its exact mutation retirement factory");
        self.fold_frontier = None;
        match store_log.envelope.vcs.edits.insert_reserved(reservation.history, edit) {
            Ok(key) => {
                store_owners.displaced_retirements.release_owner_slots(reservation.rejected_owner)?;
                Ok(key)
            }
            Err((_, edit)) => {
                let owner = Box::new(ArtifactStoreDecodedEditRetirement::new(edit, factory));
                if let Err(owner) = store_owners.displaced_retirements.push_owner_reserved(&mut reservation.rejected_owner, owner) {
                    store_owners.displaced_retirements.push_reserved(owner);
                }
                Err(VcsError::ValidationFailed("edit history reservation is stale".into()))
            }
        }
    }

    pub fn replace_tail_undo_cache_retained(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, next: Option<(String, Arc<P>)>) -> Result<(), VcsError> {
        let Some((edit_id, snapshot)) = self.tail_undo_cache.as_ref() else {
            *self.tail_undo_cache = next;
            return Ok(());
        };
        let snapshot_is_retained = (!self.current_detached && Arc::ptr_eq(snapshot, &self.current)) || std::ptr::eq(snapshot.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot());
        store_owners.displaced_retirements.reserve(usize::from(!edit_id.is_empty()) + usize::from(!snapshot_is_retained))?;
        let factory =
            if snapshot_is_retained { None } else { Some((*store_owners.snapshot_retirement_factory).clone().ok_or_else(|| VcsError::ValidationFailed("artifact store tail replacement requires its exact snapshot retirement factory".into()))?) };
        let (edit_id, snapshot) = self.tail_undo_cache.take().expect("validated tail undo owner remains present");
        if !edit_id.is_empty() {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(edit_id)));
        }
        if snapshot_is_retained {
            drop(snapshot);
        } else {
            store_owners.displaced_retirements.push_reserved(ArtifactStoreDisplacedSnapshotRetirement::boxed(snapshot, factory.expect("nonshared tail snapshot factory was validated")));
        }
        *self.tail_undo_cache = next;
        Ok(())
    }

    pub fn close_cursor_demands(&self, store_log: &StoreLog<P, Mutation>, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, phase: ArtifactStoreCursorDisposerPhase, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let birth = |capacity_bytes| RetirementDemand { capacity_bytes, depth: 2, ..Default::default() };
        let structural = RetirementDemand { depth: 1, ..Default::default() };
        Ok(match phase {
            ArtifactStoreCursorDisposerPhase::ReturnedReads if store_owners.snapshot_read_leases.has_returned() => store_owners.snapshot_read_leases.returned_admission_demands::<P>(|_| birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes())).map_err(SnapshotReadLeaseRefusal::into_value_error)?,
            ArtifactStoreCursorDisposerPhase::Displaced if !store_owners.maintenance_retirements_terminal_is_empty() => store_owners.maintenance_retirements_demands(body)?,
            ArtifactStoreCursorDisposerPhase::RuntimeOwners(lane) => birth(match lane {
                0 | 1 => size_of::<ArtifactStoreStringVectorRetirement>(),
                2 => size_of::<ArtifactStoreRevisionAccumulatorRetirement>(),
                3 | 5 => size_of::<ArtifactStoreStringRetirement>(),
                4 => size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>(),
                6 => size_of::<ArtifactStoreSupersessionRetirement>(),
                _ => size_of::<ArtifactStorePendingReportRetirement>(),
            }),
            ArtifactStoreCursorDisposerPhase::PrefixSnapshots => match self.prefix_ring.last() {
                Some(entry) if Arc::strong_count(&entry.snapshot) == 1 => birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "prefix retirement requires exact snapshot factory"))?.retirement_birth_bytes(&entry.snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::TailSnapshot => match self.tail_undo_cache.as_ref() {
                Some((_, snapshot)) if Arc::strong_count(snapshot) == 1 => birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "tail retirement requires exact snapshot factory"))?.retirement_birth_bytes(snapshot)),
                _ => structural,
            },
            ArtifactStoreCursorDisposerPhase::CurrentSnapshot if !self.current_detached => {
                if std::ptr::eq(self.current.as_ref(), store_log.envelope.vcs.genesis.facts().snapshot()) { birth(ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes()) }
                else { birth(store_owners.snapshot_retirement_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "current retirement requires exact snapshot factory"))?.retirement_birth_bytes(&self.current)) }
            }
            ArtifactStoreCursorDisposerPhase::Backbone if store_wire.member_inbox.capacity() != 0 || store_wire.backbone.is_some() => birth(ArtifactStoreBackboneRetirement::constructor_capacity_bytes()),
            ArtifactStoreCursorDisposerPhase::Causal => birth(size_of::<ArtifactStoreMutationDagRetirement>()),
            ArtifactStoreCursorDisposerPhase::Envelope if !store_log.envelope_detached => birth(size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()),
            ArtifactStoreCursorDisposerPhase::Complete => Default::default(),
            _ => structural,
        })
    }

    /// 🔂️ Full raw fold of `initial_snapshot` over every `forwards` op in `applied_edit_ids`
    /// order, WITHOUT the final `Mutation::reconcile` pass — the from-scratch computation `current`
    /// is an incrementally-maintained cache of. Used to recompute `current` on the cold paths that
    /// reassign `applied_edit_ids` wholesale instead of appending/popping its tail.
    pub async fn fold_current(&self, store_log: &StoreLog<P, Mutation>) -> Result<P, VcsError> {
        fold_history(&store_log.envelope, &*store_log.applied_edit_ids, &self.supersessions).await
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
