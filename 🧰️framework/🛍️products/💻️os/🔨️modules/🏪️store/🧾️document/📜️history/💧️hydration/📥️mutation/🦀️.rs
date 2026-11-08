//! 📥️ Exact binary mutations remain in their original receiver until typed publication and paid metadata closure.
use super::*;
use semio_framework_job::{OriginalTurnFailure,close_original_retained_step,close_original_effect_step};
use semio_framework_value_native_owned::native_decoding::NativeConstructionStepResult;

impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>+OpBinaryReceiving{
 pub(super) fn step_native_mutation(&mut self,cx:&mut StepContext<'_>)->bool{
  if self.original_failure.is_some()||self.original_context_failure.is_some(){return false}
  match self.bind_original_step(cx){Ok(true)=>{},Ok(false)=>return false,Err(error)=>{*self.original_context_failure=Some(error);return false}}
  if self.mutation_output.value.is_some(){
   if let Some(cursor)=self.mutation_receiving.as_mut(){
    if !cursor.terminal_is_empty(){let demand=match cursor.close_demands(cx.retained_grant().maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>{*self.original_failure=Some(error.into());return false}};
     if let Err(error)=cx.with_original_retirement(|cx|close_original_effect_step(cx,demand,|effect|cursor.close_step(effect))){*self.original_context_failure=Some(error)}return false}
    let release=std::mem::size_of::<M::Receiving>();let copy=std::mem::size_of_val(&self.mutation_receiving);let demand=RetirementDemand{copy_bytes:copy,release_bytes:release,depth:1,..Default::default()};
    if let Err(error)=cx.with_original_retirement(|cx|close_original_retained_step(cx,demand,|_|{self.mutation_receiving.take();semio_framework_value::RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})),work:1}})){*self.original_context_failure=Some(error)}return false
   }return true
  }
  let history=self.history.as_ref().expect("the original History remains while a mutation is receiving");let edit=&history.edits[self.edit_index];let payloads=if self.phase==Phase::DecodeForward{&edit.ops}else{&edit.inverse};
  let Some(payload)=payloads.get(self.operation_index)else{return false};let source=match native_mutation_bytes(payload){Ok(source)=>source,Err(error)=>{*self.original_failure=Some(error.into());return false}};
  if self.mutation_constructing||self.mutation_receiving.is_none(){
   let capacity=std::mem::size_of::<M::Receiving>();let overhead=capacity+2*std::mem::size_of::<Option<Box<M::Receiving>>>();let grant=cx.retained_grant();
   if !self.mutation_constructing&&(cx.fuel_remaining()<2||grant.maximum_items==0||grant.maximum_depth<2||grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<overhead){return false}
   let demand=semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:overhead,capacity_bytes:capacity,release_bytes:0,depth:2};
   let constructing=&mut self.mutation_constructing;
   let status=semio_framework_job::admit_original_construction(cx,&mut*self.mutation_receiving,&mut self.mutation_admission,demand,1,|scope,actual|{
    *constructing=true;
    let granted=scope.retained_grant();
    let Some(child)=mutation_receiving_child(granted,capacity,overhead)else{return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"mutation receiving birth lacks its quoted grant")),work:0}};
    let turn=M::admit_receiving(source,(0,source.len()),child);let work=u128::from(turn.work).saturating_add(1);
    let result=match turn.result{
     Ok(Some((owner,progress)))=>{*actual=Some(capacity);let extra=RetainedCloneProgress{copied_bytes:overhead,retained_capacity_bytes:capacity,..Default::default()};progress.checked_add(extra).map(|progress|(Box::new(owner),progress))},
     Ok(None)=>Err(ValueError::literal(ValueRefusalKind::WorkLimit,"mutation receiving birth was denied by its original owner")),
     Err(error)=>Err(error),
    };
    semio_framework_value::RetainedWorkOutcome{result,work}
   });
   match status{
    Ok(semio_framework_job::OriginalConstructionStatus::Ready)=>match self.mutation_admission.reset_step(cx){Ok(true)=>self.mutation_constructing=false,Ok(false)=>{},Err(error)=>*self.original_context_failure=Some(error)},
    Ok(semio_framework_job::OriginalConstructionStatus::Pending)=>{},
    Ok(semio_framework_job::OriginalConstructionStatus::Failed)=>match self.mutation_admission.take_failure_into_step(&mut self.mutation_refusal,cx){Ok(true)=>{self.mutation_constructing=false;*self.original_context_failure=Some(ValueError::literal(ValueRefusalKind::InvalidValue,"original mutation receiving birth was refused").into())},Ok(false)=>{},Err(error)=>*self.original_context_failure=Some(error)},
    Err(error)=>*self.original_context_failure=Some(error),
   }
   return false
  }
  let cursor=&mut*self.mutation_receiving;let output=&mut*self.mutation_output.value;let failure=&mut*self.original_failure;let grant=cx.retained_grant();let body=grant.maximum_copy_bytes;
  let mut taken=None;
  let mut scope=None;let result=cx.with_native_receiving(&mut scope,|native,_|{
   let cursor=cursor.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"mutation receiving cursor is absent"))?;
   if !cursor.is_ready(){let turn=cursor.step(source,native,body,grant);if let Err(error)=turn.result{*failure=Some(OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::Receiving(error)))}}
   else{taken=Some(cursor.take_into(output,grant))}
   Ok(())
  });
  if matches!(result,Ok(true)){if let Some(turn)=taken{if let Err(error)=semio_framework_job::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:u128::from(turn.work)}){*self.original_failure=Some(error)}}}
  match result{
   Err(failure)=>*self.original_context_failure=Some(failure),
   Ok(true)=>match scope{
    Some(scope) if scope.source_refusal.is_some()||scope.call.is_err()=>*self.original_context_failure=Some(ValueError::literal(ValueRefusalKind::InvalidValue,"original mutation receiving scope refused").with_retained_progress(scope.retained_progress()).into()),
    Some(_)=>{},
    None=>*self.original_context_failure=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"original mutation receiving scope vanished").into()),
   },
   Ok(false)=>{},
  }
  false
 }
}

fn mutation_receiving_child(grant:RetainedCloneGrant,capacity:usize,overhead:usize)->Option<RetainedCloneGrant>{
 if grant.maximum_items==0||grant.maximum_depth<2{return None}
 Some(RetainedCloneGrant{maximum_depth:grant.maximum_depth.checked_sub(1)?,maximum_copy_bytes:grant.maximum_copy_bytes.checked_sub(overhead)?,maximum_capacity_bytes:grant.maximum_capacity_bytes.checked_sub(capacity)?,..grant})
}

fn native_mutation_bytes(payload:&semio_os_spr::OpPayload)->Result<&[u8],ValueError>{payload.binary.as_deref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"native persisted hydration requires the declared binary mutation payload"))}

#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn actual_child_grants_match_the_independent_funding_corpus(){
  let law:serde_json::Value=serde_json::from_str(include_str!("grant.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){let grant=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_depth:row["depth"].as_u64().unwrap()as usize,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:73};let child=mutation_receiving_child(grant,law["capacity"].as_u64().unwrap()as usize,law["overhead"].as_u64().unwrap()as usize);assert_eq!(child.is_some(),row["accepted"].as_bool().unwrap());if let Some(child)=child{assert_eq!(child.maximum_copy_bytes+law["overhead"].as_u64().unwrap()as usize,grant.maximum_copy_bytes);assert_eq!(child.maximum_capacity_bytes+law["capacity"].as_u64().unwrap()as usize,grant.maximum_capacity_bytes);assert_eq!(child.maximum_release_bytes,grant.maximum_release_bytes);}}
 }
 #[test]
 fn exact_binary_source_matches_the_language_neutral_policy(){
  let law:serde_json::Value=serde_json::from_str(include_str!("input.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){let payload=semio_os_spr::OpPayload{binary:row["binary"].as_array().map(|values|values.iter().map(|value|value.as_u64().unwrap()as u8).collect()),text:row["text"].as_str().map(str::to_owned)};let actual=native_mutation_bytes(&payload);assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap());if let Ok(bytes)=actual{assert_eq!(bytes,row["expected"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>());}}
 }
}
