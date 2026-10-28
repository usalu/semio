//! 📤️ The original staged edit prepares causal bytes before its atomic Store publication.
use super::*;

pub(super) fn prepare_original_message<P,M>(store:&mut StoreParts<'_,P,M>,publication:&mut ArtifactStoreBatchPublication<P,M>,body:usize,grant:ArtifactStoreOneItemGrant)->Result<Option<ArtifactStoreOneItemAdvance>,VcsError>
where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{
 let Some(wire)=store.log.operation_wire else{return Ok(None)};
 let Some(admit)=wire.original_message_admission else{return Ok(None)};
 let stage=publication.stage.as_mut().expect("original message owns its staged batch");
 if !stage.digest_complete||stage.message_complete||!publication.outbound{return Ok(None)}
 let refusal=|error:ValueError|VcsError::InverseRefused(error);
 let progress=if let Some(owner)=stage.message_owner.as_mut(){
  if owner.ready(){let Some((edit,bytes,receipt))=owner.take_edit_message(grant.retained_grant()).map_err(refusal)?else{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))};stage.edit=Some(edit);stage.message=Some(bytes);owner.begin_close();receipt}
  else if owner.terminal_is_empty(){let release=size_of_val(owner.as_ref());let copy=size_of::<bool>();if grant.maximum_release_bytes<release||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}stage.message_owner=None;stage.message_complete=true;RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()}}
  else if stage.message.is_some(){let demand=owner.next_demand(body).map_err(refusal)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant.retained_grant(),demand){return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}owner.close_step(grant.retained_grant()).map_err(refusal)?.progress()}
  else{
   let demand=owner.next_message_demand(body).map_err(refusal)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant.retained_grant(),demand){return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}
   let observed=stage.observed_source.map(|key|store.log.envelope.vcs.edits.get_key(key).and_then(|edit|edit.mutation_meta.last()).and_then(|meta|meta.mutation_id.as_ref()).map(|id|id.0.as_str()).ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"observed original operation changed before message preparation")))).transpose()?;
   let step=owner.advance_message(&store.log.envelope.id,&store.log.envelope.schema,observed,grant.retained_grant()).map_err(refusal)?;
   semio_framework_value::retained_clone::admit_retained_clone_progress(grant.retained_grant(),step.progress(),"original precommit causal message").map_err(refusal)?
  }
 }else if !stage.observed_complete{
  let copy=size_of::<Option<usize>>()+size_of::<Option<ArtifactHistoryKey>>()+size_of::<bool>();if grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}
  let next=match stage.observed_source{Some(key)=>store.log.envelope.vcs.edits.previous_key(key),None=>store.log.envelope.vcs.edits.last_key()};
  if let Some(key)=next{let edit=store.log.envelope.vcs.edits.get_key(key).expect("original observed census retains its ledger key");let actor=publication.authority.as_ref().expect("original observed census retains its author").actor();stage.observed_source=Some(key);if edit.actor.as_deref().is_some_and(|author|author!=actor){if edit.mutation_meta.last().and_then(|meta|meta.mutation_id.as_ref()).is_none(){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"observed original edit lacks its canonical last mutation identity")))}stage.observed_complete=true;}}
  else{stage.observed_source=None;stage.observed_complete=true;}
  RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}
 }else{let Some((owner,receipt))=admit(&mut stage.edit,wire,grant.retained_grant()).map_err(refusal)?else{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))};stage.message_owner=Some(owner);receipt};
 Ok(Some(if progress==Default::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),progress)}))
}


pub(super) fn prepare_original_outgoing<P,M>(store:&mut StoreParts<'_,P,M>,publication:&mut ArtifactStoreBatchPublication<P,M>,body:usize,grant:ArtifactStoreOneItemGrant)->Result<Option<ArtifactStoreOneItemAdvance>,VcsError>
where P:ToValue+Send+Sync+'static,M:ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static{
 if let Some(step)=prepare_original_message(store,publication,body,grant)?{return Ok(Some(step))}
 let stage=publication.stage.as_mut().expect("original outgoing owns its staged batch");if !stage.message_complete{return Ok(None)}
 let refuse=VcsError::InverseRefused;
 let receipt=if stage.seed_owner.is_none(){let Some((owner,p))=store.log.dag.admit_original_seeds(grant.retained_grant()).map_err(refuse)?else{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))};stage.seed_owner=Some(owner);p}
 else if stage.seed_pending.is_some(){let Some(p)=store.log.dag.stage_original_seed(stage.seed_owner.as_ref().unwrap(),&mut stage.seed_pending,grant.retained_grant()).map_err(refuse)?else{return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))};if stage.seed_pending.is_none(){stage.seed_index+=1;}p}
 else if stage.seed_index<stage.edit.as_ref().expect("original outgoing retains its edit").mutation_meta.len(){
  let id=stage.edit.as_ref().unwrap().mutation_meta[stage.seed_index].mutation_id.as_ref().ok_or_else(||refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original outgoing operation lacks its canonical identity")))?;
  let copy=id.0.len().checked_add(size_of::<Option<MutationId>>()).ok_or_else(||refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original outgoing identity quote overflow")))?;
  let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:id.0.len(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant.retained_grant(),d){return Ok(Some(ArtifactStoreOneItemAdvance::Blocked))}
  stage.seed_pending=Some(MutationId(id.0.clone()));RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:d.capacity_bytes,..Default::default()}
 }else{let d=store.wire.original_outbox.append_demands().map_err(refuse)?;if d.capacity_bytes==0{return Ok(None)}store.wire.original_outbox.reserve_step(grant.retained_grant()).map_err(refuse)?};
 Ok(Some(if receipt==Default::default(){ArtifactStoreOneItemAdvance::Blocked}else{ArtifactStoreOneItemAdvance::PreparationProgress(publication.progress(),receipt)}))
}
