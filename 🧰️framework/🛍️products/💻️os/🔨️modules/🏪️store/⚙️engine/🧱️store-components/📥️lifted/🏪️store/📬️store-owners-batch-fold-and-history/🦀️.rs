//! 📬️ Store owners batch fold and history: the one item publication batch fold, the event sourced history and the remaining owner operations over the owned stage. See [`super`].

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
impl<P, Mutation> StoreOwners<P, Mutation>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    //#region 📬️OneItemPublication
    /// 🔏 The gesture's declared row budget, not the budget minus its forwards. An item's
    /// inverse row count is the VARIABLE part of a declaration, and reserving only the
    /// leftover made the staging buffer — not the gesture-wide `PreflightingCommit` gate —
    /// the first refuser of an under-declared multi-item gesture, with the wrong message and
    /// before the fold had folded anything the gate could weigh. The gate still refuses the
    /// overrun; this buffer only has to be able to hold the rows the per-item fold admits.
    ///
    /// 🔏 A stamped publication's first folded item keeps the identity its durable decision was
    /// hashed over. The `#position` form is the store's way of naming the several mutations one
    /// local gesture folds into one edit; rewriting the stamped item to it produced a committed
    /// decision whose `mutation_id` no longer matched the approval target the WAL verifies.
    pub fn fold_batch_item(&mut self, store_log: &mut StoreLog<P, Mutation>, publication: &mut ArtifactStoreBatchPublication<P, Mutation>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemAdvance, VcsError> {
        
        let amend_identity=self.batch_amend_target_ref(publication.transaction.as_ref());
        let diagnostic_source=amend_identity.and_then(|id|self.envelope.edit_messages.get_by_id(id)).map(|entry|entry.messages.as_slice());
        let diagnostic_offset:usize = amend_identity.and_then(|id|self.envelope.vcs.edits.iter().find(|edit|edit.id==id)).map_or(0, |edit| edit.forwards.len());
let authority=publication.authority.as_ref().ok_or_else(||VcsError::ValidationFailed("batched fold lost its original authority".into()))?;
        let control=size_of::<usize>();
        if grant.maximum_depth==0||grant.maximum_copy_bytes<control{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
        if publication.stage.is_none(){
            let capacity=size_of::<ArtifactStoreBatchStage<P,Mutation>>();
            if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let mut stage=Self::empty_batch_stage(authority);stage.mutation_retirement=(*self.mutation_retirement_factory).clone();stage.snapshot_retirement=(*self.snapshot_retirement_factory).clone();
            if stage.mutation_retirement.is_none()||stage.snapshot_retirement.is_none(){return Err(VcsError::ValidationFailed("batched fold lacks its exact original retirement issuers".into()));}
            publication.stage=Some(Box::new(stage));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));
        }
        let stage=publication.stage.as_mut().unwrap();
        if stage.edit.is_none(){let capacity=size_of::<Edit<Mutation>>();if grant.maximum_capacity_bytes<capacity{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.edit=Some(Box::new(Self::empty_batch_edit(authority)));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:capacity,..Default::default()}));}
        macro_rules! reserve_original {($owner:expr,$count:expr,$ty:ty)=>{{let owner=$owner;if owner.capacity()<$count{let capacity=$count.checked_mul(std::mem::size_of::<$ty>()).ok_or_else(||VcsError::ValidationFailed("batch original capacity overflow".into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<owner.capacity()*std::mem::size_of::<$ty>(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}let released=owner.capacity()*std::mem::size_of::<$ty>();owner.try_reserve_exact($count-owner.len()).map_err(|_|VcsError::ValidationFailed("batch original reservation failed".into()))?;let actual=owner.capacity()*std::mem::size_of::<$ty>();return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:control,retained_capacity_bytes:actual,released_bytes:released,..Default::default()}));}}};}
                if stage.diagnostics.batch.is_none() { let copy=size_of::<semio_framework_os_spr::command::MutationMessagePublicationBatch>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.diagnostics.batch=Some(Default::default());return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})); }
        let receipt=stage.diagnostics.batch.as_mut().unwrap().reserve(publication.admitted_items + usize::from(diagnostic_source.is_some()),grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
        if receipt!=RetainedCloneProgress::default(){return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),receipt));}
        let prefix=stage.diagnostics.prepare_prefix(diagnostic_source,grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;
        if !matches!(prefix,RetainedCloneStep::Complete(_))||prefix.progress()!=RetainedCloneProgress::default(){return Ok(if prefix.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),prefix.progress())});}
reserve_original!(&mut stage.unit_flags,publication.admitted_items,(usize,bool));
        reserve_original!(&mut stage.edit.as_mut().unwrap().forwards,publication.admitted_items,Mutation);
        reserve_original!(&mut stage.edit.as_mut().unwrap().mutation_meta,publication.admitted_items,MutationMeta);
        let inverse=&mut stage.edit.as_mut().unwrap().inverse;
        let backing=if publication.transaction_open{inverse.next_capacity_allocation_bytes(publication.footprint.work_items)}else{inverse.next_exact_capacity_allocation_bytes(publication.footprint.work_items)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;
        if let Some(capacity)=backing{let depth=inverse.next_reserve_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;if grant.maximum_capacity_bytes<capacity||grant.maximum_depth<depth{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let step=if publication.transaction_open{inverse.reserve_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}else{inverse.reserve_exact_capacity_one(publication.footprint.work_items,grant.maximum_capacity_bytes)}.map_err(|error|VcsError::ValidationFailed(error.reason.into()))?;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:usize::from(step.progressed),retained_capacity_bytes:step.allocated_bytes,..Default::default()}));}
        if stage.folding.is_none(){
            let candidate=publication.preparation.as_ref().and_then(|owner|owner.prepared()).ok_or_else(||VcsError::ValidationFailed("batched fold lost its original prepared candidate".into()))?;authority.validate_prepared(candidate).map_err(VcsError::from)?;
            if candidate.edit().forwards.len()!=1||candidate.edit().mutation_meta.len()!=1||candidate.edit().id.is_empty()||candidate.edit().id.len()>ARTIFACT_STORE_ONE_ITEM_ID_BYTES||!candidate.applied_edit_id.eq_str(&candidate.edit().id)||!candidate.tail_edit_id.eq_str(&candidate.edit().id)||candidate.next_clock()!=authority.next_clock()||!candidate.local_actor().as_ref().is_some_and(|actor|actor.eq_str(&authority.actor())){return Err(VcsError::ValidationFailed("batched fold candidate disagrees with its original fixed authority".into()));}
            if candidate.edit().inverse.len().saturating_add(1)>publication.footprint.work_items{return Err(VcsError::TooLarge{rows:candidate.edit().inverse.len()+1,capacity:publication.footprint.work_items});}
            let demand=ArtifactStoreOneItemPrepared::<P,Mutation>::retirement_birth_demand();if demand.admit(grant.retained_grant()).is_err(){return Ok(ArtifactStoreOneItemAdvance::Blocked);}
            let candidate=publication.preparation.as_mut().unwrap().take_prepared().unwrap();
            let(cursor,ownership)=candidate.admit_fold_retirement(Arc::clone(stage.mutation_retirement.as_ref().unwrap()),Arc::clone(stage.snapshot_retirement.as_ref().unwrap()),grant.retained_grant()).map_err(|(error,_,_,_)|VcsError::ValidationFailed(error.into_message()))?;stage.folding=Some(cursor);stage.fold_phase=0;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership));
        }
        let cursor=stage.folding.as_mut().unwrap();let original=cursor.edit.as_mut().unwrap();let target=stage.edit.as_mut().unwrap();
        macro_rules! copy_string {($target:expr,$source:expr)=>{{let source=$source;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}$target=source.to_owned();stage.fold_phase+=1;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:copy,..Default::default()}));}};}
        let mut ownership=RetainedCloneProgress{copied_items:1,copied_bytes:control,..Default::default()};
        match stage.fold_phase{
            0=>{if stage.folded==0{copy_string!(target.id,original.id.as_str());}stage.fold_phase=1;},
            1=>{if stage.folded==0{copy_string!(stage.applied_edit_id,target.id.as_str());}stage.fold_phase=2;},
            2=>{if stage.folded==0{copy_string!(stage.tail_edit_id,target.id.as_str());}stage.fold_phase=3;},
            3=>{if stage.folded==0{let copy=size_of::<Edit<Mutation>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.actor=original.actor.take();target.line=original.line.take();target.started_at=std::mem::take(&mut original.started_at);stage.local_actor=cursor.strings[0].take();stage.next_clock=cursor.fold_clock;ownership.copied_bytes=copy;}stage.fold_phase=4;},
            4=>{if stage.folded==0{if let Some(verb)=publication.verb.as_deref(){let copy=verb.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.verb=Some(verb.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}}stage.fold_phase=5;},
            5=>{if !original.inverse.is_empty(){let depth=original.inverse.next_pop_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?.max(target.inverse.next_push_depth_demand().map_err(|error|VcsError::ValidationFailed(error.reason.into()))?);let copy=size_of::<Mutation>();if grant.maximum_depth<depth||grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=original.inverse.pop().unwrap();target.inverse.push_reserved(item).unwrap_or_else(|_|panic!("original inverse admission lost its reserved slot"));ownership.copied_bytes=copy;}else{stage.fold_phase=6;}},
            6=>{let copy=size_of::<Mutation>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.forwards.push(original.forwards.pop().expect("original one-item forward"));ownership.copied_bytes=copy;stage.fold_phase=7;},
            7=>{let copy=size_of::<MutationMeta>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let meta=&mut original.mutation_meta[0];target.mutation_meta.push(MutationMeta{mutation_id:None,dependencies:std::mem::take(&mut meta.dependencies),base_version:meta.base_version,author_id:meta.author_id.take(),timestamp:meta.timestamp,undo_policy:meta.undo_policy,payload_hash:meta.payload_hash.take(),semantic_kind:None,label:None,group_id:meta.group_id.take(),origin:std::mem::take(&mut meta.origin),transaction:None});ownership.copied_bytes=copy;stage.fold_phase=8;},
            8=>{let position=target.forwards.len()-1;let primary=publication.admitted_items==1&&!publication.transaction_open;let copy=target.id.len()+if primary{0}else{1+position.checked_ilog10().unwrap_or(0)as usize+1};if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut id=String::with_capacity(copy);id.push_str(&target.id);if !primary{use std::fmt::Write;write!(id,"#{position}").unwrap();}target.mutation_meta.last_mut().unwrap().mutation_id=Some(MutationId(id));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=9;},
            9=>{let kind=target.forwards.last().unwrap().descriptor().semantic_kind;let copy=store_log.envelope.schema.len()+1+kind.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let mut value=String::with_capacity(copy);value.push_str(&store_log.envelope.schema);value.push('#');value.push_str(kind);target.mutation_meta.last_mut().unwrap().semantic_kind=Some(SchemaId(value));ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=10;},
            10=>{let source=target.forwards.last().unwrap().descriptor().display_name;let copy=source.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().label=Some(source.to_owned());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;stage.fold_phase=11;},
            11=>{if let Some(transaction)=publication.transaction.as_ref(){let copy=transaction.id.len()+transaction.tool.len();if grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}target.mutation_meta.last_mut().unwrap().transaction=Some(transaction.clone());ownership.copied_bytes=copy;ownership.retained_capacity_bytes=copy;}stage.fold_phase=12;},
            12=>{if target.forwards.last().unwrap().may_emit_foreign_steps(){let Some(presence)=cursor.foreign_step_presence else{return Ok(ArtifactStoreOneItemAdvance::Blocked);};let copy=size_of::<(usize,bool)>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.unit_flags.push((target.forwards.len()-1,presence));ownership.copied_bytes=copy;}stage.fold_phase=13;},
            13=>{let copy=size_of::<Arc<P>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let incoming=cursor.post.take().expect("original prepared post root");*cursor.post=stage.post.replace(incoming);ownership.copied_bytes=copy;stage.fold_phase=14;},
            
            14=>{if let Some(messages)=cursor.outcome_messages.as_mut(){if stage.diagnostic_row<messages.original_messages().len(){let offset=diagnostic_offset.checked_add(target.forwards.len()-1).ok_or_else(||VcsError::ValidationFailed("diagnostic operation offset overflow".into()))?;let receipt=messages.stamp_row(stage.diagnostic_row,offset,grant.retained_grant()).map_err(|error|VcsError::ValidationFailed(error.into_message()))?;if receipt.copied_items==0{return Ok(ArtifactStoreOneItemAdvance::Blocked);}stage.diagnostic_row+=1;return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),receipt));}}stage.fold_phase=15;},
            15=>{if let Some(messages)=cursor.outcome_messages.as_ref(){let count=messages.original_messages().len();let total=stage.diagnostics.rows.checked_add(count).ok_or_else(||VcsError::ValidationFailed("diagnostic row count overflow".into()))?;let copy=size_of::<semio_framework_os_spr::command::MutationMessagePublication>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let owner=cursor.outcome_messages.take().unwrap();match stage.diagnostics.batch.as_mut().unwrap().append(owner,grant.retained_grant()){Ok(receipt)=>{stage.diagnostics.rows=total;ownership=receipt;},Err(owner)=>{*cursor.outcome_messages=Some(owner);return Ok(ArtifactStoreOneItemAdvance::Blocked);}}}stage.diagnostic_row=0;stage.fold_phase=16;},
_=>{let copy=size_of::<ArtifactStoreOneItemCheckpoint>()+size_of::<Box<ArtifactStorePreparedRetirement<P,Mutation>>>();if grant.maximum_copy_bytes<copy{return Ok(ArtifactStoreOneItemAdvance::Blocked);}let item=publication.preparation.as_ref().unwrap().checkpoint();stage.folded+=1;stage.completed_items=stage.completed_items.saturating_add(item.completed_items);stage.completed_bytes=stage.completed_bytes.saturating_add(item.completed_bytes);stage.digest=cursor.fold_digest;stage.retiring=stage.folding.take().map(|owner|owner as Box<dyn ErasedSnapshotRetirement>);publication.item_closing=true;publication.preparation.as_mut().unwrap().begin_close();ownership.copied_bytes=copy;},
        }
        Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),ownership))
    }

    /// 🧾️ The empty staged shell every fold accumulates into — no allocation happens here; the
    /// first fold reserves the gesture's exact admitted capacity.
    pub fn empty_batch_stage(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreBatchStage<P, Mutation> {
        ArtifactStoreBatchStage {
            
            diagnostics: ArtifactStoreDiagnosticStage::default(),
            diagnostic_row: 0,
unit_flags: Vec::new(),
            folding: None,
            fold_phase: 0,
            digest_owner: None,
            digest_complete: false,
            edit: None,
            pending_edit: None,
            post: None,
            next_clock: authority.next_clock(),
            local_actor: None,
            applied_edit_id: String::new(),
            tail_edit_id: String::new(),
            digest: [0; 32],
            folded: 0,
            completed_items: 0,
            completed_bytes: 0,
            mutation_retirement: None,
            snapshot_retirement: None,
            retiring: None,
            factory_close: Default::default(),
        }
    }

    pub fn empty_batch_edit(authority: &ArtifactStoreOneItemLiveAuthority) -> Edit<Mutation> {
        Edit {
                line: None,
                id: String::new(),
                actor: None,
                forwards: Vec::new(),
                inverse: Vec::new().into(),
                mutation_meta: Vec::new(),
                verb: None,
                sequence_number: authority.next_sequence_number(),
                started_at: String::new(),
                finished_at: None,
            }
    }

    //#endregion 📬️OneItemPublication
    //#region 🔖️EventSourcedHistory
    /// 🛫️ Refuses a reprojection before it adopts anything: `fold`'s new history facts fit their ledgers and their pins
    /// parse, `replay`'s new message entries fit the message ledger, and the displaced-owner queue holds every owner the
    /// adoption (with `recorded` prefix projections) and the revision bump that follows it displace.
    pub fn preflight_reprojection(&self, store_log: &StoreLog<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold, replay: Option<&EditReplayResult<P, Mutation>>, recorded: usize) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let vcs = &store_log.envelope.vcs;
        let changes = fold.changes.iter().filter(|change| !vcs.changes.iter().any(|known| known.id == change.id)).count();
        let checkpoints = fold
            .checkpoints
            .iter()
            .filter(|checkpoint| !vcs.checkpoints.iter().any(|known| known.id == checkpoint.id || (known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id))))
            .count();
        let alternatives = fold.alternatives.iter().filter(|alternative| !vcs.alternatives.iter().any(|known| known.id == alternative.id)).count();
        if changes > vcs.changes.vacancies() || checkpoints > vcs.checkpoints.vacancies() || alternatives > vcs.alternatives.vacancies() {
            return Err(VcsError::ValidationFailed("history ledgers cannot hold the folded facts".into()));
        }
        for pin in fold.checkpoints.iter().flat_map(|checkpoint| checkpoint.pins.iter()) {
            semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?;
        }
        let entries = replay.map_or(&[][..], |result| result.replayed.as_slice());
        let admitted = entries.iter().filter(|entry| !entry.messages.is_empty() && store_log.envelope.edit_messages.get_by_id(&entry.edit_id).is_none()).count();
        if admitted > ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY.saturating_sub(store_log.envelope.edit_messages.len()) {
            return Err(VcsError::ValidationFailed("artifact edit-message ledger cannot hold the replayed messages".into()));
        }
        self.displaced_retirements.reserve(usize::from(changes + checkpoints + alternatives > 0) + entries.len() + 2 * recorded + REPROJECTION_OWNER_SLOTS)
    }

    /// 🧹️ Hands a finished replay's untaken outputs and original residual to the displaced queue; a replay without installed
    /// issuers (its aliases would wait behind the queue's own shared-root retirements) or a saturated queue retires them
    /// unscheduled from their own quoted demands instead.
    pub fn retire_replay_result(&mut self, mut result: EditReplayResult<P, Mutation>) -> Result<(), VcsError> {
        result.begin_retirement();
        if result.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !result.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(result));
                Ok(())
            }
            _ => result.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands a raw replay's original owners to the displaced queue; a saturated queue retires them unscheduled instead.
    pub fn retire_raw_replay(&mut self, mut replay: EditReplay<P, Mutation>) -> Result<(), VcsError> {
        replay.begin_retirement();
        if replay.terminal_is_empty() {
            return Ok(());
        }
        match self.displaced_retirements.reserve(1) {
            Ok(()) if !replay.cold_authority() => {
                self.displaced_retirements.push_reserved(Box::new(replay));
                Ok(())
            }
            _ => replay.retire_unscheduled().map_err(|error| VcsError::ValidationFailed(error.into_message())),
        }
    }

    /// 🧹️ Hands the running replay of a deferred reprojection, if any, to [`Self::retire_raw_replay`].
    pub fn retire_pending_replay(&mut self, replay: Option<([u8; 32], EditReplay<P, Mutation>)>) -> Result<(), VcsError> {
        replay.map_or(Ok(()), |(_, replay)| self.retire_raw_replay(replay))
    }

    /// 🏛️ Materializes the fold's change/checkpoint/alternative facts into the history ledgers: a re-identified
    /// (repinned) checkpoint is renamed in place, every alternative's chain follows the fold, facts the fold no longer
    /// holds (a retracted transition's) leave, and new facts are inserted.
    pub fn adopt_history_facts(&mut self, store_log: &mut StoreLog<P, Mutation>, fold: &semio_framework_os_spr::HistoryFold) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let shrank = |fault: semio_framework_os_vcs::ArtifactHistoryReservationFault| VcsError::ValidationFailed(format!("a history ledger cannot follow its fold: {fault:?}"));
        let stale_change = |known: &Change| !fold.changes.iter().any(|change| change.id == known.id);
        if store_log.envelope.vcs.changes.iter().any(stale_change) {
            store_log.envelope.vcs.changes.extract_if(stale_change).map_err(shrank)?;
        }
        for change in &fold.changes {
            if store_log.envelope.vcs.changes.iter().any(|known| known.id == change.id) {
                continue;
            }
            let reservation = self.reserve_change_history_slot(store_log)?;
            self.insert_reserved_change_history(store_log, reservation, Change { id: change.id.clone(), edit_ids: change.edit_ids.clone(), description: change.description.clone(), saved_at: change.saved_at.clone() })?;
        }
        let mut fresh = Vec::new();
        for checkpoint in &fold.checkpoints {
            let mut pins = Vec::with_capacity(checkpoint.pins.len());
            for pin in &checkpoint.pins {
                pins.push(semio_framework_os_vcs::CompositionPin { child_ref: semio_framework_artifact_reference::ArtifactRef::parse_uri(&pin.child_uri).map_err(VcsError::Deserialize)?, checkpoint_id: pin.checkpoint_id.clone() });
            }
            let known_position = store_log
                .envelope
                .vcs
                .checkpoints
                .iter()
                .position(|known| known.id == checkpoint.id)
                .or_else(|| store_log.envelope.vcs.checkpoints.iter().position(|known| known.parent_id == checkpoint.parent_id && known.change_ids == checkpoint.change_ids && !fold.checkpoints.iter().any(|folded| folded.id == known.id)));
            let Some(position) = known_position else {
                fresh.push((checkpoint, pins));
                continue;
            };
            let known = &mut store_log.envelope.vcs.checkpoints[position];
            if known.id != checkpoint.id {
                known.id = checkpoint.id.clone();
            }
            if known.composition_pins != pins {
                known.composition_pins = pins;
            }
        }
        let stale_checkpoint = |known: &Checkpoint| !fold.checkpoints.iter().any(|checkpoint| checkpoint.id == known.id);
        if store_log.envelope.vcs.checkpoints.iter().any(stale_checkpoint) {
            store_log.envelope.vcs.checkpoints.extract_if(stale_checkpoint).map_err(shrank)?;
        }
        for (checkpoint, pins) in fresh {
            let authors = checkpoint.authors.iter().map(|author| Author { id: author.id.clone(), name: author.name.clone(), avatar: author.avatar.clone() }).collect();
            let reservation = self.reserve_checkpoint_history_slot(store_log)?;
            self.insert_reserved_checkpoint_history(store_log, 
                reservation,
                Checkpoint {
                    id: checkpoint.id.clone(),
                    change_ids: checkpoint.change_ids.clone(),
                    parent_id: checkpoint.parent_id.clone(),
                    authors,
                    message: checkpoint.message.clone(),
                    timestamp: checkpoint.timestamp.clone(),
                    composition_pins: pins,
                },
            )?;
        }
        let stale_alternative = |known: &Alternative| !fold.alternatives.iter().any(|alternative| alternative.id == known.id);
        if store_log.envelope.vcs.alternatives.iter().any(stale_alternative) {
            store_log.envelope.vcs.alternatives.extract_if(stale_alternative).map_err(shrank)?;
        }
        for alternative in &fold.alternatives {
            if let Some(known) = store_log.envelope.vcs.alternatives.iter_mut().find(|known| known.id == alternative.id) {
                if known.checkpoint_ids != alternative.checkpoint_ids {
                    known.checkpoint_ids = alternative.checkpoint_ids.clone();
                }
                continue;
            }
            let reservation = self.reserve_alternative_history_slot(store_log)?;
            self.insert_reserved_alternative_history(store_log, reservation, Alternative { id: alternative.id.clone(), name: alternative.name.clone(), checkpoint_ids: alternative.checkpoint_ids.clone() })?;
        }
        order_ledger_as(&mut store_log.envelope.vcs.changes, |change: &Change| change.id.as_str(), fold.changes.iter().map(|change| change.id.as_str()));
        order_ledger_as(&mut store_log.envelope.vcs.checkpoints, |checkpoint: &Checkpoint| checkpoint.id.as_str(), fold.checkpoints.iter().map(|checkpoint| checkpoint.id.as_str()));
        order_ledger_as(&mut store_log.envelope.vcs.alternatives, |alternative: &Alternative| alternative.id.as_str(), fold.alternatives.iter().map(|alternative| alternative.id.as_str()));
        Ok(())
    }

    //#endregion 🔖️EventSourcedHistory
    pub fn replace_edit_messages(&mut self, store_log: &mut StoreLog<P, Mutation>, edit_id: &str, messages: Vec<semio_framework_os_spr::MutationMessage>) -> Result<(), VcsError> {
        if edit_id.len() > ARTIFACT_EDIT_MESSAGE_ID_BYTES {
            return Err(VcsError::ValidationFailed("artifact edit-message id exceeds its fixed byte authority".into()));
        }
        let established = store_log.envelope.edit_messages.get_by_id(edit_id).is_some();
        self.displaced_retirements.reserve(1)?;
        if messages.is_empty() {
            if let Some(entry) = store_log.envelope.edit_messages.remove_id(edit_id) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(entry.edit_id, entry.messages)));
            }
            return Ok(());
        }
        let mut candidate = semio_framework_os_spr::EditMessages { edit_id: edit_id.to_string(), messages };
        bound_edit_messages(edit_id, &mut candidate.messages);
        if established {
            let previous = std::mem::replace(store_log.envelope.edit_messages.get_mut_by_id(edit_id).expect("established edit-message identity remains live"), candidate);
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(previous.edit_id, previous.messages)));
        } else if let Err(rejected) = store_log.envelope.edit_messages.admit(candidate) {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMessageLedgerRetirement::new(rejected.edit_id, rejected.messages)));
            return Err(VcsError::ValidationFailed("artifact edit-message fixed ledger rejected its exact replacement owner".into()));
        }
        Ok(())
    }

    /// 🔂️ Replays `operations` over `pre_snapshot`, returning forwards, reversed-inverse,
    /// per-operation metadata, the resulting snapshot, and every `crate::os_spr::MutationMessage`
    /// the replay raised. Shared by `Apply` and `AppendTransaction`. This IS the artifact engine —
    /// `crate::os_engine::ArtifactEngine` never existed as a live trait (see
    /// `.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`), so `Mutation::diff`/
    /// `inverse` are called directly here on purpose, not as a placeholder for a future indirection.
    /// ATOMIC: if `self.merge_policy.rejects(worst)`, returns `Err` before this method's caller has
    /// touched any store field — nothing about the attempted batch is applied.
    ///
    /// 🌀️ `.await` resolves to an owned value — `.reverse()` on the unawaited future's
    /// result was mutating a throwaway temporary, never `back` itself.
    ///
    /// 🌀️ `mint_mutation_id` is async; `Option::unwrap_or_else`'s closure is sync
    /// (R10 shape 1), so it's written as an explicit match instead.
    ///
    /// 🎯️ An authored timestamp is durable as authored; the local clock observes it
    /// so its next generated timestamp remains causally later.
    ///
    /// 🎞️ CW3: direct blake3 (same primitive `crate::os_pack::ContentHash` uses) replaces the
    /// old `framework_hash::hash_bytes` String hash — `crate::os_spr::PayloadHash` is
    /// now `[u8; 32]`, not a hex string. NOT `crate::os_pack::stored_content_hash`, which reads a pack
    /// FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
    /// `OpBinary` encoding, not a JSON serialization — two ops that encode identically
    /// via `encode_op()` but differ in JSON shape (or vice versa) must hash identically.
    ///
    /// 🩹️ Everything the replay builds — its working projection, the command's operations, their inverses — is a
    /// [`ScratchOwner`]: a refusal on ANY exit retires all of it cold, and the caller adopts each part where the store takes it.
    /// 📦️ Materializes the installed original operation source under the caller's retained admission.
    pub fn authored_operation_bytes(&mut self, store_log: &mut StoreLog<P, Mutation>,mutations:&[Mutation],identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError>{Self::authored_operation_rows(mutations.len(),mutations.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)}

    /// ↩️ Streams the original recorded forward and inverse rows without cloning the durable edit.
    pub fn authored_edit_operation_bytes(&mut self, store_log: &mut StoreLog<P, Mutation>,identity:&mut EntityIdentityAuthority<'_>)->Result<Option<(Vec<Vec<u8>>,Vec<Vec<u8>>)>,VcsError>{
        let Some(edit)=store_log.envelope.vcs.edits.last()else{return Ok(None)};
        let forwards=Self::authored_operation_rows(edit.forwards.len(),edit.forwards.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)?;
        let inverse=Self::authored_operation_rows(edit.inverse.len(),edit.inverse.iter(),self.one_item_preparation_factory.as_ref(),store_log.operation_wire,&mut self.displaced_retirements,identity)?;
        Ok(Some((forwards,inverse)))
    }

    /// 🪪️ The one resolution of an authored operation's wire: the installed factory's domain codec when it speaks one, else the store's canonical wire.
    fn resolve_operation_wire<'a>(factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,mutation:&'a Mutation)->Option<ArtifactPreparedOperationSource<'a>>{
        factory.and_then(|factory|factory.operation_wire_source(mutation)).or_else(||wire.and_then(|wire|wire.source(mutation)))
    }

    fn authored_operation_rows<'a>(count:usize,mutations:impl Iterator<Item=&'a Mutation>,factory:Option<&Arc<dyn ArtifactStoreOneItemPreparationFactory<P,Mutation>>>,wire:Option<ArtifactOperationWire<Mutation>>,retirements:&mut ArtifactStoreDisplacedRetirements,identity:&mut EntityIdentityAuthority<'_>)->Result<Vec<Vec<u8>>,VcsError> where Mutation:'a{
        let slots=count.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"operation wire retirement slots overflow"))?;
        retirements.reserve(slots)?;
        let mut rows=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationWireRows>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationWireRows::new()))})?;
        let result=(||{
            identity.encode(|control|rows.initialize(count,control))?;
            for mutation in mutations{
                let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
                let encoded=match Self::resolve_operation_wire(factory,wire,mutation){
                    Some(source)=>identity.encode(|control|cursor.encode_into(source,rows.active(),control)),
                    None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
                };
                retirements.push_reserved(cursor);
                encoded?;rows.append();
            }
            Ok::<_,ValueError>(rows.take())
        })();
        retirements.push_reserved(rows);
        result.map_err(VcsError::from)
    }

    fn authored_operation_identity(&mut self, store_log: &mut StoreLog<P, Mutation>,mutation:&Mutation,stamp:(u64,u64,u64),identity:&mut EntityIdentityAuthority<'_>)->Result<(String,[u8;32]),VcsError>{
        self.displaced_retirements.reserve(1)?;
        let mut cursor=identity.encode(|control|{control.charge(size_of::<operation_identity::OperationIdentityCursor>())?;Ok::<_,ValueError>(Box::new(operation_identity::OperationIdentityCursor::new()))})?;
        let committed=match Self::resolve_operation_wire(self.one_item_preparation_factory.as_ref(),store_log.operation_wire,mutation){
            Some(source)=>identity.encode(|control|cursor.commit(source,stamp,control)),
            None=>Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"authoring requires an installed canonical operation wire or an original domain codec")),
        };
        self.displaced_retirements.push_reserved(cursor);
        committed.map_err(VcsError::from)
    }

    pub async fn replay_mutations(&mut self, store_log: &mut StoreLog<P, Mutation>, pre_snapshot: &P, mutations: Vec<Mutation>, transaction: Option<&protocol::TransactionRef>, identity: &mut EntityIdentityAuthority<'_>) -> Result<(ScratchOwner<Vec<Mutation>>, ScratchOwner<Vec<Mutation>>, Vec<MutationMeta>, ScratchOwner<P>, Vec<semio_framework_os_spr::MutationMessage>, Vec<(usize, bool)>), VcsError>  where Mutation: ColdMutation<P>, P: Clone, {
        let mut snapshot = ScratchOwner::new(pre_snapshot.clone(), retire_replayed_projection::<P, Mutation>);
        let forwards = ScratchOwner::new(mutations, retire_scratch_operations::<P, Mutation>);
        let mut inverse = ScratchOwner::new(Vec::new(), retire_scratch_operations::<P, Mutation>);
        let mut candidate_clock = store_log.clock;
        let mut mutation_meta = Vec::with_capacity(forwards.len());
        let mut messages = Vec::new();
        let mut unit_flags = Vec::new();
        for op_index in 0..forwards.len() {
            if forwards[op_index].may_emit_foreign_steps() { unit_flags.push((op_index,  <Mutation as ColdMutation<P>>::foreign_step_source(&forwards[op_index],&snapshot, 0).map_err(VcsError::from)?.is_some())); }
            let mut back = forwards[op_index].inverse(&*snapshot).map_err(VcsError::InverseRefused)?;
            back.reverse();
            inverse.extend(back);
            let authored_timestamp = forwards[op_index].timestamp();
            let timestamp = match authored_timestamp {
                Some(timestamp) => {
                    candidate_clock.merge(&timestamp);
                    timestamp
                }
                None => {
                    candidate_clock.tick(now_ms());
                    candidate_clock
                }
            };
            let (encoded_identity,payload_hash)=self.authored_operation_identity(store_log, &forwards[op_index],(candidate_clock.actor,candidate_clock.physical_ms,candidate_clock.logical),identity)?;
            let authored_id = forwards[op_index].mutation_id();
            let mutation_id = match authored_id {
                Some(id) => id,
                None => MutationId(encoded_identity),
            };
            let mutation = &forwards[op_index];
            mutation_meta.push(MutationMeta {
                mutation_id: Some(mutation_id),
                dependencies: mutation.dependencies(),
                base_version: mutation.base_version().map_or(0, |version| version.0),
                author_id: Some(mutation.author_id().unwrap_or_else(|| store_log.local_actor_id().clone())),
                timestamp,
                undo_policy: mutation.undo_policy(),
                payload_hash: Some(semio_framework_os_spr::PayloadHash(payload_hash)),
                semantic_kind: Some(operation_semantic_kind::<P, Mutation>(mutation, &store_log.envelope.schema)),
                label: Some(mutation.descriptor().display_name.to_string()),
                group_id: None,
                origin: Default::default(),
                transaction: transaction.cloned(),
            });
            let (applied, op_messages) = replay::apply_operation_into::<P, Mutation>(&mut *snapshot, mutation, op_index as u32);
            messages.extend(op_messages);
             applied?;
        }
        if let Some(level) = semio_framework_os_spr::worst_level(&messages) {
            if store_log.merge_policy.rejects(level) {
                return Err(VcsError::Rejected { policy: store_log.merge_policy, messages });
            }
        }
        store_log.clock = candidate_clock;
        Ok((forwards, inverse, mutation_meta, snapshot, messages, unit_flags))
    }

    /// 🧹️ Refuses an ingest after its candidate causal graph was built: the candidate is
    /// retired through the bounded close protocol, never dropped, and the store stays unchanged.
    pub fn refuse_with_candidate_dag<T>(&mut self, candidate_dag: semio_framework_os_spr::MutationDag, error: VcsError) -> Result<T, VcsError> {
        self.displaced_retirements.reserve(1)?;
        self.displaced_retirements.push_reserved(Box::new(ArtifactStoreMutationDagRetirement::new(candidate_dag)));
        Err(error)
    }

    /// 🎯️ Mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
    /// `envelope.cursor` — the single choke point that keeps the persisted cursor in sync with
    /// live undo/redo state. Called from every `bump()`, so every mutating command re-syncs it.
    /// Only the suffixes `dirt` names are rewritten ([`StackMirrorDirt`]); a missing cursor, or one whose group root is
    /// staged, is replaced whole.
    pub fn sync_cursor(&mut self, store_log: &mut StoreLog<P, Mutation>, dirt: StackMirrorDirt) {
        if store_log.envelope.cursor.as_ref().is_none_or(|cursor| cursor.group_visibility().is_some()) {
            let next = Some(ArtifactCursor::new((*store_log.applied_edit_ids).clone(), (*store_log.redo_edit_ids).clone(), (*store_log.current_checkpoint_id).clone()));
            if let Some(previous) = std::mem::replace(&mut store_log.envelope.cursor, next) {
                self.displaced_retirements.push_reserved(Box::new(ArtifactStoreCursorRetirement::new(previous)));
            }
            return;
        }
        let cursor = store_log.envelope.cursor.as_mut().expect("a mirrored cursor remains present");
        let applied = mirror_id_suffix(&mut cursor.applied_edit_ids, &store_log.applied_edit_ids, dirt.first_applied());
        let redo = mirror_id_suffix(&mut cursor.redo_edit_ids, &store_log.redo_edit_ids, dirt.first_redo());
        let checkpoint = if cursor.checkpoint_id != *store_log.current_checkpoint_id { std::mem::replace(&mut cursor.checkpoint_id, (*store_log.current_checkpoint_id).clone()) } else { None };
        for displaced in [applied, redo].into_iter().flatten() {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringVectorRetirement::new(displaced)));
        }
        if let Some(previous) = checkpoint {
            self.displaced_retirements.push_reserved(Box::new(ArtifactStoreStringRetirement::new(previous)));
        }
    }

    /// 🌡️ Retires displaced owners within this change's own bounded grant while the queue is under pressure
    /// ([`ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY`]): every bump retires at most [`DISPLACED_RELIEF_STEPS`] one-item steps
    /// of [`ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`] — more than any single change displaces — so a burst of changes without
    /// maintenance turns (a long ingest, a test or tool driving the store directly) keeps the queue below its fixed authority,
    /// at O(change) cost, while the runtime's maintenance rotation still drains the rest. A blocked owner (an outstanding
    /// read) stops the relief.
    pub fn relieve_displaced_pressure(&mut self) -> Result<(), VcsError> {
        for _ in 0..DISPLACED_RELIEF_STEPS {
            if !self.displaced_retirements.under_pressure() {
                break;
            }
            let refused = |error: ValueError| VcsError::ValidationFailed(error.into_message());
            let demand = self.displaced_retirements.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(refused)?;
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            match self.displaced_retirements.close_step(grant).map_err(refused)? {
                RetainedCloneStep::Complete(_) => break,
                RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default() => break,
                RetainedCloneStep::Progress(_) => {}
            }
        }
        Ok(())
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
