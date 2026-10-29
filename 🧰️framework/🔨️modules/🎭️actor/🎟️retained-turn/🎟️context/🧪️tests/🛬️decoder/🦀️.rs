use super::*;
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::shared_issuance::SealedShared;
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
  let mut raw=Some(fixture["text"].as_str().unwrap().to_owned());let raw_pointer=raw.as_ref().unwrap().as_ptr();let mut output:Option<SealedShared<String>>=None;epoch+=1;let input=retained_input(&slot,epoch);let mut grant=loan(policy,input.grant);if case["capacity"]=="zero"{grant.maximum_capacity_bytes=0;}let mut external=Default::default();
  let(call,heap)=observe(||slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,fixture_clock,|cx|{
   let source=cx.payload_authority_identity();if let Some(original)=physical_source{assert_eq!(source,original)}else{physical_source=Some(source)}let original_recipient=cx.original_recipient_identity();if let Some(original)=source_recipient{assert_eq!(original_recipient,original)}else{source_recipient=Some(original_recipient)}assert_eq!(cx.original_clock_binding(),Some(first.work.clock_binding));
   let result=cx.with_native_receiving::<_,ValueError>(|control|{if let Some(original)=ledger{assert_eq!(control.ledger_identity(),original)}else{ledger=Some(control.ledger_identity())}if let Some(original)=recipient{assert_eq!(control.retirement_recipient_identity(),Some(original))}else{recipient=control.retirement_recipient_identity()}control.receive_shared(&mut raw,&mut output,semio_framework_value::shared_issuance::SharedIssuer::owned())});
   let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let accepted=cx.consume_receiving_retained(grant,progress);cx.consume_fuel(progress.copied_items as u64);accepted?;result?;
   if case["semanticFailure"].as_bool().unwrap(){return Err(OriginalTurnFailure::from(refusal("same original semantic failure after native admission")))}Ok(())
  }));assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));assert_eq!(call.receipt_spent,external);assert_eq!(call.receipt().unwrap().remaining,slot.context.as_ref().unwrap().remaining_grant().unwrap());
  assert_eq!(output.is_some(),case["admitted"].as_bool().unwrap());if let Some(output)=output.as_ref(){assert_eq!(output.get().as_ptr(),raw_pointer);assert!(raw.is_none());}else{assert_eq!(raw.as_ref().unwrap().as_ptr(),raw_pointer);assert_eq!(heap,(0,0));}
  if case["semanticFailure"].as_bool().unwrap()&&case["admitted"].as_bool().unwrap(){assert!(matches!(call.result,Err(OriginalActorCallError::Semantic(_))));}else if case["admitted"].as_bool().unwrap(){assert!(call.result.is_ok());}
  close_source(&mut slot,&mut output,&mut epoch,policy);close_source(&mut slot,&mut raw,&mut epoch,policy);
 }
 let mut denied=false;for _ in 0..4096{if slot.terminal_is_empty(){break}epoch+=1;let input=retained_input(&slot,epoch);let context=slot.context.as_ref().unwrap();
  if context.input().is_none()&&!context.decoder.terminal_is_empty()&&!denied{let demand=context.retirement_demands(policy.maximum_copy_bytes).unwrap();for grant in [RetainedCloneGrant{maximum_items:0,..loan(policy,input.grant)},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..loan(policy,input.grant)},RetainedCloneGrant{maximum_depth:0,..loan(policy,input.grant)}]{let source=slot.context.as_ref().unwrap().source.as_ref().unwrap()as*const _;let(call,heap)=observe(||slot.close_step(input,grant));assert_eq!(call.receipt_spent,Default::default());assert_eq!(heap,(0,0));assert!(call.result.is_ok());assert_eq!(slot.context.as_ref().unwrap().source.as_ref().unwrap()as*const _,source);}denied=true;}
  let decoder_pending=!slot.context.as_ref().unwrap().decoder.terminal_is_empty();let(call,heap)=observe(||slot.close_step(input,loan(policy,input.grant)));assert_eq!(heap,(call.receipt_spent.retained_capacity_bytes,call.receipt_spent.released_bytes));assert!(call.result.is_ok(),"{:?}",call.result);if decoder_pending{assert!(slot.context.as_ref().unwrap().source.is_some());}
 }
 assert!(denied);assert!(slot.terminal_is_empty());assert_eq!(observe(||drop(slot)).1,(0,0));
}

#[test]
fn original_actor_live_and_close_epochs_preserve_source_and_decoder(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️source-drain.json")).unwrap();let first=RetainedTurnInput{operation:fixture["operation"].as_u64().unwrap(),generation:fixture["generation"].as_u64().unwrap(),epoch:1,work:declared_work(&fixture),grant:declared_grant(&fixture,"totalGrant")};let policy=declared_grant(&fixture,"callPolicy");let mut slot=OriginalActorContextSlot::empty();let mut epoch=1;
 loop{let input=if slot.context.is_none(){first}else{retained_input(&slot,epoch)};let call=slot.admit_step(input,loan(policy,input.grant),fixture_clock);if call.result.unwrap().ready{break}epoch+=1;assert!(epoch<=5);}
 let mut raw=Some(fixture["text"].as_str().unwrap().to_owned());let mut output:Option<SealedShared<String>>=None;let mut binding=None;let mut decoder=None;let mut phases=Vec::new();let mut records=Vec::new();let mut identity=None;
 for phase in fixture["phases"].as_array().unwrap(){
  epoch+=1;let input=retained_input(&slot,epoch);let grant=loan(policy,input.grant);let mut external=Default::default();
  let call=slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,fixture_clock,|cx|{
   let current=semio_framework_job::OriginalSourceBinding::borrow(cx);if let Some(original)=binding{assert!(original==current)}else{binding=Some(current)}
   if output.is_none(){let progress=cx.with_native_receiving::<_,ValueError>(|control|control.receive_shared(&mut raw,&mut output,semio_framework_value::shared_issuance::SharedIssuer::owned()))?;let issued=cx.receiving_issued_grant().unwrap();cx.consume_receiving_retained(issued,progress)?;cx.consume_fuel(progress.copied_items as u64);}
   let current=cx.native_receiving_identity().expect("actual native receiving source exists after paid admission");if let Some(original)=decoder{assert_eq!(original,current)}else{decoder=Some(current)}
   identity=Some(serde_json::json!({"operation":cx.operation().0,"generation":cx.generation().0,"parent":cx.original_source_parent()?.source_identity()?,"payload":format!("{:?}",cx.payload_authority_identity()),"decoder":current,"recipient":cx.original_recipient_identity(),"clock":cx.original_clock_binding(),"deadline":cx.original_absolute_deadline_us()}));
   if phase["cancelled"].as_bool().unwrap(){cx.observe_original_cancellation()?;cx.with_original_retirement(|cx|{assert!(binding.unwrap().matches(cx));assert_eq!(cx.native_receiving_identity(),decoder);Ok(())})?;}
   Ok::<_,OriginalTurnFailure>(())
  });assert!(call.result.is_ok(),"{:?}",call.result);assert_eq!(call.receipt_spent,external);assert_eq!(call.receipt().unwrap().remaining,slot.context.as_ref().unwrap().remaining_grant().unwrap());phases.push(phase["name"].as_str().unwrap());
  records.push(serde_json::json!({"role":phase["name"],"epoch":input.epoch,"identity":identity,"beforeStock":input.grant,"receipt":call.receipt_spent,"afterStock":slot.context.as_ref().unwrap().remaining_grant().unwrap(),"fuelBefore":input.work.fuel,"workSpent":call.work_units_spent,"fuelAfter":slot.context.as_ref().unwrap().remaining_work().unwrap().fuel}));
 }
 let mut refused=Vec::new();
 for axis in fixture["refusedAxes"].as_array().unwrap(){
  let input=retained_input(&slot,epoch+1);let mut supplied=input;let axis=axis.as_str().unwrap();
  match axis{"operation"=>supplied.operation+=1,"generation"=>supplied.generation+=1,"clock"=>supplied.work.clock_binding+=1,"deadline"=>supplied.work.absolute_deadline_us+=1,"fuel"=>supplied.work.fuel+=1,"stock"=>supplied.grant.maximum_items+=1,_=>unreachable!()}
  let mut external=Default::default();let call=slot.dispatch_step(supplied,StepBudget::new(1,supplied.work.absolute_deadline_us,loan(policy,supplied.grant)),&mut external,fixture_clock,|_|Ok::<_,OriginalTurnFailure>(()));assert!(matches!(call.result,Err(OriginalActorCallError::Authority(_))));assert_eq!(external,RetainedCloneProgress::default());
  refused.push(serde_json::json!({"axis":axis,"accepted":call.result.is_ok(),"receipt":call.receipt_spent,"input":input,"supplied":supplied}));
 }
 println!("[SOURCE-LIFETIME] {}",serde_json::json!({"records":records,"refused":refused}));
 assert_eq!(serde_json::to_value(phases).unwrap(),fixture["expectedPhases"]);assert!(decoder.is_some());close_source(&mut slot,&mut output,&mut epoch,policy);assert!(raw.is_none());
 for _ in 0..4096{if slot.terminal_is_empty(){break}epoch+=1;let input=retained_input(&slot,epoch);assert!(slot.close_step(input,loan(policy,input.grant)).result.is_ok());}assert!(slot.terminal_is_empty());
}

std::thread_local!{static EXPIRY_CLOCK:std::cell::Cell<u64>=const{std::cell::Cell::new(1)};}
fn expiry_clock()->Option<u64>{EXPIRY_CLOCK.with(|clock|Some(clock.get()))}
#[test]
fn original_actor_expired_cancelled_source_retires_without_renewing_authority(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⌛️source-expiry.json")).unwrap();EXPIRY_CLOCK.with(|clock|clock.set(fixture["initialClock"].as_u64().unwrap()));let first=RetainedTurnInput{operation:fixture["operation"].as_u64().unwrap(),generation:fixture["generation"].as_u64().unwrap(),epoch:1,work:declared_work(&fixture),grant:declared_grant(&fixture,"totalGrant")};let policy=declared_grant(&fixture,"callPolicy");let mut slot=OriginalActorContextSlot::empty();let mut epoch=1;
 loop{let input=if slot.context.is_none(){first}else{retained_input(&slot,epoch)};let call=slot.admit_step(input,loan(policy,input.grant),expiry_clock);if call.result.unwrap().ready{break}epoch+=1;assert!(epoch<=5);}
 let mut outputs:[Option<SealedShared<String>>;2]=[None,None];let mut binding=None;let mut decoder=None;
 for output in &mut outputs{epoch+=1;let input=retained_input(&slot,epoch);let grant=loan(policy,input.grant);let mut raw=Some(fixture["text"].as_str().unwrap().to_owned());let mut external=Default::default();let call=slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,expiry_clock,|cx|{
  binding=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));let progress=cx.with_native_receiving::<_,ValueError>(|control|control.receive_shared(&mut raw,output,semio_framework_value::shared_issuance::SharedIssuer::owned()))?;cx.consume_receiving_retained(cx.receiving_issued_grant().unwrap(),progress)?;cx.consume_fuel(progress.copied_items as u64);decoder=cx.native_receiving_identity();Ok::<_,OriginalTurnFailure>(())
 });assert!(call.result.is_ok());assert!(raw.is_none());}
 EXPIRY_CLOCK.with(|clock|clock.set(fixture["expiredClock"].as_u64().unwrap()));let pointer=outputs[0].as_ref().unwrap().get().as_ptr();epoch+=1;let unscoped_input=retained_input(&slot,epoch);let mut unscoped_external=Default::default();let unscoped=slot.dispatch_step(unscoped_input,StepBudget::new(64,unscoped_input.work.absolute_deadline_us,loan(policy,unscoped_input.grant)),&mut unscoped_external,expiry_clock,|cx|{
  assert!(binding.unwrap().matches(cx));assert!(cx.should_yield());assert!(cx.retirement_should_yield());let step=semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|panic!("expired semantic scope cannot enter physical owner"))?;assert_eq!(step.progress(),RetainedCloneProgress::default());Ok::<_,OriginalTurnFailure>(())
 });assert!(unscoped.result.is_ok());assert_eq!(unscoped.receipt_spent,RetainedCloneProgress::default());assert_eq!(unscoped.work_units_spent,0);assert_eq!(outputs[0].as_ref().unwrap().get().as_ptr(),pointer);assert!(fixture["unscopedBlocked"].as_bool().unwrap());epoch+=1;let input=retained_input(&slot,epoch);let grant=loan(policy,input.grant);let mut external=Default::default();let zero=slot.dispatch_step(input,StepBudget::new(0,input.work.absolute_deadline_us,grant),&mut external,expiry_clock,|cx|cx.with_original_retirement(|cx|{
  assert!(binding.unwrap().matches(cx));assert_eq!(cx.native_receiving_identity(),decoder);assert!(cx.should_yield());assert!(cx.retirement_should_yield());let step=semio_framework_job::close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|_|panic!("zero original fuel cannot enter physical owner"))?;assert_eq!(step.progress(),RetainedCloneProgress::default());Ok(())
 }));assert!(zero.result.is_ok());assert_eq!(zero.receipt_spent,RetainedCloneProgress::default());assert_eq!(zero.work_units_spent,0);assert_eq!(outputs[0].as_ref().unwrap().get().as_ptr(),pointer);assert!(fixture["zeroFuelBlocked"].as_bool().unwrap());let mut records=Vec::new();
 for (case,output) in fixture["cases"].as_array().unwrap().iter().zip(&mut outputs){let mut close=RetainedCloneClose::default();let mut released=0;
  for _ in 0..64{if output.is_none()&&close.is_empty(){break}epoch+=1;let input=retained_input(&slot,epoch);let grant=loan(policy,input.grant);let mut external=Default::default();let call=slot.dispatch_step(input,StepBudget::new(64,input.work.absolute_deadline_us,grant),&mut external,expiry_clock,|cx|cx.with_original_retirement(|cx|{
   assert!(cx.should_yield());assert!(!cx.retirement_should_yield());assert!(binding.unwrap().matches(cx));assert_eq!(cx.native_receiving_identity(),decoder);assert_eq!(cx.original_absolute_deadline_us(),first.work.absolute_deadline_us);assert_eq!(cx.original_clock_binding(),Some(first.work.clock_binding));if case["cancelled"].as_bool().unwrap(){cx.observe_original_cancellation()?;}
   let demand=close.next_owner_demands::<SealedShared<String>>(output.is_some(),&None,grant.maximum_copy_bytes)?;let step=cx.with_native_retirement(|control|{control.charge(demand.capacity_bytes)?;if close.is_empty(){close.begin_granted(output,grant)?.ok_or_else(||refusal("expired original close denied its paid birth"))}else{close.step_granted(grant)}})?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(())
  }));assert!(call.result.is_ok(),"{:?}",call.result);assert_eq!(call.receipt_spent,external);released+=external.released_bytes;assert_eq!(call.receipt().unwrap().remaining,slot.context.as_ref().unwrap().remaining_grant().unwrap());}
  assert!(output.is_none()&&close.is_empty());assert!(released>0);records.push(serde_json::json!({"name":case["name"],"cancelled":case["cancelled"],"releasedBytes":released,"deadline":first.work.absolute_deadline_us,"clockBinding":first.work.clock_binding,"observedClock":expiry_clock()}));
 }
 println!("[SOURCE-EXPIRY] {}",serde_json::json!({"zeroFuelBlocked":true,"zeroFuelReceipt":zero.receipt_spent,"unscopedBlocked":true,"unscopedReceipt":unscoped.receipt_spent,"records":records}));
 for _ in 0..4096{if slot.terminal_is_empty(){break}epoch+=1;let input=retained_input(&slot,epoch);assert!(slot.close_step(input,loan(policy,input.grant)).result.is_ok());}assert!(slot.terminal_is_empty());
}
