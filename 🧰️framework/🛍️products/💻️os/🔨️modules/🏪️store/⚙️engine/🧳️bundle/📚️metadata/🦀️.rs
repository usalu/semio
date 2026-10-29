//! 📚️ Standalone and grouped publication prepare the same original Store metadata frontier.
use super::*;
use semio_framework_os_store_one_item_batch::lifted_member_group::BatchCommitMetadata;
/// ⏳️ One exact metadata event completes before its prepared root can publish.
pub enum BatchCommitMetadataStep{Blocked,Progress(RetainedCloneProgress),Ready}
/// 🧮️ Clones only the selected original ledger/index entry into its admitted private destination.
pub fn advance_batch_metadata<P,M>(store:&mut StoreParts<'_,P,M>,metadata:&mut BatchCommitMetadata,edit:&Edit<M>,digest:[u8;32],unit_flags:&mut Vec<(usize,bool)>,diagnostics:&mut semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage,grant:ArtifactStoreOneItemGrant)->Result<BatchCommitMetadataStep,String>
where P: semio_framework_os_spr::StoreState,M: semio_framework_os_spr::StoreMutationFor<P>{
 if !grant.permits_one(){return Ok(BatchCommitMetadataStep::Blocked)}
 if metadata.edit_id_digest.is_none(){let id=&edit.id;let copy=id.len().checked_add(size_of::<Option<[u8;32]>>()).ok_or("commit metadata digest quote overflow")?;if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(BatchCommitMetadataStep::Blocked)}metadata.edit_id_digest=Some(CursorRevisionAccumulator::hash_record(b"edit-id",&[id.as_bytes()]));return Ok(BatchCommitMetadataStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}

        let mut ownership=RetainedCloneProgress{copied_items:1,..Default::default()};
        if metadata.displaced.is_none(){metadata.demand=0;metadata.displaced=Some(store.owners.displaced_retirements.reserve_owner_slots(12).map_err(|error|error.to_string())?);}
        else if metadata.history.is_none(){
            metadata.demand=store.log.envelope.vcs.edits.next_reservation_allocation_bytes();
            if grant.maximum_capacity_bytes<metadata.demand{return Ok(BatchCommitMetadataStep::Blocked);}
            ownership.retained_capacity_bytes=metadata.demand;metadata.history=Some(reserve_edit_history_slot(store).map_err(|error|error.to_string())?);
        }else{
            if metadata.revision.is_none(){let copy=size_of::<Option<CursorRevisionAccumulator>>()+32;if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(BatchCommitMetadataStep::Blocked)}metadata.revision=Some(CursorRevisionAccumulator{identity_digest:store.cursor.revision_accumulator.identity_digest,applied:semio_framework_os_vcs::HistoryPageStack::empty(),redo:semio_framework_os_vcs::HistoryPageStack::empty(),applied_tail_chains:None,mutation_positions:protocol::HistoryFoldIndex::new(),indexed_edits:protocol::HistoryFoldIndex::new(),unit_flags:protocol::HistoryFoldIndex::new()});return Ok(BatchCommitMetadataStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}
            if metadata.ordinal<store.log.applied_edit_ids.len(){
                let source=&store.log.applied_edit_ids[metadata.ordinal];
                ownership.copied_bytes=if metadata.phase<2{source.len()}else{size_of::<CursorRevisionRecord>()};
                metadata.demand=match metadata.phase{0=>metadata.applied.next_push_allocation_bytes()+source.len(),1=>metadata.cursor.next_push_allocation_bytes()+source.len(),_=>metadata.revision.as_ref().unwrap().applied.next_push_allocation_bytes()};
                if grant.maximum_capacity_bytes<metadata.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(BatchCommitMetadataStep::Blocked);}
                ownership.retained_capacity_bytes=metadata.demand;
                match metadata.phase{0=>{metadata.applied.try_push(source.clone()).map_err(|_|"member applied catalog exhausted")?;metadata.phase=1;},1=>{metadata.cursor.try_push(source.clone()).map_err(|_|"member cursor catalog exhausted")?;metadata.phase=2;},_=>{metadata.revision.as_mut().unwrap().applied.try_push(store.cursor.revision_accumulator.applied[metadata.ordinal].clone()).map_err(|_|"member revision catalog exhausted")?;metadata.ordinal+=1;metadata.phase=0;}}
            }else{
                if metadata.phase<3{metadata.phase=3;}
                macro_rules! reserve_index{($index:expr,$key:expr)=>{{
                    let index=&mut $index;let key=$key;
                    if grant.maximum_depth<index.next_insert_depth_demand(key).map_err(|error|error.to_string())?{return Ok(BatchCommitMetadataStep::Blocked);}
                    metadata.demand=index.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes).map_err(|error|error.to_string())?;
                    if metadata.demand!=0{
                        if grant.maximum_capacity_bytes<metadata.demand{return Ok(BatchCommitMetadataStep::Blocked);}
                        let receipt=index.reserve_insert_step(key,grant.retained_grant()).map_err(|(error,_)|error.to_string())?;
                        return Ok(BatchCommitMetadataStep::Progress(receipt));
                    }
                }}}
                macro_rules! clone_insert{($index:expr,$key:expr,$value:expr)=>{{
                    let key=$key;reserve_index!($index,key);
                    metadata.demand=key.0.len();ownership.copied_bytes=key.0.len();ownership.retained_capacity_bytes=key.0.len();
                    if grant.maximum_capacity_bytes<metadata.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(BatchCommitMetadataStep::Blocked);}
                    $index.insert_reserved(key.clone(),$value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;
                }}}
                use std::ops::Bound::{Excluded,Unbounded};
                match metadata.phase{
                    3|4=>{
                        let source=&edit.id;let destination=if metadata.phase==3{&mut metadata.applied}else{&mut metadata.cursor};
                        metadata.demand=destination.next_push_allocation_bytes()+source.len();ownership.copied_bytes=source.len();ownership.retained_capacity_bytes=metadata.demand;
                        if grant.maximum_capacity_bytes<metadata.demand||grant.maximum_copy_bytes<source.len(){return Ok(BatchCommitMetadataStep::Blocked);}
                        destination.try_push(source.clone()).map_err(|_|"member appended catalog exhausted")?;metadata.phase+=1;
                    },
                    5=>{
                        let revision=metadata.revision.as_mut().unwrap();metadata.demand=revision.applied.next_push_allocation_bytes();ownership.copied_bytes=size_of::<CursorRevisionRecord>()+64;ownership.retained_capacity_bytes=metadata.demand;
                        if grant.maximum_capacity_bytes<metadata.demand||grant.maximum_copy_bytes<ownership.copied_bytes{return Ok(BatchCommitMetadataStep::Blocked);}
                        let previous=revision.applied.last().map_or(revision.identity_digest,|record|record.prefix_digest);
                        revision.applied.try_push(CursorRevisionRecord::new(None,metadata.edit_id_digest.unwrap(),digest,CursorRevisionAccumulator::hash_record(b"applied",&[&previous,&digest]))).map_err(|_|"member appended revision exhausted")?;metadata.phase=6;
                    },
                    6=>{
                        metadata.demand=store.log.current_checkpoint_id.as_ref().map_or(0,String::len);ownership.copied_bytes=metadata.demand;ownership.retained_capacity_bytes=metadata.demand;
                        if grant.maximum_capacity_bytes<metadata.demand||grant.maximum_copy_bytes<metadata.demand{return Ok(BatchCommitMetadataStep::Blocked);}
                        metadata.checkpoint=(*store.log.current_checkpoint_id).clone();metadata.phase=7;
                    },
                    7=>{
                        let revision=metadata.revision.as_mut().unwrap();let next=match revision.mutation_positions.iter().next_back(){Some((key,_))=>store.cursor.revision_accumulator.mutation_positions.range((Excluded(key),Unbounded)).next(),None=>store.cursor.revision_accumulator.mutation_positions.first_key_value()};
                        if let Some((key,value))=next{clone_insert!(revision.mutation_positions,key,*value);}else{metadata.phase=8;}
                    },
                    8=>{
                        let revision=metadata.revision.as_mut().unwrap();let next=match revision.indexed_edits.iter().next_back(){Some((key,_))=>store.cursor.revision_accumulator.indexed_edits.range((Excluded(key),Unbounded)).next(),None=>store.cursor.revision_accumulator.indexed_edits.first_key_value()};
                        if let Some((key,value))=next{reserve_index!(revision.indexed_edits,key);revision.indexed_edits.insert_reserved(*key,*value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;}else{metadata.phase=9;}
                    },
                    9=>{
                        let revision=metadata.revision.as_mut().unwrap();let next=match revision.unit_flags.iter().next_back(){Some((key,_))=>store.cursor.revision_accumulator.unit_flags.range((Excluded(key),Unbounded)).next(),None=>store.cursor.revision_accumulator.unit_flags.first_key_value()};
                        if let Some((key,value))=next{reserve_index!(revision.unit_flags,key);revision.unit_flags.insert_reserved(*key,*value,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;}else{metadata.phase=10;}
                    },
                    10 if metadata.operation<edit.forwards.len()=>{
                        let identity_digest=metadata.edit_id_digest.unwrap();let revision=metadata.revision.as_mut().unwrap();
                        if metadata.operation_indexed{reserve_index!(revision.indexed_edits,&identity_digest);revision.indexed_edits.insert_reserved(identity_digest,(metadata.applied.len()-1,edit.forwards.len()),grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;metadata.operation+=1;metadata.operation_indexed=false;}
                        else{let key=edit.mutation_meta.get(metadata.operation).and_then(|meta|meta.mutation_id.as_ref()).ok_or("member staged operation lost its canonical original mutation identity")?;clone_insert!(revision.mutation_positions,key,(metadata.applied.len()-1,metadata.operation,digest));metadata.operation_indexed=true;}
                    },
                    10=>{
                        if let Some((index,flag))=unit_flags.last(){let key=(metadata.edit_id_digest.unwrap(),*index);let revision=metadata.revision.as_mut().unwrap();reserve_index!(revision.unit_flags,&key);revision.unit_flags.insert_reserved(key,*flag,grant.retained_grant()).map_err(|(error,_,_)|error.to_string())?;unit_flags.pop();}else{metadata.phase=11;}
                    },
                    _=>{
                        if diagnostics.row_count()==0{if !diagnostics.terminal_is_empty(){diagnostics.begin_close();let step=diagnostics.close_step(grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if step.progress()==RetainedCloneProgress::default(){BatchCommitMetadataStep::Blocked}else{BatchCommitMetadataStep::Progress(step.progress())});}}
                        else{let identity=edit.id.as_str();if !diagnostics.prepared(){let step=diagnostics.advance(identity,grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if step.progress()==RetainedCloneProgress::default(){BatchCommitMetadataStep::Blocked}else{BatchCommitMetadataStep::Progress(step.progress())});}let bytes=diagnostics.measured_bytes(identity).map_err(|error|error.to_string())?;if !store.log.envelope.edit_messages.admits_measured(identity,bytes){return Err("member diagnostic ledger lacks its measured fixed authority".into());}if store.log.envelope.edit_messages.get_by_id(identity).is_some()&&diagnostics.displaced.is_none(){let receipt=diagnostics.admit_displaced(grant.retained_grant()).map_err(|error|error.to_string())?;return Ok(if receipt==RetainedCloneProgress::default(){BatchCommitMetadataStep::Blocked}else{BatchCommitMetadataStep::Progress(receipt)});}}
                        return Ok(BatchCommitMetadataStep::Ready);
                    }
                }
            }
        }

 metadata.completed=metadata.completed.saturating_add(1);
 Ok(BatchCommitMetadataStep::Progress(ownership))
}
