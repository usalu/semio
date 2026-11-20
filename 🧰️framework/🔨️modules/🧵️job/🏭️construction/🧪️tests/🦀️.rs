//! 🧫️ The lower native gate consumes the same neutral axes and authentic fixture roots.
use semio_framework_value_testing_allocator as _;
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::OriginalSourceBinding;
use crate::{OriginalJobFixtureCaller,OriginalJobDecodeOwner};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100_000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn clock()->Option<u64>{Some(1)}
fn finish(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..10000{if decoder.terminal_is_empty(){return}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}panic!("original construction decoder close must terminate")}
#[test]
fn defining_five_currency_quotes_match_the_language_neutral_cases(){
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){let q=|key:&str,index:usize|row[key][index].as_u64().unwrap()as usize;let demand=OriginalConstructionDemand{items:q("quote",0),copy_bytes:q("quote",1),capacity_bytes:q("quote",2),release_bytes:q("quote",3),depth:q("quote",4)};let grant=RetainedCloneGrant{maximum_items:q("grant",0),maximum_copy_bytes:q("grant",1),maximum_capacity_bytes:q("grant",2),maximum_release_bytes:q("grant",3),maximum_depth:q("grant",4)};assert_eq!(demand.fits(grant),row["accepted"].as_bool().unwrap(),"{}",row["name"]);}
}
#[test]
fn distinct_authentic_roots_with_identical_semantic_ids_do_not_match(){
 let mut original=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock(),clock,u64::MAX,10000);
 let mut foreign=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock(),clock,u64::MAX,10000);
 let binding=original.turn(stock(),|cx|Ok(OriginalSourceBinding::borrow(cx))).unwrap();
 original.turn(stock(),|cx|{assert!(binding.matches(cx));Ok(())}).unwrap();assert_eq!(original.receipt(),Default::default());
 let(result,heap)=observe_heap(||foreign.turn(stock(),|cx|{assert!(!binding.matches(cx));Ok(())}));result.unwrap();assert_eq!(heap,(0,0));assert_eq!(foreign.receipt(),Default::default());
 foreign.retire(stock());original.retire(stock());
}

fn bind(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,admission:&mut OriginalConstructionAdmission){for _ in 0..16{if caller.turn(stock(),|cx|Ok(decoder.admit_step(cx)?)).unwrap(){break}}assert!(decoder.source_loan_admitted());let mut slot=None::<()>;caller.turn_with_decoder(stock(),decoder,|cx|{assert_eq!(admit_original_construction(cx,&mut slot,admission,OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1},1,|_,_|panic!("binding does not enter constructor"))?,OriginalConstructionStatus::Pending);Ok(())}).unwrap();assert!(admission.binding.is_some())}
fn retire<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,original:&mut Option<T>){
 let mut owner=None;
 caller.turn(stock(),|cx|{let bytes=std::mem::size_of::<Option<T>>()+std::mem::size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>();assert!(construction_metadata(cx,bytes,2)?);owner=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(original.take().unwrap()));Ok(())}).unwrap();
 for _ in 0..10000{
  let retirement=owner.as_mut().unwrap();
  if retirement.terminal_is_empty(){break}
  caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{
   let body=cx.retained_grant().maximum_copy_bytes;
   let demand=retirement.next_demand(body)?;
   crate::close_original_effect_step(cx,demand,|effect|retirement.step(body,effect)).map(|_|())
  })).unwrap();
 }
 assert!(owner.as_ref().unwrap().terminal_is_empty(),"original construction output retirement must terminate");
 caller.turn(stock(),|cx|{assert!(construction_metadata(cx,std::mem::size_of_val(&owner),1)?);owner=None;Ok(())}).unwrap();
}
fn close_admission(caller:&mut OriginalJobFixtureCaller,admission:&mut OriginalConstructionAdmission){for _ in 0..10000{if admission.terminal_is_empty(){return}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{admission.close_step(cx)?;Ok(())})).unwrap();}panic!("construction control close must terminate")}
#[test]
fn over_quote_preserves_the_born_candidate_and_actual_original_decoder_receipt(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(73),Generation(5),stock(),clock,u64::MAX,200000);let mut decoder=OriginalJobDecodeOwner::empty();let mut admission=OriginalConstructionAdmission::default();bind(&mut caller,&mut decoder,&mut admission);let mut output:Option<Box<[u8;70000]>>=None;
 let demand=OriginalConstructionDemand{items:1,copy_bytes:70000,capacity_bytes:69999,release_bytes:0,depth:1};
 let before=caller.cumulative_receipt();let(mut requested,mut released)=(0,0);let mut result=Ok(OriginalConstructionStatus::Pending);for _ in 0..16{let(turn,heap)=observe_heap(||caller.turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction(cx,&mut output,&mut admission,demand,1,|_,actual|{let value=Box::new([7;70000]);*actual=Some(70000);semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:70000,retained_capacity_bytes:70000,..Default::default()})),work:1}})));requested+=heap.0;released+=heap.1;result=turn;if !matches!(result,Ok(OriginalConstructionStatus::Pending)){break}}
 assert_eq!(result.unwrap(),OriginalConstructionStatus::Failed);assert!(admission.is_failed());let failure=admission.child.as_ref().unwrap().as_ref().unwrap_err();assert!(failure.allocation_failure.is_some()||failure.native_allocation.is_some());assert_eq!(decoder.owned_bytes(),demand.capacity_bytes);let after=caller.cumulative_receipt();assert_eq!((after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes),(requested,released));assert_eq!(failure.allocation_failure.as_ref().and_then(|failure|failure.progress.actual_capacity_bytes).or_else(||failure.native_allocation.as_ref().and_then(|failure|failure.actual)),Some(70000));assert_eq!(output.as_deref().unwrap()[69999],7);
 let mut failure=None;caller.turn(stock(),|cx|{assert!(admission.take_failure_into_step(&mut failure,cx)?);Ok(())}).unwrap();retire(&mut caller,&mut output);retire(&mut caller,&mut failure);close_admission(&mut caller,&mut admission);finish(&mut caller,&mut decoder);caller.retire(stock());
}

fn actual_byte_birth(_: &mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_, '_>,actual:&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{
 let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<u8>())};*actual=Some(usize::from(!raw.is_null()));
 if raw.is_null(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"actual original byte birth refused")),work:1}}
 unsafe{raw.write(7);}semio_framework_value::RetainedWorkOutcome{result:Ok((unsafe{Box::from_raw(raw)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}
}
thread_local!{static REFUSED_ATTEMPTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn refused_byte_birth(_: &mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_, '_>,actual:&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{
 let(raw,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||unsafe{std::alloc::alloc(std::alloc::Layout::new::<u8>())});REFUSED_ATTEMPTS.with(|cell|cell.set(attempts));*actual=Some(usize::from(!raw.is_null()));
 if raw.is_null(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"actual original byte birth refused")),work:1}}
 unsafe{raw.write(7);}semio_framework_value::RetainedWorkOutcome{result:Ok((unsafe{Box::from_raw(raw)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}
}

#[test]
fn actual_native_construction_forwards_null_attempt_work_without_inventing_physical_items(){
 let row:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&row["workLaw"];let controller:serde_json::Value=serde_json::from_str(include_str!("admission.json")).unwrap();let controller=&controller["controller"];let overhead_work=controller["scopeAdmissionWork"].as_u64().unwrap()+controller["sourceWork"].as_u64().unwrap()+controller["captureWork"].as_u64().unwrap();let overhead_items=controller["scopeAdmissionItems"].as_u64().unwrap()+controller["sourceItems"].as_u64().unwrap()+controller["captureItems"].as_u64().unwrap();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7542),Generation(2),stock(),clock,u64::MAX,100000);let mut decoder=OriginalJobDecodeOwner::empty();let mut admission=OriginalConstructionAdmission::default();let mut output=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:1,release_bytes:0,depth:1};caller.admit_return_table();bind(&mut caller,&mut decoder,&mut admission);
 caller.turn_with_decoder(RetainedCloneGrant{maximum_items:0,..stock()},&mut decoder,|cx|{let before=cx.original_work_units_spent();assert_eq!(admit_original_construction(cx,&mut output,&mut admission,demand,1,actual_byte_birth)?,OriginalConstructionStatus::Pending);assert_eq!(cx.original_work_units_spent()-before,u128::from(law["deniedWork"].as_u64().unwrap()));assert!(output.is_none());Ok(())}).unwrap();
 for _ in 0..16{if caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let(result,heap)=observe_heap(||admit_original_construction(cx,&mut output,&mut admission,demand,1,refused_byte_birth));let result=result?;if result==OriginalConstructionStatus::Pending{return Ok(false)}let attempts=REFUSED_ATTEMPTS.with(|cell|cell.take());assert_eq!(attempts,1);assert_eq!(result,OriginalConstructionStatus::Failed);assert!(admission.is_failed());assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()+overhead_work));assert_eq!(cx.retained_progress().copied_items-physical.copied_items,(law["failedItems"].as_u64().unwrap()+overhead_items)as usize);assert!(output.is_none());assert_eq!((heap.0,heap.1),(0,0));Ok(true)}).unwrap(){break}}
 let mut failure=None;caller.turn(stock(),|cx|{assert!(admission.take_failure_into_step(&mut failure,cx)?);Ok(())}).unwrap();retire(&mut caller,&mut failure);close_admission(&mut caller,&mut admission);
 // Measure both physical attempts with a fresh scope and an already resident return table.
 finish(&mut caller,&mut decoder);
 decoder=OriginalJobDecodeOwner::empty();
 bind(&mut caller,&mut decoder,&mut admission);
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let(result,heap)=observe_heap(||admit_original_construction(cx,&mut output,&mut admission,demand,1,actual_byte_birth));assert_eq!(result?,OriginalConstructionStatus::Pending);assert_eq!(cx.receiving_issued_grant(),None,"a born candidate returns its physical scope before metadata settlement");assert_eq!(cx.receiving_child_progress(),RetainedCloneProgress::default());assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()+overhead_work+controller["candidateWork"].as_u64().unwrap()));let child_items=(law["successItems"].as_u64().unwrap()+controller["candidateItems"].as_u64().unwrap())as usize;assert_eq!(admission.child.as_ref().unwrap().as_ref().unwrap().copied_items,child_items);assert_eq!(cx.retained_progress().copied_items-physical.copied_items,child_items+overhead_items as usize);assert_eq!((heap.0,heap.1),(1,0));Ok(())}).unwrap();assert_eq!(serde_json::to_value(**output.as_ref().unwrap()).unwrap(),law["value"]);
 caller.turn(stock(),|cx|{assert_eq!(admit_original_construction(cx,&mut output,&mut admission,demand,1,|_,_|panic!("ready attempt must not rerun"))?,OriginalConstructionStatus::Ready);Ok(())}).unwrap();
 retire(&mut caller,&mut output);close_admission(&mut caller,&mut admission);finish(&mut caller,&mut decoder);caller.retire(stock());
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
fn valued_byte_birth(value:u8,ran:&std::cell::Cell<bool>,actual:&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{
 ran.set(true);let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<u8>())};*actual=Some(usize::from(!raw.is_null()));
 if raw.is_null(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"actual original byte birth refused")),work:1}}
 unsafe{raw.write(value);}semio_framework_value::RetainedWorkOutcome{result:Ok((unsafe{Box::from_raw(raw)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}
}
thread_local!{static CLOCK_READS_LEFT:std::cell::Cell<usize>=const{std::cell::Cell::new(usize::MAX)};static CLOCK_TICK:std::cell::Cell<u64>=const{std::cell::Cell::new(1)};}
fn expiring_clock()->Option<u64>{CLOCK_READS_LEFT.with(|left|match left.get(){0=>{left.set(usize::MAX);None}n=>{if n!=usize::MAX{left.set(n-1)}Some(CLOCK_TICK.with(|tick|{tick.set(tick.get()+1000);tick.get()}))}})}
fn prepared(operation:u64)->(OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalConstructionAdmission){let mut caller=OriginalJobFixtureCaller::admit(OperationId(operation),Generation(2),stock(),expiring_clock,1<<40,100000);let mut decoder=caller.admitted_decoder();let mut admission=OriginalConstructionAdmission::default();bind(&mut caller,&mut decoder,&mut admission);(caller,decoder,admission)}
fn quote_grant(quote:OriginalConstructionDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:quote.items,maximum_copy_bytes:quote.copy_bytes,maximum_capacity_bytes:quote.capacity_bytes,maximum_release_bytes:quote.release_bytes,maximum_depth:quote.depth}}
/// 🔁️ A settled construction whose reset went unpaid leaves its completion marker for the next output, whose constructor must still run; the sweep moves one clock refusal across every deadline check of the turn.
#[test]
fn settled_construction_whose_reset_went_unpaid_still_runs_the_next_outputs_constructor(){
 let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:1,release_bytes:0,depth:1};let(quote,_)=original_construction_settled_quote::<Box<u8>>(demand,1).unwrap();let mut hits=Vec::new();
 for reads in 0..=160usize{
  CLOCK_READS_LEFT.with(|left|left.set(usize::MAX));let(mut caller,mut decoder,mut admission)=prepared(7600+reads as u64);
  let mut first=None::<Box<u8>>;let ran_first=std::cell::Cell::new(false);
  CLOCK_READS_LEFT.with(|left|left.set(reads));
  let turn=caller.refreshed_turn_with_decoder(quote_grant(quote),&mut decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut first,demand,1,|_,actual|valued_byte_birth(7,&ran_first,actual)));
  CLOCK_READS_LEFT.with(|left|left.set(usize::MAX));
  let Ok(mut settled)=turn else{std::mem::forget((caller,decoder,admission));continue};
  if settled{assert!(first.is_some())}
  if first.is_none(){std::mem::forget((caller,decoder,admission));continue}
  if !settled&&admission.complete{hits.push(reads)}
  for _ in 0..16{if settled{break}settled=caller.refreshed_turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut first,demand,1,|_,actual|valued_byte_birth(7,&ran_first,actual))).unwrap();}
  assert!(settled&&ran_first.get());assert_eq!(first.as_deref(),Some(&7));
  let mut second=None::<Box<u8>>;let ran_second=std::cell::Cell::new(false);let mut done=false;
  for _ in 0..16{let status=caller.refreshed_turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut second,demand,1,|_,actual|valued_byte_birth(9,&ran_second,actual))).unwrap();if status{assert!(second.is_some(),"settled reported with an empty output (clock reads {reads})");done=true;break}}
  assert!(done&&ran_second.get(),"the second constructor never ran (clock reads {reads})");assert_eq!(second.as_deref(),Some(&9));
  drop(second);drop(first);close_admission(&mut caller,&mut admission);finish(&mut caller,&mut decoder);caller.retire(stock());
 }
 assert!(!hits.is_empty(),"no variant cut the reset");
}

fn retirement_decoder(operation:u64)->(OriginalJobFixtureCaller,OriginalJobDecodeOwner){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(operation),Generation(1),stock(),clock,u64::MAX,200000);
 let mut decoder=caller.admitted_decoder();
 assert!(caller.turn_with_decoder(stock(),&mut decoder,|cx|{
  let mut outcome=None;
  let ran=cx.with_native_receiving(&mut outcome,|_,_|Ok(()))?;
  cx.settle_receiving_scope()?;
  Ok(ran)
 }).unwrap());
 (caller,decoder)
}

#[test]
fn native_retirement_receipt_allows_same_turn_forward_receiving(){
 let(mut caller,mut decoder)=retirement_decoder(7862);
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let expected=&law["retirementScope"];
 let demand=RetirementDemand{copy_bytes:expected["copiedBytes"].as_u64().unwrap() as usize,depth:1,..Default::default()};
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
 let mut effect=semio_framework_value_native_owned::native_decoding::NativeReceivingEffect::default();
 let mut observed=None;
 for _ in 0..16{
  observed=caller.turn_with_decoder(stock(),&mut decoder,|cx|{
   use semio_framework_value_native_owned::native_decoding::NativeReceivingEffectPoll;
   let mut outcome=None;
   let mut completed=None;
   if !cx.with_original_retirement(|cx|{
    cx.with_native_retirement_receiving(&mut outcome,StepContext::native_retirement_quote(demand,1),|native,_|{
     completed=Some(native.receive_effect(demand,1,&mut effect,|_,effect|{
      assert!(effect.performed(1).unwrap());
      semio_framework_value::RetainedWorkOutcome{result:Ok::<_,ValueError>(RetainedCloneStep::Complete(progress)),work:1}
     })?);
     Ok(())
    })
   })?{return Ok(None)}
   let outcome=outcome.unwrap();
   assert!(outcome.call.is_ok(),"{}",outcome.message());
   match completed{
    Some(NativeReceivingEffectPoll::Complete(turn))=>{
     assert_eq!(cx.receiving_child_progress(),progress);
     let paid=cx.retained_progress();
     crate::receive_original_receiving_work(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|error|crate::OriginalTurnPrimary::Receiving(error.receiving)),work:turn.work})?;
     let added=cx.retained_progress().copied_bytes-paid.copied_bytes;
     let mut forward=None;
     let opened=cx.with_native_receiving(&mut forward,|_,_|Ok(())).is_ok();
     if opened{cx.settle_receiving_scope()?}
     Ok(Some((added,opened)))
    }
    _=>Ok(None),
   }
  }).unwrap();
  if observed.is_some(){break}
 }
 finish(&mut caller,&mut decoder);
 caller.retire(stock());
 let(added,opened)=observed.expect("original retirement effect must post within its quoted turns");
 assert_eq!(serde_json::json!({"copiedBytes":progress.copied_bytes,"duplicateBytes":added,"forwardOpens":opened}),*expected);
}

#[test]
fn typed_native_retirement_uses_its_reserved_receipt_without_leaving_a_child_charge(){
 use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffect,NativeReceivingEffectPoll};
 let(mut caller,mut decoder)=retirement_decoder(7863);
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let expected=&law["reservedRetirementScope"];
 let demand=RetirementDemand{copy_bytes:expected["copiedBytes"].as_u64().unwrap() as usize,depth:1,..Default::default()};
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
 let mut effect=NativeReceivingEffect::<ValueError>::default();
 let mut observed=None;
 for _ in 0..16{
  observed=caller.turn_with_decoder(stock(),&mut decoder,|cx|{
   let mut outcome=None;
   let before=cx.retained_progress();
   let ran=cx.with_original_retirement(|cx|{
    cx.with_native_retirement_receiving_value(&mut outcome,StepContext::native_retirement_quote(demand,1),|native,_|{
     let poll=native.receive_effect(demand,1,&mut effect,|_,effect|{
      assert!(effect.performed(1).unwrap());
      semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(progress)),work:1}
     })?;
     let NativeReceivingEffectPoll::Complete(turn)=poll else{panic!("quoted typed retirement stayed pending")};
     Ok(turn.result.unwrap().progress().copied_bytes)
    })
   })?;
   if !ran{return Ok(None)}
   let outcome=outcome.unwrap();
   assert!(outcome.source_refusal.is_none(),"{}",outcome.message());
   assert!(cx.retained_progress().copied_bytes-before.copied_bytes>=demand.copy_bytes);
   Ok(Some((outcome.call.unwrap(),cx.receiving_child_progress().copied_bytes,cx.receiving_issued_grant().is_some())))
  }).unwrap();
  if observed.is_some(){break}
 }
 finish(&mut caller,&mut decoder);
 caller.retire(stock());
 let(copied_bytes,child_bytes,scope_open)=observed.expect("typed original receipt must post within its quoted turns");
 assert_eq!(serde_json::json!({"copiedBytes":copied_bytes,"childBytes":child_bytes,"scopeOpen":scope_open}),*expected);
}

#[test]
fn native_retirement_recovery_redeems_its_reserved_receipt_without_leaving_a_child_charge(){
 use semio_framework_value_native_owned::native_decoding::{NativeReceivingEffect,NativeReceivingEffectPhase};
 let(mut caller,mut decoder)=retirement_decoder(7864);
 caller.retain_dispatch().unwrap();
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let expected=&law["reservedRetirementScope"];
 let demand=RetirementDemand{copy_bytes:expected["copiedBytes"].as_u64().unwrap() as usize,depth:1,..Default::default()};
 let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
 let mut effect=NativeReceivingEffect::<ValueError>::default();
 let mut reply=None;
 let dispatch=caller.turn_with_decoder(stock(),&mut decoder,|cx|{
  let mut forward=None;
  assert!(cx.with_native_receiving(&mut forward,|native,_|{
   let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||native.receive_allocation_effect(demand,&mut effect,false,1,|effect|{
    assert!(effect.performed(1).unwrap());
    reply=Some(semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work:1});
    panic!("interrupt at the original effect reply")
   })));
   assert!(interrupted.is_err());
   Ok(())
  })?);
  cx.settle_receiving_scope()?;
  Ok(cx.original_source_turn_identity().unwrap())
 }).unwrap();
 assert_eq!(effect.phase(),NativeReceivingEffectPhase::EffectStarted);
 caller.cancel().cancel_now();
 let(copied_bytes,posted_bytes,child_bytes,scope_open)=caller.turn_with_decoder(stock(),&mut decoder,|cx|{
  assert_eq!(cx.original_source_turn_identity(),Some(dispatch));
  let mut outcome=None;
  let mut copied_bytes=None;
  let before=cx.retained_progress();
  assert!(cx.with_original_retirement(|cx|cx.with_native_retirement_receiving(&mut outcome,StepContext::native_retirement_quote(demand,1),|native,_|{
   native.resolve_unallocated_receiving_child(&mut effect,reply.take().unwrap()).unwrap();
   copied_bytes=Some(native.finish_receiving(&mut effect).result.unwrap().copied_bytes);
   Ok(())
  }))?);
  let outcome=outcome.unwrap();
  assert!(outcome.call.is_ok()&&outcome.source_refusal.is_none(),"{}",outcome.message());
  Ok((copied_bytes.unwrap(),cx.retained_progress().copied_bytes-before.copied_bytes,cx.receiving_child_progress().copied_bytes,cx.receiving_issued_grant().is_some()))
 }).unwrap();
 assert_eq!(effect.phase(),NativeReceivingEffectPhase::Empty);
 assert!(posted_bytes>=demand.copy_bytes);
 finish(&mut caller,&mut decoder);
 caller.retire(stock());
 assert_eq!(serde_json::json!({"copiedBytes":copied_bytes,"childBytes":child_bytes,"scopeOpen":scope_open}),*expected);
}
