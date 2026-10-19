use super::*;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_value::retirement::RetireOwned;
use std::sync::Arc;
fn retained_input(slot:&OriginalActorContextSlot,epoch:u64)->RetainedTurnInput{let owner=slot.context.as_ref().unwrap();let(operation,generation)=owner.identity.unwrap_or_else(||{let(operation,generation,_)=slot.binding.unwrap();(OperationId(operation),Generation(generation))});RetainedTurnInput{operation:operation.0,generation:generation.0,epoch,work:owner.remaining_work().unwrap(),grant:owner.remaining_grant().unwrap()}}
fn close_source<T:RetireOwned+Send+Sync+'static>(slot:&mut OriginalActorContextSlot,raw:&mut Option<T>,epoch:&mut u64,policy:RetainedCloneGrant){
 let mut close=RetainedCloneClose::default();
 for _ in 0..4096{if raw.is_none()&&close.is_empty(){return}*epoch+=1;let input=retained_input(slot,*epoch);let grant=loan(policy,input.grant);let mut external=Default::default();let(call,heap)=observe(||slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,fixture_clock,|cx|{
  let demand=close.next_owner_demands::<T>(raw.is_some(),&None,grant.maximum_copy_bytes)?;
  let step=cx.with_native_retirement(|control|{control.charge(demand.capacity_bytes)?;if close.is_empty(){close.begin_granted(raw,grant)?.ok_or_else(||refusal("original decoder law retained its denied source"))}else{close.step_granted(grant)}})?;let progress=step.progress();cx.consume_retained(progress)?;cx.consume_fuel(progress.copied_items as u64);Ok::<_,OriginalTurnFailure>(())
 }));assert!(call.result.is_ok(),"{:?}",call.result);assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));assert_eq!(external,call.receipt_spent);assert_eq!(call.receipt().unwrap().remaining,slot.context.as_ref().unwrap().remaining_grant().unwrap());}
 panic!("original decoder law did not close retained input/output");
}
#[test]
fn original_actor_decoder_keeps_native_candidate_ledger_receipt_and_root_close_order(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛬️decoder.json")).unwrap();let first=RetainedTurnInput{operation:fixture["operation"].as_u64().unwrap(),generation:fixture["generation"].as_u64().unwrap(),epoch:1,work:declared_work(&fixture),grant:declared_grant(&fixture,"totalGrant")};let policy=declared_grant(&fixture,"callPolicy");let mut slot=OriginalActorContextSlot::empty();let mut epoch=1;
 loop{let input=if slot.context.is_none(){first}else{retained_input(&slot,epoch)};let(call,heap)=observe(||slot.admit_step(input,loan(policy,input.grant),fixture_clock));assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));if call.result.unwrap().ready{break}epoch+=1;assert!(epoch<=5);}
 let mut ledger=None;let mut recipient=None;let mut physical_source=None;let mut source_recipient=None;
 for case in fixture["cases"].as_array().unwrap(){
  let mut raw=Some(fixture["text"].as_str().unwrap().to_owned());let raw_pointer=raw.as_ref().unwrap().as_ptr();let mut output:Option<Arc<String>>=None;epoch+=1;let input=retained_input(&slot,epoch);let mut grant=loan(policy,input.grant);if case["capacity"]=="zero"{grant.maximum_capacity_bytes=0;}let mut external=Default::default();
  let(call,heap)=observe(||slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,fixture_clock,|cx|{
   let source=cx.payload_authority_identity();if let Some(original)=physical_source{assert_eq!(source,original)}else{physical_source=Some(source)}let original_recipient=cx.original_recipient_identity();if let Some(original)=source_recipient{assert_eq!(original_recipient,original)}else{source_recipient=Some(original_recipient)}assert_eq!(cx.original_clock_binding(),Some(first.work.clock_binding));
   let result=cx.with_native_receiving::<_,ValueError>(|control|{if let Some(original)=ledger{assert_eq!(control.ledger_identity(),original)}else{ledger=Some(control.ledger_identity())}if let Some(original)=recipient{assert_eq!(control.retirement_recipient_identity(),Some(original))}else{recipient=control.retirement_recipient_identity()}control.receive_shared(&mut raw,&mut output,semio_framework_value::retirement::shared::sealed::SharedIssuer::owned())});
   let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let accepted=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(progress.copied_items as u64);accepted?;result?;
   if case["semanticFailure"].as_bool().unwrap(){return Err(OriginalTurnFailure::from(refusal("same original semantic failure after native admission")))}Ok(())
  }));assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));assert_eq!(call.receipt_spent,external);assert_eq!(call.receipt().unwrap().remaining,slot.context.as_ref().unwrap().remaining_grant().unwrap());
  assert_eq!(output.is_some(),case["admitted"].as_bool().unwrap());if let Some(output)=output.as_ref(){assert_eq!(output.as_ptr(),raw_pointer);assert!(raw.is_none());}else{assert_eq!(raw.as_ref().unwrap().as_ptr(),raw_pointer);assert_eq!(heap,(0,0));}
  if case["semanticFailure"].as_bool().unwrap()&&case["admitted"].as_bool().unwrap(){assert!(matches!(call.result,Err(OriginalActorCallError::Semantic(_))));}else if case["admitted"].as_bool().unwrap(){assert!(call.result.is_ok());}
  close_source(&mut slot,&mut output,&mut epoch,policy);close_source(&mut slot,&mut raw,&mut epoch,policy);
 }
 let mut denied=false;for _ in 0..4096{if slot.terminal_is_empty(){break}epoch+=1;let input=retained_input(&slot,epoch);let context=slot.context.as_ref().unwrap();
  if context.input().is_none()&&!context.decoder.terminal_is_empty()&&!denied{let demand=context.retirement_demands(policy.maximum_copy_bytes).unwrap();for grant in [RetainedCloneGrant{maximum_items:0,..loan(policy,input.grant)},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..loan(policy,input.grant)},RetainedCloneGrant{maximum_depth:0,..loan(policy,input.grant)}]{let source=slot.context.as_ref().unwrap().source.as_ref().unwrap()as*const _;let(call,heap)=observe(||slot.close_step(input,grant));assert_eq!(call.receipt_spent,Default::default());assert_eq!(heap,(0,0));assert!(call.result.is_ok());assert_eq!(slot.context.as_ref().unwrap().source.as_ref().unwrap()as*const _,source);}denied=true;}
  let decoder_pending=!slot.context.as_ref().unwrap().decoder.terminal_is_empty();let(call,heap)=observe(||slot.close_step(input,loan(policy,input.grant)));assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));assert!(call.result.is_ok(),"{:?}",call.result);if decoder_pending{assert!(slot.context.as_ref().unwrap().source.is_some());}
 }
 assert!(denied);assert!(slot.terminal_is_empty());assert_eq!(observe(||drop(slot)).1,(0,0));
}
