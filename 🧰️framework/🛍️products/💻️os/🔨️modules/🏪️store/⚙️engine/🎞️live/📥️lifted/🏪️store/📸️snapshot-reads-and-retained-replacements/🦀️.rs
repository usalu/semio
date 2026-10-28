//! 📸️ Snapshot reads and retained replacements: the projection event, the derived snapshots and history previews, the snapshot reads and the retained replacement of roots, edit ids, checkpoints and the tail undo cache. See [`super`].

use super::*;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_os_spr::ColdMutation;

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
    P: ToValue + Send + Sync + 'static,
    Mutation: ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 📬️ Captures one immutable projection or inference input from the current reconciled snapshot.
    

    /// ⚡️ The live snapshot: the incrementally-maintained `current` fold, as-is. Always `Ok`
    /// in practice (kept as `Result` for API stability); O(1) instead of a full replay. See the
    /// `current` field doc for the maintenance invariant.
    pub fn snapshot(&self, store_wire: &StoreWire<P, Mutation>) -> Result<P, VcsError>  where P: Clone {
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
    pub fn derived_snapshot_before(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>, store_wire: &mut StoreWire<P, Mutation>, store_cursor: &mut StoreCursor<P, Mutation>, target: &MutationId, drafts: &protocol::HistoryInputDrafts) -> Result<ArtifactDerivedSnapshot<P>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        let owner = self.state_before(store_log, store_owners, store_wire, store_cursor, target, drafts)?;
        Ok(store_cursor.seal_derived_snapshot(store_owners, store_wire, owner))
    }

    /// ⏭️ Resolves identities or folds at most one operation between deadline checks; downstream operations are excluded.
    

    /// ⏭️ Steps sealed planning and replay against this store's actual immutable frontier.
    

    fn selected_history_edit<'a>(&self,store_log:&'a StoreLog<P,Mutation>,store_wire:&StoreWire<P,Mutation>,store_cursor:&StoreCursor<P,Mutation>,selection:Option<semio_framework_os_vcs::OriginalHistoryEditSelection>,position:usize)->Result<&'a Edit<Mutation>,VcsError>{match selection{Some(selection)=>match selection.original_position(position){None=>store_log.envelope.vcs.edits.get_key(selection.key).ok_or(VcsError::HistoryReplaying),Some(position)=>store_cursor.applied_edit(store_log,store_wire,position)},None=>store_cursor.applied_edit(store_log,store_wire,position)}}

    pub fn step_history_read_plan(&self,store_log:&StoreLog<P,Mutation>,store_wire:&StoreWire<P,Mutation>,store_cursor:&StoreCursor<P,Mutation>,plan:&mut HistoryReadPlan,order:Option<&semio_framework_os_vcs::HistoryPageStack<semio_framework_os_vcs::ArtifactHistoryKey>>,grant:RetainedCloneGrant,control:&mut semio_framework_value::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<(bool,RetainedCloneProgress),VcsError>{
        use semio_framework_value::{RetirementDemand,ValueRefusalKind};
        if plan.original_order!=order.map(|o|(o as*const _ as usize,o.len())){return Err(VcsError::HistoryReplaying)}let idle=RetainedCloneProgress::default();if plan.finished{return Ok((true,idle))}if deadline()||grant.maximum_items==0{return Ok((false,idle))}
        let metadata=2*size_of::<usize>()+size_of::<u8>()+size_of::<ReplayProgress>();
        let mut demand=RetirementDemand{copy_bytes:metadata,depth:1,..Default::default()};
        macro_rules! admit {()=>{if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok((false,idle))}control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(demand.capacity_bytes).map_err(VcsError::InverseRefused)?;}}
        macro_rules! reserve {($index:expr,$key:expr)=>{{let index=&mut $index;let key=$key;let capacity=index.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes).map_err(VcsError::InverseRefused)?;if capacity!=0{let depth=index.next_insert_depth_demand(key).map_err(VcsError::InverseRefused)?;let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(metadata),..grant};if grant.maximum_copy_bytes<metadata||grant.maximum_capacity_bytes<capacity||grant.maximum_depth<depth{return Ok((false,idle))}control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(capacity).map_err(VcsError::InverseRefused)?;let mut p=index.reserve_insert_step(key,child).map_err(|(error,_)|VcsError::InverseRefused(error))?;if p.copied_items!=0{p.copied_bytes+=metadata;}return Ok((false,p));}}}}
        match plan.native_phase{
            0=>{admit!();plan.progress.total=u32::try_from(usize::from(plan.target.is_some()).saturating_add(plan.drafts.len()).saturating_add(if plan.report{store_log.applied_edit_ids.len().saturating_add(self.supersessions.len())}else{0})).unwrap_or(u32::MAX);plan.native_phase=1;},
            1=>{if let Some(target)=plan.target.as_ref(){reserve!(plan.positions,target);demand.copy_bytes+=target.0.len()+size_of::<(MutationId,(usize,usize))>();demand.capacity_bytes=target.0.len();demand.depth=demand.depth.max(plan.positions.next_insert_depth_demand(target).map_err(VcsError::InverseRefused)?);admit!();let position=store_cursor.locate_mutation(store_log,store_wire,target)?;plan.positions.insert_reserved(target.clone(),position,grant).map_err(|(error,_,_)|VcsError::InverseRefused(error))?;plan.first_input=plan.first_input.min(position.0);plan.progress.done=plan.progress.done.saturating_add(1);}else{admit!();}plan.located_target=true;plan.native_phase=2;},
            2=>{if let Some((target,_))=plan.drafts.at_rank(plan.native_row){reserve!(plan.positions,target);demand.copy_bytes+=target.0.len()+size_of::<(MutationId,(usize,usize))>();demand.capacity_bytes=target.0.len();demand.depth=demand.depth.max(plan.positions.next_insert_depth_demand(target).map_err(VcsError::InverseRefused)?);admit!();let position=store_cursor.locate_mutation(store_log,store_wire,target)?;plan.positions.insert_reserved(target.clone(),position,grant).map_err(|(error,_,_)|VcsError::InverseRefused(error))?;plan.first_input=plan.first_input.min(position.0);plan.last_draft=Some(plan.last_draft.unwrap_or(0).max(position.0+1));plan.native_row+=1;plan.progress.done=plan.progress.done.saturating_add(1);}else{admit!();plan.located_inputs=true;plan.native_row=0;plan.native_phase=if plan.report{3}else{8};}},
            3=>{if plan.cursor<if let Some(order)=order{order.len()}else{plan.original_selection.map_or(Some(store_log.applied_edit_ids.len()),|selection|selection.selected_len(store_log.applied_edit_ids.len())).ok_or(VcsError::HistoryReplaying)?}{let edit=if let Some(order)=order{store_log.envelope.vcs.edits.get_key(*order.get(plan.cursor).ok_or(VcsError::HistoryReplaying)?).ok_or(VcsError::HistoryReplaying)?}else{self.selected_history_edit(store_log,store_wire,store_cursor,plan.original_selection,plan.cursor)?};demand.copy_bytes+=size_of::<u32>();demand.capacity_bytes=plan.counts.next_push_allocation_bytes();admit!();plan.counts.try_push(plan.operations).map_err(|_|VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native history count catalog is full")))?;plan.operations=plan.operations.saturating_add(u32::try_from(edit.forwards.len()).unwrap_or(u32::MAX));plan.native_phase=4;}else{demand.copy_bytes+=size_of::<u32>();demand.capacity_bytes=plan.counts.next_push_allocation_bytes();admit!();plan.counts.try_push(plan.operations).map_err(|_|VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native history terminal count catalog is full")))?;plan.native_phase=5;}},
            4=>{let edit=if let Some(order)=order{store_log.envelope.vcs.edits.get_key(*order.get(plan.cursor).ok_or(VcsError::HistoryReplaying)?).ok_or(VcsError::HistoryReplaying)?}else{self.selected_history_edit(store_log,store_wire,store_cursor,plan.original_selection,plan.cursor)?};demand.copy_bytes+=edit.id.len()+size_of::<String>();demand.capacity_bytes=plan.order.next_push_allocation_bytes()+edit.id.len();admit!();plan.order.try_push(edit.id.clone()).map_err(|_|VcsError::InverseRefused(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native history order catalog is full")))?;plan.cursor+=1;plan.progress.done=plan.progress.done.saturating_add(1);plan.native_phase=3;},
            5 if plan.original_supersessions=>{admit!();plan.native_phase=6;},
            5=>{if let Some((target,row))=self.supersessions.ranked(plan.native_row){reserve!(plan.supersessions,target);let bytes=target.0.len()+row.transition_id.len()+row.scope.as_ref().map_or(0,String::len)+match &row.replacement{protocol::InputReplacement::Input{schema,payload}=>schema.len()+payload.len(),protocol::InputReplacement::Withdrawn=>0};demand.copy_bytes+=bytes+size_of::<(MutationId,protocol::EffectiveSupersession)>();demand.capacity_bytes=bytes;demand.depth=demand.depth.max(plan.supersessions.next_insert_depth_demand(target).map_err(VcsError::InverseRefused)?);admit!();plan.supersessions.insert_reserved(target.clone(),row.clone(),grant).map_err(|(error,_,_)|VcsError::InverseRefused(error))?;plan.native_row+=1;plan.progress.done=plan.progress.done.saturating_add(1);}else{admit!();plan.native_phase=6;}},
            6=>{demand.copy_bytes+=store_log.envelope.schema.len()+size_of::<String>();demand.capacity_bytes=store_log.envelope.schema.len();admit!();plan.native_schema=store_log.envelope.schema.clone();plan.native_phase=8;},
            _=>{admit!();plan.finished=true;},
        }
        Ok((plan.finished,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes}))
    }

    /// 🧵️ Immutable O(1) snapshot capability for worker and composition boundaries.
    pub fn snapshot_read(&self, store_owners: &StoreOwners<P, Mutation>, store_wire: &StoreWire<P, Mutation>, store_cursor: &StoreCursor<P, Mutation>) -> Result<SnapshotRead<P>, VcsError>
    where
        P: Sync,
    {
        if !store_owners.snapshot_read_leases.publish_authority(store_cursor.generation(store_wire),store_cursor.content_revision(store_wire)){return Err(VcsError::ValidationFailed("snapshot read commit authority is busy or exhausted".into()))}
        self.snapshot_read_original(store_owners,store_wire,store_cursor).map_err(|error|VcsError::ValidationFailed(error.to_string()))
    }

    /// 🎟️ The original registry issues the same typed lease and preserves its allocation-free native refusal.
    pub fn snapshot_read_original(&self,store_owners:&StoreOwners<P,Mutation>,store_wire:&StoreWire<P,Mutation>,store_cursor:&StoreCursor<P,Mutation>)->Result<SnapshotRead<P>,semio_framework_value::ValueError> where P:Sync {
        let generation=store_cursor.generation(store_wire);let revision=store_cursor.content_revision(store_wire);
        if !store_owners.snapshot_read_leases.authority_matches(generation,revision){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original snapshot read differs from its funded Store publication authority"))}
        let owner=self.snapshot_owner(store_wire);
        let lease=store_owners.snapshot_read_leases.try_issue_original(owner.clone()).map_err(|(error,_original)|error.into_value_error())?;
        Ok(SnapshotRead::new(owner,lease))
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
            ArtifactStoreCursorDisposerPhase::Backbone if !store_wire.original_outbox.terminal_is_empty()=>ArtifactStoreBackboneRetirement::original_outbox_birth_demands(),
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
    pub async fn fold_current(&self, store_log: &StoreLog<P, Mutation>) -> Result<P, VcsError>  where P: Clone, Mutation: ColdMutation<P> {
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
