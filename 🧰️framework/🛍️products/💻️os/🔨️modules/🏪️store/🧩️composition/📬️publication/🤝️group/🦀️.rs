//! 🤝️ Retains one private member root until its shared visibility owner decides.

use super::*;

pub(super) struct MemberGroupPreparation {
    visibility: Arc<crate::os_vcs::ArtifactGroupVisibility>,
    pub(super) history: Option<ArtifactStoreHistoryCommitReservation>,
    pub(super) displaced: Option<ArtifactStoreDisplacedOwnerReservation>,
    pub(super) applied: crate::os_vcs::HistoryPageStack<String>,
    pub(super) cursor: crate::os_vcs::HistoryPageStack<String>,
    pub(super) revision: Option<CursorRevisionAccumulator>,
    pub(super) checkpoint: Option<String>,
    phase: u8,
    ordinal: usize,
    operation: usize,
    operation_indexed: bool,
    demand: usize,
    completed: u32,
}

impl MemberGroupPreparation {
    pub(super) fn new(visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, history: Option<ArtifactStoreHistoryCommitReservation>, displaced: Option<ArtifactStoreDisplacedOwnerReservation>) -> Self {
        Self { visibility: Arc::clone(visibility), history, displaced, applied: crate::os_vcs::HistoryPageStack::empty(), cursor: crate::os_vcs::HistoryPageStack::empty(), revision: None, checkpoint: None, phase: 0, ordinal: 0, operation: 0, operation_indexed: false, demand: 1, completed: 0 }
    }

    pub(super) fn next_byte_demand(&self) -> usize { self.demand }
    pub(super) fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.history.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.displaced.is_some() {
            let capacity_bytes = if self.applied.capacity() != 0 || self.cursor.capacity() != 0 { std::mem::size_of::<ArtifactStoreStringVectorRetirement>() } else if self.revision.is_some() { std::mem::size_of::<ArtifactStoreRevisionAccumulatorRetirement>() } else if self.checkpoint.is_some() { std::mem::size_of::<ArtifactStoreStringRetirement>() } else { 0 };
            return Ok(RetirementDemand { capacity_bytes, depth: 1, ..Default::default() });
        }
        let release_bytes = if Arc::strong_count(&self.visibility) == 1 { std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<crate::os_vcs::ArtifactGroupVisibility>()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "group visibility frame layout overflow"))?.0.pad_to_align().size() } else { 0 };
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }

    pub(super) fn visibility(&self) -> Arc<crate::os_vcs::ArtifactGroupVisibility> { Arc::clone(&self.visibility) }

    pub(super) fn completed_items(&self) -> u32 { self.completed }
}

fn retain_group_owner(retirements: &mut ArtifactStoreDisplacedRetirements, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) {
    if let Err(owner) = retirements.push_owner_reserved(reservation, owner) { retirements.push_reserved(owner); }
}

impl<P, Mu> ArtifactStore<P, Mu>
where
    P: Clone + ToValue + FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub(super) fn stage_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        let group = publication.group_preparation.as_mut().ok_or("member group lacks its retained preparation")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.pending() { return Err("member group visibility authority does not match its pending decision".into()); }
        if self.generation != publication.expected_generation || self.content_revision != publication.expected_revision { return Err("member group became stale before its common decision".into()); }
        if group.phase == 12 { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let stage = publication.stage.as_ref().ok_or("member group lost its staged batch")?;
        if publication.phase != ArtifactStoreOneItemPublicationPhase::Publishing || stage.post.is_none() || self.durable_group_root.is_some() { return Err("member group requires an exact private publication root".into()); }
        if publication.close_started || publication.cancel_requested { return Err("member group candidate was cancelled".into()); }
        let mut ownership=RetainedCloneProgress{copied_items:1,..Default::default()};
        if group.displaced.is_none(){group.demand=0;group.displaced=Some(self.displaced_retirements.reserve_owner_slots(12).map_err(|error|error.to_string())?);}
        else if group.history.is_none(){
            group.demand=self.envelope.vcs.edits.next_reservation_allocation_bytes();
            if grant.maximum_capacity_bytes<group.demand{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
            ownership.retained_capacity_bytes=group.demand;group.history=Some(self.reserve_edit_history_slot().map_err(|error|error.to_string())?);
        }else{
            if group.revision.is_none(){group.revision=Some(CursorRevisionAccumulator{identity_digest:self.revision_accumulator.identity_digest,applied:crate::os_vcs::HistoryPageStack::empty(),redo:crate::os_vcs::HistoryPageStack::empty(),applied_tail_chains:None,mutation_positions:protocol::HistoryFoldIndex::new(),indexed_edits:protocol::HistoryFoldIndex::new(),unit_flags:protocol::HistoryFoldIndex::new()});}
            if group.ordinal<self.applied_edit_ids.len(){
                let source=&self.applied_edit_ids[group.ordinal];
                ownership.copied_bytes=if group.phase<2{source.len()}else{std::mem::size_of::<CursorRevisionRecord>()};
                group.demand=match group.phase{0=>group.applied.next_push_allocation_bytes()+source.len(),1=>group.cursor.next_push_allocation_bytes()+source.len(),_=>group.revision.as_ref().unwrap().applied.next_push_allocation_bytes()};
                if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                ownership.retained_capacity_bytes=group.demand;
                match group.phase{0=>{group.applied.try_push(source.clone()).map_err(|_|"member applied catalog exhausted")?;group.phase=1;},1=>{group.cursor.try_push(source.clone()).map_err(|_|"member cursor catalog exhausted")?;group.phase=2;},_=>{group.revision.as_mut().unwrap().applied.try_push(self.revision_accumulator.applied[group.ordinal].clone()).map_err(|_|"member revision catalog exhausted")?;group.ordinal+=1;group.phase=0;}}
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
                        let revision=group.revision.as_mut().unwrap();group.demand=revision.applied.next_push_allocation_bytes();ownership.copied_bytes=std::mem::size_of::<CursorRevisionRecord>();ownership.retained_capacity_bytes=group.demand;
                        if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        let previous=revision.applied.last().map_or(revision.identity_digest,|record|record.prefix_digest);
                        revision.applied.try_push(CursorRevisionRecord{ledger_key:None,id_digest:CursorRevisionAccumulator::hash_record(b"edit-id",&[stage.edit.as_ref().unwrap().id.as_bytes()]),edit_digest:stage.digest,prefix_digest:CursorRevisionAccumulator::hash_record(b"applied",&[&previous,&stage.digest])}).map_err(|_|"member appended revision exhausted")?;group.phase=6;
                    },
                    6=>{
                        group.demand=self.current_checkpoint_id.as_ref().map_or(0,String::len);ownership.copied_bytes=group.demand;ownership.retained_capacity_bytes=group.demand;
                        if grant.maximum_capacity_bytes<group.demand||grant.maximum_copy_bytes<group.demand{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        group.checkpoint=(*self.current_checkpoint_id).clone();group.phase=7;
                    },
                    7=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.mutation_positions.iter().next_back(){Some((key,_))=>self.revision_accumulator.mutation_positions.range((Excluded(key),Unbounded)).next(),None=>self.revision_accumulator.mutation_positions.first_key_value()};
                        if let Some((key,value))=next{clone_insert!(revision.mutation_positions,key,*value);}else{group.phase=8;}
                    },
                    8=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.indexed_edits.iter().next_back(){Some((key,_))=>self.revision_accumulator.indexed_edits.range((Excluded(key),Unbounded)).next(),None=>self.revision_accumulator.indexed_edits.first_key_value()};
                        if let Some((key,value))=next{reserve_index!(revision.indexed_edits,key);revision.indexed_edits.insert_reserved(*key,*value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;}else{group.phase=9;}
                    },
                    9=>{
                        let revision=group.revision.as_mut().unwrap();let next=match revision.unit_flags.iter().next_back(){Some((key,_))=>self.revision_accumulator.unit_flags.range((Excluded(key),Unbounded)).next(),None=>self.revision_accumulator.unit_flags.first_key_value()};
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
                        let capacity=std::mem::size_of::<durable_group::StagedRootRetirement<P,Mu>>();let release=std::mem::size_of::<Edit<Mu>>();group.demand=capacity;
                        if grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<release{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
                        let ownership=durable_group::stage_prebuilt_batch_root(self,publication,grant)?;let group=publication.group_preparation.as_mut().unwrap();group.phase=12;group.completed=group.completed.saturating_add(1);
                        return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),ownership));
                    }
                }
            }
        }
        publication.group_preparation.as_mut().unwrap().completed=publication.group_preparation.as_ref().unwrap().completed.saturating_add(1);
        Ok(ArtifactStoreOneItemPreparationStep::Progress(publication.progress(),ownership))
    }

    pub(super) fn adopt_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, visibility: &Arc<crate::os_vcs::ArtifactGroupVisibility>, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        if publication.published { return Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),Default::default())); }
        let group = publication.group_preparation.as_ref().ok_or("member group lacks its retained adoption")?;
        if !Arc::ptr_eq(&group.visibility, visibility) || !visibility.committed() || group.phase != 12 { return Err("member adoption requires its exact committed common decision".into()); }
        if !grant.permits_one() { return Ok(ArtifactStoreOneItemPreparationStep::Blocked); }
        let checkpoint=publication.progress();
        durable_group::adopt_staged_store_member(self,visibility).map_err(|error|error.to_string())?;
        *self.durable_group_root=None;publication.group_preparation=None;publication.retained_checkpoint=checkpoint;
        publication.receipt=Some(LaneItemReceipt{generation_before:publication.expected_generation,generation_after:self.generation});publication.expected_generation=self.generation;publication.expected_revision=self.content_revision;publication.published=true;publication.phase=ArtifactStoreOneItemPublicationPhase::AwaitingAck;
        Ok(ArtifactStoreOneItemPreparationStep::Prepared(publication.progress(),RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    pub(super) fn abort_apply_batch_group(&mut self, publication: &mut ArtifactStoreBatchPublication<P, Mu>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let error = |message: &'static str| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message);
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "member group abort requires admitted depth")); }
        let Some(group) = publication.group_preparation.as_mut() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if group.visibility.committed() || group.visibility.pending() { return Err(error("member abort requires its exact aborted common decision")); }
        if self.durable_group_root.is_some() {
            durable_group::abort_staged_store_member(self, &group.visibility).map_err(|_| error("member abort lost its exact staged root"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(history) = group.history.take() {
            let ArtifactStoreHistoryCommitReservation { history, rejected_owner } = history;
            if let Err(history) = self.envelope.vcs.edits.cancel_reservation(history) { group.history = Some(ArtifactStoreHistoryCommitReservation { history, rejected_owner }); return Err(error("member abort lost its exact ledger reservation")); }
            self.displaced_retirements.release_owner_slots(rejected_owner).map_err(|_| error("member abort lost its rejected owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(reservation) = group.displaced.as_mut() {
            if group.applied.capacity() != 0 {
                let original = std::mem::replace(&mut group.applied, crate::os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.applied = original; return Err(error); } }
            }
            if group.cursor.capacity() != 0 {
                let original = std::mem::replace(&mut group.cursor, crate::os_vcs::HistoryPageStack::empty());
                match admit_artifact_retirement(original, grant, ArtifactStoreStringVectorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.cursor = original; return Err(error); } }
            }
            if let Some(original) = group.revision.take() { match admit_artifact_retirement(original, grant, ArtifactStoreRevisionAccumulatorRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.revision = Some(original); return Err(error); } } }
            if let Some(original) = group.checkpoint.take() { match admit_artifact_retirement(original, grant, ArtifactStoreStringRetirement::new) { Ok((owner, receipt)) => { retain_group_owner(&mut self.displaced_retirements, reservation, owner); return Ok(RetainedCloneStep::Progress(receipt)); }, Err((error, original)) => { group.checkpoint = Some(original); return Err(error); } } }
            self.displaced_retirements.release_owner_slots(group.displaced.take().unwrap()).map_err(|_| error("member abort lost its displaced owner reservation"))?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let released_bytes = group.retirement_demands()?.release_bytes;
        if grant.maximum_release_bytes < released_bytes || Arc::weak_count(&group.visibility) != 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        publication.begin_close();
        publication.group_preparation = None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }))
    }
}
