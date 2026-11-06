//! 🏷️ Original envelope metadata keeps every raw field until its typed destination has real Source capacity.
use super::*;
use semio_framework_value_persistent::work::RetainedWorkLedger;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn metadata_header_valid(source:&semio_os_spr::HistoryOpMeta)->bool{source.op_id.as_deref().is_some_and(|id|!id.trim().is_empty())&&source.author_id.as_deref().is_some_and(|id|!id.trim().is_empty())&&source.hlt.is_some_and(|(_,physical,_)|physical>=0)&&source.undo_policy<=3}
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 pub(super) fn advance_envelope_metadata(&mut self,cx:&mut StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if cx.should_yield()||cx.fuel_remaining()<2{return Ok(())}
  let mut work=RetainedWorkLedger::new(cx.retained_grant());
  let result=(||->Result<(),ValueError>{
   let history=std::sync::Arc::get_mut(self.history.as_mut().ok_or_else(||invalid("metadata lost its retained History"))?).ok_or_else(||invalid("metadata still has a foreign History reader"))?;
   let source=&mut history.edits[self.edit_index];
   let metadata=source.meta.as_mut().ok_or_else(||invalid("persisted edit has no operation metadata"))?;
   if metadata.len()!=source.ops.len(){return Err(invalid("persisted metadata count differs from its operations"))}
   if self.operation_index==metadata.len(){if work.copy_bytes(std::mem::size_of::<Phase>()+std::mem::size_of::<usize>()){self.operation_index=0;self.phase=Phase::FinishEdit;}return Ok(())}
   let source=&mut metadata[self.operation_index];
   if self.original_meta.value.is_none(){
    if !metadata_header_valid(source){return Err(invalid("persisted metadata has no valid identity author or clock"))}
    if !work.copy_bytes(std::mem::size_of::<semio_os_spr::HistoryOpMeta>()+std::mem::size_of::<semio_os_spr::MutationMeta>()+std::mem::size_of::<usize>()){return Ok(())}
    let(actor,physical_ms,logical)=source.hlt.take().unwrap();
    *self.original_meta.value=Some(semio_os_spr::MutationMeta{mutation_id:source.op_id.take().map(semio_os_spr::MutationId),dependencies:Vec::new(),base_version:source.base_version,author_id:source.author_id.take().map(semio_os_spr::ActorId),timestamp:semio_os_spr::HybridLogicalTimestamp{actor,physical_ms:physical_ms as u64,logical},undo_policy:semio_os_store_text_io::text_format::protocol_undo_policy_from_ordinal(source.undo_policy),payload_hash:source.payload_hash.take().map(semio_os_spr::PayloadHash),semantic_kind:None,label:None,group_id:source.group_id.take(),origin:std::mem::take(&mut source.origin),transaction:source.transaction.take()});
    self.metadata_message=0;return Ok(())
   }
   let target=self.original_meta.value.as_mut().unwrap();
   if target.dependencies.len()<source.dependencies.len(){
    if !work.reserve_vec(&mut target.dependencies,source.dependencies.len()).map_err(invalid)?{return Ok(())}
    if !work.vec_slot(&mut target.dependencies).map_err(invalid)?||!work.copy_bytes(std::mem::size_of::<String>()){return Ok(())}
    let index=target.dependencies.len();let id=semio_os_spr::MutationId(std::mem::take(&mut source.dependencies[index]));
    if let Err((message,id))=work.vec_push(&mut target.dependencies,id){source.dependencies[index]=id.0;return Err(invalid(message))}return Ok(())
   }
   let messages=self.pending_messages.as_mut().ok_or_else(||invalid("metadata lost its original message recipient"))?;
   if self.metadata_message<source.messages.len(){
    let message=&mut source.messages[self.metadata_message];let level=semio_framework_diagnostic::Severity::from_u8(message.level).ok_or_else(||invalid("persisted message severity is not declared"))?;
    let count=messages.messages.len().checked_add(source.messages.len()-self.metadata_message).ok_or_else(||invalid("persisted message count overflow"))?;
    if !work.reserve_vec(&mut messages.messages,count).map_err(invalid)?{return Ok(())}
    if !work.vec_slot(&mut messages.messages).map_err(invalid)?||!work.copy_bytes(std::mem::size_of::<semio_os_spr::history::HistoryMessage>()+std::mem::size_of::<usize>()){return Ok(())}
    let value=semio_os_spr::MutationMessage{level,code:semio_framework_diagnostic::FaultCode(std::mem::take(&mut message.code)),message:std::mem::take(&mut message.message),target:std::mem::take(&mut message.target),op_index:message.op_index.take()};
    if let Err((reason,value))=work.vec_push(&mut messages.messages,value){message.code=value.code.0;message.message=value.message;message.target=value.target;message.op_index=value.op_index;return Err(invalid(reason))}self.metadata_message+=1;return Ok(())
   }
   let edit=self.pending_edit.as_mut().ok_or_else(||invalid("metadata lost its typed edit recipient"))?;
   if !work.vec_slot(&mut edit.mutation_meta).map_err(invalid)?||!work.copy_bytes(std::mem::size_of::<Option<semio_os_spr::MutationMeta>>()+std::mem::size_of::<usize>()){return Ok(())}
   let value=self.original_meta.value.take().unwrap();
   if let Err((reason,value))=work.vec_push(&mut edit.mutation_meta,value){*self.original_meta.value=Some(value);return Err(invalid(reason))}
   self.operation_index+=1;Ok(())
  })();
  let progress=work.receipt();let receipt=cx.consume_retained(progress);cx.consume_fuel(work.actual_work()+u64::from(progress.copied_bytes!=0));
  match result{Err(error)=>{let mut cause=semio_framework_job::OriginalTurnFailure::from(error);cause.effects=progress;cause.receipt_refusal=receipt.err();Err(cause)},Ok(())=>receipt.map_err(Into::into)}
 }
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn original_metadata_matches_the_neutral_identity_clock_and_severity_corpus(){
  let law:serde_json::Value=serde_json::from_str(include_str!("input.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){let source=semio_os_spr::HistoryOpMeta{undo_policy:row["undoPolicy"].as_u64().unwrap()as u8,op_id:Some(row["id"].as_str().unwrap().into()),author_id:Some(row["author"].as_str().unwrap().into()),hlt:row["timestamp"].as_array().map(|v|(v[0].as_u64().unwrap(),v[1].as_i64().unwrap(),v[2].as_u64().unwrap())),..Default::default()};let valid=metadata_header_valid(&source)&&row["levels"].as_array().unwrap().iter().all(|v|semio_framework_diagnostic::Severity::from_u8(v.as_u64().unwrap()as u8).is_some());assert_eq!(valid,row["accepted"].as_bool().unwrap());}
 }
}

fn append_declared_text(work:&mut RetainedWorkLedger,target:&mut String,pieces:&[&str],maximum_bytes:usize)->Result<bool,ValueError>{
 let total=pieces.iter().try_fold(0usize,|n,piece|n.checked_add(piece.len()).ok_or_else(||invalid("derived metadata text extent overflow")))?;
 if !work.reserve_string(target,total).map_err(invalid)?{return Ok(false)}if target.len()==total{return Ok(true)}
 let mut offset=target.len();for piece in pieces{if offset>=piece.len(){offset-=piece.len();continue}let available=work.remaining().maximum_copy_bytes.min(maximum_bytes);let mut end=offset.saturating_add(available).min(piece.len());while end>offset&&!piece.is_char_boundary(end){end-=1}if end==offset{return Ok(false)}work.string_push(target,&piece[offset..end]).map_err(invalid)?;return Ok(false)}Err(invalid("derived metadata text frontier changed"))
}
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 pub(super) fn advance_envelope_semantics(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  let edit=self.pending_edit.as_mut().ok_or_else(||invalid("derived metadata lost its typed edit"))?;
  if edit.mutation_meta.len()!=edit.forwards.len(){return Err(invalid("derived metadata count differs from its actual typed operations").into())}
  if self.semantic_index==edit.mutation_meta.len(){return Ok(true)}
  if cx.should_yield()||cx.fuel_remaining()<2{return Ok(false)}
  let descriptor=edit.forwards[self.semantic_index].descriptor();let meta=&mut edit.mutation_meta[self.semantic_index];let schema=&self.envelope.as_ref().ok_or_else(||invalid("derived metadata lost its declared schema"))?.schema;let mut work=RetainedWorkLedger::new(cx.retained_grant());
  let result=(||->Result<(),ValueError>{
   if meta.semantic_kind.is_none(){if work.copy_bytes(std::mem::size_of::<Option<semio_os_spr::SchemaId>>()){meta.semantic_kind=Some(semio_os_spr::SchemaId(String::new()))}return Ok(())}
   if !append_declared_text(&mut work,&mut meta.semantic_kind.as_mut().unwrap().0,&[schema.as_str(),"#",descriptor.semantic_kind],usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX))?{return Ok(())}
   if meta.label.is_none(){if work.copy_bytes(std::mem::size_of::<Option<String>>()){meta.label=Some(String::new())}return Ok(())}
   if !append_declared_text(&mut work,meta.label.as_mut().unwrap(),&[descriptor.display_name],usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX))?{return Ok(())}
   if work.copy_bytes(std::mem::size_of::<usize>()){self.semantic_index+=1}Ok(())
  })();
  let progress=work.receipt();let receipt=cx.consume_retained(progress);cx.consume_fuel(work.actual_work()+u64::from(progress.copied_bytes!=0));
  if let Err(error)=result{let mut cause=semio_framework_job::OriginalTurnFailure::from(error);cause.effects=progress;cause.receipt_refusal=receipt.err();return Err(cause)}receipt?;Ok(false)
 }
}

#[cfg(test)]
mod derived_semantic_laws{
 use super::*;
 #[test]
 fn bounded_native_semantic_text_matches_the_same_portable_declared_fields(){
  let law:serde_json::Value=serde_json::from_str(include_str!("semantics.json")).unwrap();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:8,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64};
  for row in law["cases"].as_array().unwrap(){for(pieces,expected)in[(vec![row["schema"].as_str().unwrap(),"#",row["kind"].as_str().unwrap()],row["semanticKind"].as_str().unwrap()),(vec![row["displayName"].as_str().unwrap()],row["displayName"].as_str().unwrap())]{
   let mut value=String::new();let mut complete=false;for _ in 0..10000{let mut work=RetainedWorkLedger::new(grant);complete=append_declared_text(&mut work,&mut value,&pieces,4).unwrap();assert!(work.receipt().fits(grant));assert!(work.actual_work()<=1);if complete{break}}
   assert!(complete);assert_eq!(value,expected);
  }}
 }
}
