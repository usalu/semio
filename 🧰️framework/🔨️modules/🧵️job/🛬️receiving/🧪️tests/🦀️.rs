//! 🛬️ Genuine context laws preserve the original decoder, complete receipts and finite source.
use semio_framework_job_context::*;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value_testing_allocator as _;
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedWorkOutcome,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource}};
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value_native_owned::{NativeReceive,NativeReceiver,OriginalReceivingPrimary};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeRetirementDecodeControl,NativePreparedDecodeControl,NativeReceivingEffect,NativeReceivingEffectError,NativeReceivingEffectPoll,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_native_owned::owned::NativeAdvanceSelection;
use semio_framework_test_fixtures::fixture_text;
fn bank()->RetainedCloneGrant{let row:serde_json::Value=serde_json::from_str(include_str!("../♻️close/📐️.json")).unwrap();let s=&row["stock"];RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize}}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:a.copied_items-b.copied_items,copied_bytes:a.copied_bytes-b.copied_bytes,retained_capacity_bytes:a.retained_capacity_bytes-b.retained_capacity_bytes,released_bytes:a.released_bytes-b.released_bytes}}
fn native_failure(primary:OriginalReceivingPrimary,receipt_refusal:Option<ValueError>)->OriginalTurnFailure{OriginalTurnFailure{effects:primary.retained_progress(),primary:OriginalTurnPrimary::Receiving(primary),receipt_refusal,context_refusal:None,work_refusal:None}}
fn source_received(outcome:Option<OriginalNativeReceivingOutcome>){let outcome=outcome.expect("funded original Source outcome is retained");assert!(outcome.call.is_ok(),"original decoder call refused");assert!(outcome.source_refusal.is_none(),"independent Source posting refused")}
fn receive_step(cx:&mut StepContext<'_>,operation:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>)->Result<RetainedCloneStep,OriginalTurnFailure>{
 let mut outcome=None;let mut child=None;
 if !cx.with_native_receiving(&mut outcome,|control,body_grant|{child=Some(operation(control,body_grant));Ok(())})?{assert!(outcome.is_none()&&child.is_none());return Ok(RetainedCloneStep::Progress(Default::default()))}
 source_received(outcome);child.expect("original child result was retained before Source posting").result.map_err(|primary|native_failure(primary,None))
}
fn polled<E>(poll:RetainedWorkPoll<RetainedCloneStep,E>)->RetainedWorkOutcome<RetainedCloneStep,E>{match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}}
fn settle<E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,run:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkOutcome<RetainedCloneStep,E>{
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|run(&mut *effect,reply).map(|_|()));let work=effect.performed_work();
 match(ran,slot.take()){(Err(error),_)=>RetainedWorkOutcome{result:Err(error.into()),work},(Ok(()),Some(result))=>RetainedWorkOutcome{result,work},(Ok(()),None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}}
}
fn effect_step<E:NativeReceivingEffectError>(control:&mut NativeDecodeControl<'_>,demand:RetirementDemand,work:u64,frame:&mut NativeReceivingEffect<E>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 match control.receive_effect(demand,work,frame,|_,effect|run(effect)){Ok(NativeReceivingEffectPoll::Pending{work})=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},Ok(NativeReceivingEffectPoll::Complete(turn))=>RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.receiving),work:turn.work},Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}}
}
fn retirement_effect_step<E:NativeReceivingEffectError>(control:&mut NativeRetirementDecodeControl<'_,'_>,demand:RetirementDemand,work:u64,frame:&mut NativeReceivingEffect<E>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 match control.receive_effect(demand,work,frame,|_,effect|run(effect)){Ok(NativeReceivingEffectPoll::Pending{work})=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},Ok(NativeReceivingEffectPoll::Complete(turn))=>RetainedWorkOutcome{result:turn.result.map_err(|failure|failure.receiving),work:turn.work},Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}}
}
fn warm(caller:&mut OriginalJobFixtureCaller,owner:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant){for _ in 0..64{let mut outcome=None;if caller.refreshed_turn_with_decoder(grant,owner,|cx|cx.with_native_receiving(&mut outcome,|_,_|Ok(()))).unwrap(){return}}panic!("original receiving never became ready")}
fn close_decoder(caller:&mut OriginalJobFixtureCaller,owner:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant){for _ in 0..10000{if owner.terminal_is_empty(){return}caller.refreshed_turn(grant,|cx|{owner.close_on_source(cx)?;Ok(())}).unwrap();}panic!("original decoder exceeded its existing close bound")}
fn copy_native_text(caller:&mut OriginalJobFixtureCaller,owner:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,text:&str)->String{
 use semio_framework_value::OriginalRetirementFailure;
 let mut admission=RetainedCloneSource::<String>::admission_owned(text.to_owned(),());let mut source=None;let mut effect=NativeReceivingEffect::default();
 for _ in 0..10000{
  if source.is_some()&&!effect.has_pending_recovery(){break}
  caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{
   let turn=if effect.has_pending_recovery(){control.finish_receiving(&mut effect)}else{let demand=admission.next_demand().unwrap();let ready=admission.phase()=="ready";let work=if ready{1}else{admission.advance_work_demand().unwrap()};control.receive_allocation_effect(demand,&mut effect,demand.capacity_bytes!=0,work,|e|if ready{admission.take_output(&mut source,e)}else{admission.advance(e)})};
   RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
  })?;Ok(())}).unwrap();
 }
 assert!(source.is_some());assert!(admission.terminal_is_empty());assert!(!effect.has_pending_recovery());
 let mut receiver=<String as NativeReceive<String>>::receiver();let mut advance_frame=NativeReceivingEffect::<OriginalReceivingPrimary>::default();
 for _ in 0..10000{if receiver.output_ready(){break}caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{
  let maximum=match control.available_work_units(){Ok(n)=>n,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};let input=source.as_ref().unwrap().borrow();let body=g.maximum_copy_bytes;
  let demand=match receiver.demands(input,body,maximum){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  let NativeAdvanceSelection::Local{work}=(match receiver.select_advance(input,body,maximum){Ok(s)=>s,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}})else{panic!("a string receiver advances locally")};
  effect_step(control,demand,work.max(1),&mut advance_frame,|effect|settle(effect,|e,r|receiver.advance_local(input,body,e,r)))
 })?;Ok(())}).unwrap();}
 assert!(receiver.output_ready());let mut output=None;let mut take_frame=NativeReceivingEffect::<OriginalReceivingPrimary>::default();
 for _ in 0..10000{if output.is_some(){break}caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{
  let d=receiver.take_demand().unwrap();let q=receiver.take_work_demand().unwrap();effect_step(control,d,q.max(1),&mut take_frame,|effect|settle(effect,|e,r|receiver.take(&mut output,e,r)))
 })?;Ok(())}).unwrap();}
 assert!(output.is_some());
 let mut close_effect=NativeReceivingEffect::<OriginalRetirementFailure>::default();
 caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,_|{effect_step(control,receiver.begin_close_demand().unwrap(),receiver.begin_close_work_demand().unwrap().max(1),&mut close_effect,|effect|settle(effect,|e,r|receiver.begin_close_into(e,r)))})?;Ok(())}).unwrap();
 for _ in 0..10000{if receiver.terminal_is_empty()&&!close_effect.has_pending_recovery(){break}caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{
  if close_effect.has_pending_recovery(){let turn=control.finish_retirement_receiving(&mut close_effect);return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
  let body=g.maximum_copy_bytes;let d=receiver.close_demands(body).unwrap();let q=receiver.close_work_demand(body).unwrap();effect_step(control,d,q.max(1),&mut close_effect,|effect|settle(effect,|e,r|receiver.close_step(body,e,r)))
 })?;Ok(())}).unwrap();}
 assert!(receiver.terminal_is_empty());
 for _ in 0..10000{if source.as_ref().unwrap().terminal_is_empty()&&!close_effect.has_pending_recovery(){break}caller.refreshed_turn_with_decoder(grant,owner,|cx|{receive_step(cx,|control,g|{
  if close_effect.has_pending_recovery(){let turn=control.finish_retirement_receiving(&mut close_effect);return RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
  let body=g.maximum_copy_bytes;let d=source.as_ref().unwrap().next_close_demand(body).unwrap();let q=source.as_ref().unwrap().next_close_work_demand(body).unwrap();effect_step(control,d,q.max(1),&mut close_effect,|effect|polled(source.as_mut().unwrap().close_step(body,effect)))
 })?;Ok(())}).unwrap();}
 assert!(source.as_ref().unwrap().terminal_is_empty());assert!(!close_effect.has_pending_recovery());drop(source);output.take().unwrap()
}
#[test]
fn original_context_charges_child_construction_and_enclosing_semantic_once(){let mut shared_allocation_frame=semio_framework_value::native_decoding::NativeSharedAdmission::default();
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🛬️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["receiptLaw"];assert_eq!(law["childCopy"].as_u64().unwrap()+law["parentCopy"].as_u64().unwrap(),law["expectedCopy"].as_u64().unwrap());assert_eq!(law["childCapacity"],law["expectedCapacity"]);
 let stock=bank();let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,logical_now_us,u64::MAX,10000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);
 for _ in 0..2{let mut raw=Some(String::from("π 🌿"));let mut output=None;
  for _ in 0..10000{if output.is_some(){break}caller.refreshed_turn_with_decoder(grant,&mut owner,|cx|{let before=cx.retained_progress();let issued=cx.retained_grant();let mut work=0;let mut outcome=None;let mut child=None;let(result,heap)=observe_heap(||cx.with_native_receiving(&mut outcome,|control,_|{let turn=control.receive_shared(&mut raw,&mut output,semio_framework_value::shared_issuance::SharedIssuer::owned(),&mut shared_allocation_frame);work=turn.work;child=Some(turn.result);Ok(())}));assert!(result?);source_received(outcome);let p=child.expect("original child result was retained before Source posting").map_err(|error|native_failure(error,None))?;assert_eq!((heap.0,heap.1),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(cx.receiving_child_progress(),p);let mut enclosing=p;let mut marker=[0u8;1];marker.copy_from_slice(&[1]);assert_eq!(marker,[1]);enclosing.copied_bytes+=law["parentCopy"].as_u64().unwrap()as usize;let receipt=cx.consume_receiving_retained(issued,enclosing);cx.consume_fuel(u64::try_from(work).unwrap());cx.consume_fuel(1);receipt?;assert_eq!(delta(cx.retained_progress(),before),enclosing);assert_eq!(cx.receiving_child_progress(),Default::default());Ok(())}).unwrap();}assert!(output.is_some());assert!(raw.is_none());
  let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::retain(output.take().unwrap());for _ in 0..10000{if close.terminal_is_empty(){break}caller.refreshed_turn(grant,|cx|{let body=cx.retained_grant().maximum_copy_bytes;let demand=close.selected_demand(body)?;close_original_effect_step(cx,demand,|effect|close.step(body,effect))?;Ok(())}).unwrap();}assert!(close.terminal_is_empty());
 }
 close_decoder(&mut caller,&mut owner,grant);caller.retire(stock);
}
fn retire_original_text(close:&mut ControlledRetirement<String>,frame:&mut NativeReceivingEffect<semio_framework_value::OriginalRetirementFailure>,control:&mut NativeRetirementDecodeControl<'_,'_>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
 let turn=if frame.has_pending_recovery(){control.finish_retirement_receiving(frame)}else{
  let body=grant.maximum_copy_bytes;let d=match close.selected_demand(body){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};let q=match close.next_computational_work(body){Ok(q)=>q,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
  return retirement_effect_step(control,d,q.max(1),frame,|effect|polled(close.step(body,effect)))
 };
 RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}
}
#[test]
fn original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../🛬️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus["retirementCases"].as_array().unwrap(){
  let stock=bank();let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(75),Generation(4),stock,logical_now_us,u64::MAX,10000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);let text=copy_native_text(&mut caller,&mut owner,grant,"π 🌿");let identity=owner.receiving_identity();assert_eq!(serde_json::to_value(&text).unwrap(),serde_json::json!("π 🌿"));let mut close=ControlledRetirement::retain(text);
  if case["cancelled"].as_bool().unwrap(){caller.cancel().cancel_now()}
  let mut frame=NativeReceivingEffect::<semio_framework_value::OriginalRetirementFailure>::default();let visited=std::cell::Cell::new(false);let admitted=owner.owned_bytes();let mut initial=Default::default();
  caller.refreshed_turn_with_decoder(grant,&mut owner,|cx|{
   let deadline=if case["deadlineOpen"].as_bool().unwrap(){cx.deadline_us()}else{0};let before=cx.retained_progress();
   let result:Result<(),OriginalTurnFailure>=cx.with_original_work_loan(case["fuel"].as_u64().unwrap(),deadline,|cx|cx.with_original_retirement(|cx|{
    let mut outcome=None;let mut child=None;
    let(result,heap)=observe_heap(||{let quote=(cx.retained_grant(),cx.fuel_remaining());cx.with_native_retirement_receiving(&mut outcome,quote,|control,body_grant|{visited.set(true);assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);child=Some(retire_original_text(&mut close,&mut frame,control,body_grant));Ok(())})});
    match result{Ok(true)=>{source_received(outcome);let turn=child.expect("actual retirement result is retained");let p=turn.result.unwrap().progress();assert_eq!((heap.0,heap.1),(p.retained_capacity_bytes,p.released_bytes));},Ok(false)=>{assert!(outcome.is_none()&&child.is_none());assert_eq!((heap.0,heap.1),(0,0));},Err(failure)=>{assert!(!case["visited"].as_bool().unwrap());assert_eq!(failure.retained_progress(),Default::default());assert!(outcome.is_none()&&child.is_none());assert_eq!((heap.0,heap.1),(0,0));}}Ok(())
   }));
   initial=delta(cx.retained_progress(),before);if let Err(failure)=result{assert!(!case["visited"].as_bool().unwrap());assert_eq!(failure.retained_progress(),Default::default())}Ok(())
  }).unwrap();assert_eq!(visited.get(),case["visited"].as_bool().unwrap());assert_eq!(owner.owned_bytes()-admitted,initial.retained_capacity_bytes);
  for _ in 0..10000{
   if close.terminal_is_empty()&&!frame.has_pending_recovery(){break}
   let admitted=owner.owned_bytes();let mut actual=Default::default();caller.refreshed_turn_with_decoder(grant,&mut owner,|cx|cx.with_original_retirement(|cx|{
    let before=cx.retained_progress();let mut outcome=None;let mut child=None;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let body=cx.retained_grant().maximum_copy_bytes;let quote=StepContext::native_retirement_quote(close.selected_demand(body)?,close.next_computational_work(body)?.max(1));cx.with_native_retirement_receiving(&mut outcome,quote,|control,body_grant|{assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);child=Some(retire_original_text(&mut close,&mut frame,control,RetainedCloneGrant{maximum_copy_bytes:body,..body_grant}));Ok(())})});assert!(result?);source_received(outcome);let p=child.expect("funded original retirement ran").result.unwrap().progress();actual=delta(cx.retained_progress(),before);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert!(actual.fits(grant));Ok(())
   })).unwrap();assert_eq!(owner.owned_bytes()-admitted,actual.retained_capacity_bytes);
  }
  assert!(close.terminal_is_empty()&&!frame.has_pending_recovery());assert_eq!(owner.receiving_identity(),identity);close_decoder(&mut caller,&mut owner,grant);caller.retire(stock);
 }
}
#[test]
fn original_receiving_ready_scope_settles_a_ready_step_and_refuses_a_suspended_one(){
 let stock=bank();let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};let mut caller=OriginalJobFixtureCaller::admit(OperationId(76),Generation(5),stock,logical_now_us,u64::MAX,10000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);
 caller.refreshed_turn_with_decoder(grant,&mut owner,|cx|{
  let mut outcome=None;let mut value=None;assert!(cx.with_native_receiving_ready(&mut outcome,async|_,_|{value=Some(7u8);Ok(())})?);assert_eq!(value,Some(7));source_received(outcome);
  let mut suspended=None;assert!(cx.with_native_receiving_ready(&mut suspended,async|_,_|std::future::pending::<Result<(),ValueError>>().await)?);
  let refused=suspended.expect("suspension refusal stays in its preborn Source outcome");assert!(refused.source_refusal.is_none());let call=refused.call.err().expect("a suspended native step must refuse");let primary=call.context.as_ref().or(call.operation.as_ref()).expect("original suspension cause is retained");assert_eq!(primary.kind,ValueRefusalKind::InvariantViolated);assert!(call.output.is_none());Ok(())
 }).unwrap();close_decoder(&mut caller,&mut owner,grant);caller.retire(stock);
}

#[test]
fn native_receiving_uses_the_actual_current_cpu_loan_without_a_second_debit(){
 let row:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/case.schema.json")).unwrap();for(k,v)in row.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}
 let stock=bank();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7680),Generation(3),stock,||Some(0),10000,10000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);let bytes:Vec<u8>=serde_json::from_value(row["probe"].clone()).unwrap();
 caller.refreshed_turn_with_decoder(stock,&mut owner,|cx|cx.with_original_work_loan(row["quantum"].as_u64().unwrap(),10000,|cx|{
  let before=cx.original_work_units_spent();let mut outcome=None;let mut child=None;
  let(result,heap)=observe_heap(||cx.with_native_receiving(&mut outcome,|control,_|{
   let start=control.actual_work();assert_eq!(control.available_work_units()?,row["childQuantum"].as_u64().unwrap());assert!(control.prepare_work(1)?);let valid=std::str::from_utf8(&bytes).is_ok();control.commit_work(1)?;assert_eq!(control.prepare_work(1)?,row["postScopeAdditionalWork"].as_bool().unwrap());child=Some((valid,control.actual_work()-start));Ok(())
  }));assert!(result?);source_received(outcome);assert_eq!([heap.0,heap.1],[0,0]);let(valid,work)=child.expect("native probe completed inside the actual Source loan");assert_eq!(valid,row["valid"].as_bool().unwrap());assert_eq!(work,u128::from(row["actualWork"].as_u64().unwrap()));assert_eq!(cx.original_work_units_spent()-before,u128::from(row["sourceWork"].as_u64().unwrap()));Ok(())
 })).unwrap();close_decoder(&mut caller,&mut owner,stock);caller.retire(stock);
}

#[test]
fn original_context_accepts_metadata_of_a_wide_loan_without_inventing_allocation(){
 use semio_framework_value_native_owned::native_decoding::NativeReceivingEffect;
 let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),stock,logical_now_us,u64::MAX,1000000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);let mut frame=NativeReceivingEffect::default();
 caller.refreshed_turn_with_decoder(stock,&mut owner,|cx|{let issued=cx.retained_grant();assert!(issued.maximum_capacity_bytes>=26000);let before=cx.retained_progress();let step=receive_step(cx,|control,_|{let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};let turn=control.receive_allocation(d,&mut frame,false,1,|g,_|{if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}let mut byte=[0];byte.copy_from_slice(&[1]);assert_eq!(byte,[1]);RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})),work:1}});RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;
  assert_eq!(step.progress().retained_capacity_bytes,0);assert_eq!(delta(cx.retained_progress(),before).retained_capacity_bytes,0);assert_eq!(step.progress().copied_bytes,1);Ok(())
 }).unwrap();assert!(!frame.has_pending_recovery());close_decoder(&mut caller,&mut owner,stock);caller.retire(stock);
}

#[test]
fn cancelled_source_recovers_actual_pending_child_with_same_construction_issuer(){
 use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffect,NativeReceivingEffectPhase};
 let stock=bank();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7690),Generation(3),stock,||Some(0),10000,10000);let mut owner=caller.admitted_decoder();warm(&mut caller,&mut owner,stock);let mut frame=NativeReceivingEffect::<ValueError>::default();let mut reply=None;
 caller.refreshed_turn_with_decoder(stock,&mut owner,|cx|{
  let mut outcome=None;assert!(cx.with_native_receiving(&mut outcome,|control,_|{
   let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.receive_allocation_effect(RetirementDemand{depth:1,..Default::default()},&mut frame,false,1,|effect|{
    assert!(effect.performed(1).unwrap());reply=Some(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1});panic!("retained child boundary")
   })));assert!(interrupted.is_err());Ok(())
  })?);Ok(())
 }).unwrap();assert_eq!(frame.phase(),NativeReceivingEffectPhase::EffectStarted);let identity=owner.receiving_identity();caller.cancel().cancel_now();
 let mut denied=None;let mut visited=false;let narrow=RetainedCloneGrant{maximum_items:1,..stock};
 caller.refreshed_turn_with_decoder(narrow,&mut owner,|cx|{let quote=(cx.retained_grant(),cx.fuel_remaining());assert!(!cx.with_native_retirement_receiving(&mut denied,quote,|_,_|{visited=true;Ok(())})?);Ok(())}).unwrap();assert!(!visited&&denied.is_none());
 let mut outcome=None;
 caller.refreshed_turn_with_decoder(stock,&mut owner,|cx|cx.with_original_retirement(|cx|{
  let before=cx.retained_progress();let work=cx.original_work_units_spent();
  assert!(cx.with_native_retirement_receiving(&mut outcome,StepContext::native_retirement_quote(RetirementDemand{depth:1,..Default::default()},1),|control,body_grant|{
   assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);
   control.resolve_unallocated_receiving_child(&mut frame,reply.take().unwrap()).unwrap();
   let turn=control.finish_receiving(&mut frame);assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap().copied_items,1);Ok(())
  })?);
  let used=delta(cx.retained_progress(),before);let placement=OriginalNativeReceivingOutcome::<()>::placement();assert_eq!(used.copied_items,placement.copied_items+1);assert_eq!(used.copied_bytes,placement.copied_bytes);assert_eq!(cx.original_work_units_spent()-work,3);
  assert!(cx.receiving_issued_grant().is_none());assert_eq!(cx.receiving_child_progress(),Default::default());Ok(())
 })).unwrap();
 assert!(outcome.as_ref().unwrap().call.is_ok());assert!(outcome.as_ref().unwrap().source_refusal.is_none());assert!(reply.is_none());assert_eq!(frame.phase(),NativeReceivingEffectPhase::Empty);assert_eq!(owner.receiving_identity(),identity);
 let typed_fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧳️typed-source.json")).unwrap();
 let expected=typed_fixture["value"].as_u64().unwrap();let mut typed=None;let mut typed_visited=false;
 let exact=OriginalNativeReceivingOutcome::<u64>::typed_placement().unwrap();let narrow=RetainedCloneGrant{maximum_copy_bytes:exact.copied_bytes-1,..stock};
 caller.refreshed_turn_with_decoder(narrow,&mut owner,|cx|cx.with_original_retirement(|cx|{
  let quote=(cx.retained_grant(),cx.fuel_remaining());assert!(!cx.with_native_retirement_receiving_value(&mut typed,quote,|_,_|{typed_visited=true;Ok(expected)})?);Ok(())
 })).unwrap();assert!(!typed_visited&&typed.is_none());
 for _ in 0..stock.maximum_items{if typed.is_some(){break}caller.refreshed_turn_with_decoder(stock,&mut owner,|cx|cx.with_original_retirement(|cx|{
  let before=cx.retained_progress();
  let(result,witness)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_retirement_receiving_value(&mut typed,StepContext::native_retirement_quote(RetirementDemand{depth:1,..Default::default()},1),|control,_grant|{
   typed_visited=true;assert_eq!(Some((control.ledger_identity(),control.retirement_recipient_identity().unwrap())),identity);Ok(expected)
  }));
  assert!(!witness.overflowed);let heap=(witness.requested_bytes,witness.released_bytes);
  let _=result?;let used=delta(cx.retained_progress(),before);assert_eq!((heap.0,heap.1),(used.retained_capacity_bytes,used.released_bytes));Ok(())
 })).unwrap();}
 assert!(typed_visited);let typed=typed.expect("typed Source outcome retained");assert!(typed.source_refusal.is_none());assert_eq!(serde_json::to_value(typed.call.unwrap()).unwrap(),typed_fixture["value"]);
 close_decoder(&mut caller,&mut owner,stock);caller.retire(stock);
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
