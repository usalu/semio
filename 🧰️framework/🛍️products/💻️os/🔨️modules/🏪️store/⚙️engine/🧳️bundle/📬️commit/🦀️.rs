//! 📬️ Original publication donates its prepared history, causal suffix and transport intent together.
use super::*;
use semio_framework_os_store_one_item_batch::lifted_member_group::BatchCommitMetadata;

pub(super) fn advance_original_commit<P,M>(store:&mut StoreParts<'_,P,M>,publication:&mut ArtifactStoreBatchPublication<P,M>,body:usize,grant:ArtifactStoreOneItemGrant)->Result<ArtifactStoreOneItemAdvance,VcsError>
where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{
 if let Some(step)=original_message::prepare_original_outgoing(store,publication,body,grant)?{return Ok(step)}
 let stage=publication.stage.as_mut().expect("original commit retains its batch stage");
 let fail=|message:&'static str|VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message));
 if stage.commit_metadata.is_none(){let copy=size_of::<BatchCommitMetadata>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(ArtifactStoreOneItemAdvance::Blocked)}stage.commit_metadata=Some(BatchCommitMetadata::new(None,None));return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}
 let metadata=stage.commit_metadata.as_mut().unwrap();
 match advance_batch_metadata(store,metadata,stage.edit.as_ref().ok_or_else(||fail("original commit lost its prepared edit"))?,stage.digest,&mut stage.unit_flags,&mut stage.diagnostics,grant).map_err(VcsError::ValidationFailed)?{
  BatchCommitMetadataStep::Blocked=>return Ok(ArtifactStoreOneItemAdvance::Blocked),
  BatchCommitMetadataStep::Progress(p)=>return Ok(ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),p)),
  BatchCommitMetadataStep::Ready=>{},
 }
 let queue=store.wire.original_outbox.append_demands().map_err(VcsError::InverseRefused)?;
 let seed=protocol::MutationDag::original_seed_commit_demands();
 let diagnostic=if stage.diagnostics.row_count()==0{0}else{ArtifactStoreDiagnosticStage::transfer_copy_demand()};
 let copy=size_of::<Edit<M>>()+size_of::<BatchCommitMetadata>()+size_of::<semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement>()+8*size_of::<usize>()+queue.copy_bytes+seed.copy_bytes+diagnostic;
 if grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<size_of::<Edit<M>>()||grant.maximum_depth<queue.depth.max(seed.depth).max(1){return Ok(ArtifactStoreOneItemAdvance::Blocked)}
 let lane_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copy,..grant.retained_grant()};
 if let Some(owner)=publication.lane_insert.as_ref(){if !owner.commit_ready(&store.log.envelope.lanes,lane_grant).map_err(VcsError::InverseRefused)?{return Ok(ArtifactStoreOneItemAdvance::Blocked)}}
 if !store.wire.original_outbox.can_append(grant.retained_grant()).map_err(VcsError::InverseRefused)?||stage.message.is_none()||stage.seed_owner.is_none()||metadata.history.is_none()||metadata.revision.is_none()||stage.post.is_none()||store.log.envelope.cursor.is_none(){return Err(fail("original commit lost one of its admitted publication owners"))}
 let before=store.cursor.generation;
 let edit=stage.edit.take().unwrap();
 let sequence=edit.sequence_number;
 let reservation=metadata.history.take().unwrap();
 let key=store.log.envelope.vcs.edits.insert_reserved(reservation.history,*edit).unwrap_or_else(|_|panic!("original commit retains its exact reserved history slot"));
 store.owners.displaced_retirements.release_owner_slots(reservation.rejected_owner).expect("original commit retains its rejected owner reservation");
 let revision=metadata.revision.as_mut().unwrap();let mut tail=revision.applied.pop().expect("original commit prepared its appended revision");tail.ledger_key=Some(key);revision.applied.try_push(tail).unwrap_or_else(|_|panic!("original revision retains its just vacated tail slot"));
 std::mem::swap(&mut *store.log.applied_edit_ids,&mut metadata.applied);
 let cursor=store.log.envelope.cursor.as_mut().unwrap();std::mem::swap(&mut cursor.applied_edit_ids,&mut metadata.cursor);
 metadata.cursor_redo=Some(std::mem::replace(&mut cursor.redo_edit_ids,semio_framework_os_vcs::HistoryPageStack::empty()));
 metadata.log_redo=Some(std::mem::replace(&mut *store.log.redo_edit_ids,semio_framework_os_vcs::HistoryPageStack::empty()));
 std::mem::swap(&mut *store.cursor.revision_accumulator,metadata.revision.as_mut().unwrap());
 stage.displaced_report=Some(semio_framework_os_store_ledger::ArtifactStorePendingReportRetirement::new(std::mem::take(&mut *store.replay.pending_report)));
 let incoming=stage.post.take().unwrap();let old=std::mem::replace(&mut *store.live.current,incoming);
 let next=(std::mem::take(&mut stage.tail_edit_id),Arc::clone(&old));
 if let Some((identity,snapshot))=store.live.tail_undo_cache.replace(next){stage.tail_edit_id=identity;stage.post=Some(snapshot)}
 drop(old);
 let diagnostics=publish_prepared_diagnostics(store,&mut stage.diagnostics,grant.retained_grant())?;
 let queued=store.wire.original_outbox.append_reserved(&mut stage.message,grant.retained_grant()).map_err(VcsError::InverseRefused)?.expect("original commit preflight funds its reserved queue slot");
 let seeded=store.log.dag.commit_original_seeds(&mut stage.seed_owner,grant.retained_grant()).map_err(VcsError::InverseRefused)?.expect("original commit preflight funds its exact causal suffix");
 let lane=if let Some(owner)=publication.lane_insert.as_mut(){let(slot,p)=owner.commit_controlled(&mut store.log.envelope.lanes,lane_grant,|lane|lane).expect("original commit retains its lane reservation");assert!(slot.is_some());p}else{Default::default()};
 store.log.edit_sequence=sequence;store.log.clock=stage.next_clock;store.live.fold_frontier=None;
 store.cursor.generation+=1;store.cursor.content_revision=store.cursor.revision_accumulator.revision(store.log.current_checkpoint_id.as_deref());
 assert!(store.owners.snapshot_read_leases.publish_authority(store.cursor.generation,store.cursor.content_revision));
 store.cursor.last_projection_cause=Some(ArtifactProjectionCause::Apply);
 publication.announce_from=0;publication.announce_items=publication.admitted_items;publication.outbound=false;
 let receipt=LaneItemReceipt{generation_before:before,generation_after:store.cursor.generation};publication.expected_generation=store.cursor.generation;publication.expected_revision=store.cursor.content_revision;publication.receipt=Some(receipt);publication.published=true;publication.phase=ArtifactStoreOneItemPublicationPhase::AwaitingAck;
 Ok(ArtifactStoreOneItemAdvance::Published(receipt,RetainedCloneProgress{copied_items:1,copied_bytes:copy+lane.copied_bytes,retained_capacity_bytes:diagnostics.retained_capacity_bytes+queued.retained_capacity_bytes+seeded.retained_capacity_bytes+lane.retained_capacity_bytes,released_bytes:size_of::<Edit<M>>()+diagnostics.released_bytes+lane.released_bytes}))
}
