//! 🤝️ Retains one private member root until its shared visibility owner decides.

use std::sync::Arc;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_engine_live::lifted_store::StoreParts;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication;
use semio_framework_os_store_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreDisplacedRetirements, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement};
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, CursorRevisionRecord};
use semio_framework_os_store_one_item::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep, LaneItemReceipt};
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_spr::{Edit, Mutation, OpBinary, OpText};
use semio_framework_value::{ErasedSnapshotRetirement, FromValue, ToValue, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;

fn retain_group_owner(retirements: &mut ArtifactStoreDisplacedRetirements, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) {
    if let Err(owner) = retirements.push_owner_reserved(reservation, owner) { retirements.push_reserved(owner); }
}

/// 🤝️ Publishes one store's member group under a common decision: stage while the decision is pending, adopt after it commits, abort after it refuses.
pub trait ApplyBatchGroup<P, Mu> {
    fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String>;
    fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
}

impl<P, Mu> ApplyBatchGroup<P, Mu> for StoreParts<'_, P, Mu>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    Mu: Clone + ToValue + FromValue + Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary + OpText + Send + 'static,
{
    fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let group = publication.group_preparation.as_mut().ok_or("member group lacks its retained preparation")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.pending() { return Err("member group visibility authority does not match its pending decision".into()); }
        if self.cursor.generation != publication.expected_generation || self.cursor.content_revision != publication.expected_revision { return Err("member group became stale before its common decision".into()); }
        if group.phase == 12 { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let stage = publication.stage.as_ref().ok_or("member group lost its staged batch")?;
        if publication.phase != ArtifactStoreOneItemPublicationPhase::Publishing || stage.post.is_none() || self.wire.durable_group_root.is_some() { return Err("member group requires an exact private publication root".into()); }
        if publication.close_started || publication.cancel_requested { return Err("member group candidate was cancelled".into()); }
        let mut ownership=RetainedCloneProgress{copied_items:1,..Default::default()};
        if group.displaced.is_none(){group.demand=0;group.displaced=Some(self.owners.displaced_retirements.reserve_owner_slots(12).map_err(|error|error.to_string())?);}
        else if group.history.is_none(){
            group.demand=self.log.envelope.vcs.edits.next_reservation_allocation_bytes();
            if grant.maximum_capacity_bytes<group.demand{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            ownership.retained_capacity_bytes=group.demand;group.history=Some(self.owners.reserve_edit_history_slot(self.log).map_err(|error|error.to_string())?);
        }else{
            if group.revision.is_none(){group.revision=Some(CursorRevisionAccumulator{identity_digest:self.cursor.revision_accumulator.identity_digest,applied:semio_framework_os_vcs::HistoryPageStack::empty(),redo:semio_framework_os_vcs::HistoryPageStack::empty(),applied_tail_chains:None,mutation_positions:protocol::HistoryFoldIndex::new(),indexed_edits:protocol::HistoryFoldIndex::new(),unit_flags:protocol::HistoryFoldIndex::new()});}
            if group.ordinal<self.log.applied_edit_ids.len(){
                let source=&self.log.applied_edit_ids[group.ordinal];
                ownership.copied_bytes=if group.phase<2{source.len()}else{size_of::<CursorRevisionRecord>()};
                group.demand=match group.phase{0=>group.applied.next_push_allocation_bytes()+source.len(),1=>group.cursor.next_push_allocation_bytes()+source.len(),_=>group.revision.as_ref().unwrap().applied.next_push_allocation_bytes()};
                if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                ownership.retained_capacity_bytes=group.demand;
                match group.phase{0=>{group.applied.try_push(source.clone()).map_err(|_|"member applied catalog exhausted")?;group.phase=1;},1=>{group.cursor.try_push(source.clone()).map_err(|_|"member cursor catalog exhausted")?;group.phase=2;},_=>{group.revision.as_mut().unwrap().applied.try_push(self.cursor.revision_accumulator.applied[group.ordinal].clone()).map_err(|_|"member revision catalog exhausted")?;group.ordinal+=1;group.phase=0;}}
            }else{
                if group.phase<3{group.phase=3;}
                macro_rules! reserve_index{($index:expr,$key:expr)=>{{
                    let index=&mut $index;let key=$key;
                    if grant.maximum_depth<index.next_insert_depth_demand(key).map_err(|error|error.to_string())?{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                    group.demand=index.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes).map_err(|error|error.to_string())?;
                    if group.demand!=0{
                        if grant.maximum_capacity_bytes<group.demand{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        let receipt=index.reserve_insert_step(key,grant.retained_grant()).map_err(|(error,_)|error.to_string())?;
                        return Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),receipt));
                    }
                }}}
                macro_rules! clone_insert{($index:expr,$key:expr,$value:expr)=>{{
                    let key=$key;reserve_index!($index,key);
                    group.demand=key.0.len();ownership.copied_bytes=key.0.len();ownership.retained_capacity_bytes=key.0.len();
                    if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                    $index.insert_reserved(key.clone(),$value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;
                }}}
                use std::ops::Bound::{Excluded,Unbounded};
                match group.phase{
                    3|4=>{
                        let source=&stage.edit.as_ref().unwrap().id;let destination=if group.phase==3{&mut group.applied}else{&mut group.cursor};
                        group.demand=destination.next_push_allocation_bytes()+source.len();ownership.copied_bytes=source.len();ownership.retained_capacity_bytes=group.demand;
                        if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<source.len(){return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        destination.try_push(source.clone()).map_err(|_|"member appended catalog exhausted")?;group.phase+=1;
                    },
                    5=>{
                        let revision=group.revision.as_mut().unwrap();group.demand=revision.applied.next_push_allocation_bytes();ownership.copied_bytes=size_of::<CursorRevisionRecord>();ownership.retained_capacity_bytes=group.demand;
                        if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        let previous=revision.applied.last().map_or(revision.identity_digest,|record|record.prefix_digest);
                        revision.applied.try_push(CursorRevisionRecord::new(None,CursorRevisionAccumulator::hash_record(b"edit-id",&[stage.edit.as_ref().unwrap().id.as_bytes()]),stage.digest,CursorRevisionAccumulator::hash_record(b"applied",&[&previous,&stage.digest]))).map_err(|_|"member appended revision exhausted")?;group.phase=6;
                    },
                    6=>{
                        group.demand=self.log.current_checkpoint_id.as_ref().map_or(0,String::len);ownership.copied_bytes=group.demand;ownership.retained_capacity_bytes=group.demand;
                        if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<group.demand{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        group.checkpoint=(*self.log.current_checkpoint_id).clone();group.phase=7;
                    },
                    7=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.mutation_positions.iter().next_back(){Some((key,_))=>self.cursor.revision_accumulator.mutation_positions.range((Excluded(key),Unbounded)).next(),None=>self.cursor.revision_accumulator.mutation_positions.first_key_value()};
                        if let Some((key,value))=next{clone_insert!(revision.mutation_positions,key,*value);}else{group.phase=8;}
                    },
                    8=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.indexed_edits.iter().next_back(){Some((key,_))=>self.cursor.revision_accumulator.indexed_edits.range((Excluded(key),Unbounded)).next(),None=>self.cursor.revision_accumulator.indexed_edits.first_key_value()};
                        if let Some((key,value))=next{reserve_index!(revision.indexed_edits,key);revision.indexed_edits.insert_reserved(*key,*value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;}else{group.phase=9;}
                    },
                    9=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.unit_flags.iter().next_back(){Some((key,_))=>self.cursor.revision_accumulator.unit_flags.range((Excluded(key),Unbounded)).next(),None=>self.cursor.revision_accumulator.unit_flags.first_key_value()};
                        if let Some((key,value))=next{reserve_index!(revision.unit_flags,key);revision.unit_flags.insert_reserved(*key,*value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;}else{group.phase=10;}
                    },
                    10 if group.operation<stage.edit.as_ref().unwrap().forwards.len()=>{
                        let edit=stage.edit.as_ref().unwrap();let digest=CursorRevisionAccumulator::hash_record(b"edit-id",&[edit.id.as_bytes()]);let revision=group.revision.as_mut().unwrap();
                        if group.operation_indexed{reserve_index!(revision.indexed_edits,&digest);revision.indexed_edits.insert_reserved(digest,(group.applied.len()-1,edit.forwards.len()),grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;group.operation+=1;group.operation_indexed=false;}
                        else{let key=edit.mutation_meta.get(group.operation).and_then(|meta|meta.mutation_id.as_ref()).ok_or("member staged operation lost its canonical original mutation identity")?;clone_insert!(revision.mutation_positions,key,(group.applied.len()-1,group.operation,digest));group.operation_indexed=true;}
                    },
                    10=>{
                        if let Some((index,flag))=stage.unit_flags.last(){let key=(CursorRevisionAccumulator::hash_record(b"edit-id",&[stage.edit.as_ref().unwrap().id.as_bytes()]),*index);let revision=group.revision.as_mut().unwrap();reserve_index!(revision.unit_flags,&key);revision.unit_flags.insert_reserved(key,*flag,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;publication.stage.as_mut().unwrap().unit_flags.pop();}else{group.phase=11;}
                    },
                    _=>{
                        let stage=publication.stage.as_mut().ok_or("member diagnostics lost staged original")?;
                        if stage.diagnostics.row_count()==0{if !stage.diagnostics.terminal_is_empty(){stage.diagnostics.begin_close();let step=stage.diagnostics.close_step(grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if step.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemPreparationStep::Blocked}else{ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),step.progress())});}}
                        else{let identity=stage.edit.as_ref().ok_or("member diagnostic edit is absent")?.id.as_str();if !stage.diagnostics.prepared(){let step=stage.diagnostics.advance(identity,grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if step.progress()==RetainedCloneProgress::default(){ArtifactStoreOneItemPreparationStep::Blocked}else{ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),step.progress())});}let bytes=stage.diagnostics.measured_bytes(identity).map_err(|error|error.to_string())?;if !self.log.envelope.edit_messages.admits_measured(identity,bytes){return Err("member diagnostic ledger lacks its measured fixed authority".into());}if self.log.envelope.edit_messages.get_by_id(identity).is_some()&&stage.diagnostics.displaced.is_none(){let receipt=stage.diagnostics.admit_displaced(grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if receipt==RetainedCloneProgress::default(){ArtifactStoreOneItemPreparationStep::Blocked}else{ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),receipt)});}}
                        let capacity=size_of::<semio_framework_os_store_durable_group_wire::lifted_durable_group::StagedRootRetirement<P,Mu>>();let release=size_of::<Edit<Mu>>();group.demand=capacity;
                        if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<release{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        let ownership=semio_framework_os_store_engine_stage::lifted_durable_group::stage_prebuilt_batch_root(self,publication,grant)?;let group=publication.group_preparation.as_mut().unwrap();group.phase=12;group.completed=group.completed.saturating_add(1);
                        return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),ownership));
                    }
                }
            }
        }
        publication.group_preparation.as_mut().unwrap().completed=publication.group_preparation.as_ref().unwrap().completed.saturating_add(1);
        Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),ownership))
    }

    fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<semio_framework_os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if publication.published { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        let group = publication.group_preparation.as_ref().ok_or("member group lacks its retained adoption")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.committed() || group.phase != 12 { return Err("member adoption requires its exact committed common decision".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let checkpoint=publication.progress();

        let(done,ownership)=semio_framework_os_store_engine_stage::lifted_durable_group::adopt_staged_store_member(self,visibility,grant.retained_grant()).map_err(|error|error.to_string())?;
        if !done{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        *self.wire.durable_group_root=None;publication.group_preparation=None;publication.retained_checkpoint=checkpoint;
        publication.receipt=Some(LaneItemReceipt{generation_before:publication.expected_generation,generation_after:self.cursor.generation});publication.expected_generation=self.cursor.generation;publication.expected_revision=self.cursor.content_revision;publication.published=true;publication.phase=ArtifactStoreOneItemPublicationPhase::AwaitingAck;
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),RetainedCloneProgress{copied_items:1,..ownership}))
    }

    fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let error = |message: &'static str| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member group abort requires admitted depth")); }
        let Some(group) = publication.group_preparation.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if group.visibility.committed() || group.visibility.pending() { return Err(error("member abort requires its exact aborted common decision")); }
        if self.wire.durable_group_root.is_some() {
            semio_framework_os_store_engine_stage::lifted_durable_group::abort_staged_store_member(self, &group.visibility).map_err(|_| error("member abort lost its exact staged root"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(history) = group.history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = history;
            if let Err(history) = self.log.envelope.vcs.edits.cancel_reservation(history) { group.history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner }); return Err(error("member abort lost its exact ledger reservation")); }
            self.owners.displaced_retirements.release_owner_slots(rejected_owner).map_err(|_| error("member abort lost its rejected owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = group.displaced.as_mut() {
            if group.applied.capacity() != 0 {
                let original = std::mem::replace(&mut group.applied, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.applied = original; return Err(error); } }
            }
            if group.cursor.capacity() != 0 {
                let original = std::mem::replace(&mut group.cursor, semio_framework_os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.cursor = original; return Err(error); } }
            }
            if let Some(original) = group.revision.take() { match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.revision = Some(original); return Err(error); } } }
            if let Some(original) = group.checkpoint.take() { match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.owners.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.checkpoint = Some(original); return Err(error); } } }
            self.owners.displaced_retirements.release_owner_slots(group.displaced.take().unwrap()).map_err(|_| error("member abort lost its displaced owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let released_bytes = group.retirement_demands()?.release_bytes;
        if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        publication.begin_close();
        publication.group_preparation = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }))
    }
}
