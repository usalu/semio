//! 🧫️ The lower native gate consumes the same neutral axes and authentic fixture roots.
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
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(stock(),|cx|{assert!(!binding.matches(cx));Ok(())}));result.unwrap();assert_eq!(heap,Default::default());assert_eq!(foreign.receipt(),Default::default());
 foreign.retire(stock());original.retire(stock());
}

fn bind(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,admission:&mut OriginalConstructionAdmission){let mut slot=None::<()>;caller.turn_with_decoder(stock(),decoder,|cx|{assert_eq!(admit_original_construction(cx,&mut slot,admission,OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1},1,|_,_|panic!("binding does not enter constructor"))?,OriginalConstructionStatus::Pending);Ok(())}).unwrap();assert!(admission.binding.is_some())}
fn retire<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,original:&mut Option<T>){
 let mut owner=None;caller.turn(stock(),|cx|{let bytes=std::mem::size_of::<Option<T>>()+std::mem::size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>();assert!(construction_metadata(cx,bytes,2)?);owner=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(original.take().unwrap()));Ok(())}).unwrap();
 let mut effect=semio_framework_value_native_owned::native_decoding::NativeReceivingEffect::default();
 for _ in 0..10000{if owner.as_ref().unwrap().terminal_is_empty(){break}caller.turn_with_decoder(stock(),decoder,|cx|cx.with_original_retirement(|cx|{
  let owner=owner.as_mut().unwrap();let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;let work=owner.next_computational_work(cx.retained_grant().maximum_copy_bytes)?;let mut source=None;let mut child=None;
  assert!(construction_metadata(cx,std::mem::size_of_val(&child),1)?);
  assert!(cx.with_native_retirement_receiving(&mut source,|native,_body_grant|{let turn=native.receive_retirement(demand,&mut effect,true,work,|grant,actual|{let result=owner.step(grant,actual.expect("actual cleanup allocation frame"));semio_framework_value::RetainedWorkOutcome{result,work:owner.last_turn_work()}});child=Some(turn.result);Ok(())})?);
  assert!(source.as_ref().unwrap().call.is_ok());assert!(source.as_ref().unwrap().source_refusal.is_none());assert!(child.as_ref().unwrap().is_ok());
  assert!(construction_metadata(cx,std::mem::size_of_val(&source)+std::mem::size_of_val(&child),2)?);source=None;child=None;Ok(())
 })).unwrap();}
 assert!(owner.as_ref().unwrap().terminal_is_empty());caller.turn(stock(),|cx|{assert!(construction_metadata(cx,std::mem::size_of_val(&owner),1)?);owner=None;Ok(())}).unwrap();
}
fn close_admission(caller:&mut OriginalJobFixtureCaller,admission:&mut OriginalConstructionAdmission){for _ in 0..10000{if admission.terminal_is_empty(){return}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{admission.close_step(cx)?;Ok(())})).unwrap();}panic!("construction control close must terminate")}
#[test]
fn over_quote_preserves_the_born_candidate_and_actual_original_decoder_receipt(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(73),Generation(5),stock(),clock,u64::MAX,200000);let mut decoder=OriginalJobDecodeOwner::empty();let mut admission=OriginalConstructionAdmission::default();bind(&mut caller,&mut decoder,&mut admission);let mut output:Option<Box<[u8;70000]>>=None;
 let demand=OriginalConstructionDemand{items:1,copy_bytes:70000,capacity_bytes:69999,release_bytes:0,depth:1};
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction(cx,&mut output,&mut admission,demand,1,|_,actual|{let value=Box::new([7;70000]);*actual=Some(70000);semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:70000,retained_capacity_bytes:70000,..Default::default()})),work:1}})));
 assert_eq!(result.unwrap(),OriginalConstructionStatus::Failed);assert!(admission.has_failure());let failure=admission.child.as_ref().unwrap().as_ref().unwrap_err();assert!(failure.allocation_failure.is_some()||failure.native_allocation.is_some());assert_eq!(decoder.owned_bytes(),70000);assert_eq!(caller.receipt().retained_capacity_bytes,70000);assert_eq!((heap.requested_bytes,heap.released_bytes),(70000,0));assert_eq!(output.as_deref().unwrap()[69999],7);
 let mut failure=None;caller.turn(stock(),|cx|{assert!(admission.take_failure_into_step(&mut failure,cx)?);Ok(())}).unwrap();retire(&mut caller,&mut decoder,&mut output);retire(&mut caller,&mut decoder,&mut failure);close_admission(&mut caller,&mut admission);finish(&mut caller,&mut decoder);caller.retire(stock());
}

fn actual_byte_birth(_: &mut semio_framework_value_native_owned::native_decoding::NativeConstructionScope<'_, '_>,actual:&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{
 let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<u8>())};*actual=Some(usize::from(!raw.is_null()));
 if raw.is_null(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"actual original byte birth refused")),work:1}}
 unsafe{raw.write(7);}semio_framework_value::RetainedWorkOutcome{result:Ok((unsafe{Box::from_raw(raw)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}
}

#[test]
fn actual_native_construction_forwards_null_attempt_work_without_inventing_physical_items(){
 let row:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&row["workLaw"];let controller:serde_json::Value=serde_json::from_str(include_str!("admission.json")).unwrap();let controller=&controller["controller"];let overhead_work=controller["sourceWork"].as_u64().unwrap()+controller["captureWork"].as_u64().unwrap();let overhead_items=controller["sourceItems"].as_u64().unwrap()+controller["captureItems"].as_u64().unwrap();let mut caller=OriginalJobFixtureCaller::admit(OperationId(7542),Generation(2),stock(),clock,u64::MAX,100000);let mut decoder=OriginalJobDecodeOwner::empty();let mut admission=OriginalConstructionAdmission::default();let mut output=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:1,release_bytes:0,depth:1};bind(&mut caller,&mut decoder,&mut admission);
 caller.turn_with_decoder(RetainedCloneGrant{maximum_items:0,..stock()},&mut decoder,|cx|{let before=cx.original_work_units_spent();assert_eq!(admit_original_construction(cx,&mut output,&mut admission,demand,1,actual_byte_birth)?,OriginalConstructionStatus::Pending);assert_eq!(cx.original_work_units_spent()-before,u128::from(law["deniedWork"].as_u64().unwrap()));assert!(output.is_none());Ok(())}).unwrap();
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let((result,attempts),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||admit_original_construction(cx,&mut output,&mut admission,demand,1,actual_byte_birth)));assert_eq!(attempts,1);assert_eq!(result?,OriginalConstructionStatus::Failed);assert!(admission.has_failure());assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()+overhead_work));assert_eq!(cx.retained_progress().copied_items-physical.copied_items,(law["failedItems"].as_u64().unwrap()+overhead_items)as usize);assert!(output.is_none());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();
 let mut failure=None;caller.turn(stock(),|cx|{assert!(admission.take_failure_into_step(&mut failure,cx)?);Ok(())}).unwrap();retire(&mut caller,&mut decoder,&mut failure);close_admission(&mut caller,&mut admission);bind(&mut caller,&mut decoder,&mut admission);
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_original_construction(cx,&mut output,&mut admission,demand,1,actual_byte_birth));assert_eq!(result?,OriginalConstructionStatus::Pending);assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()+overhead_work+controller["candidateWork"].as_u64().unwrap()));assert_eq!(cx.retained_progress().copied_items-physical.copied_items,(law["successItems"].as_u64().unwrap()+overhead_items+controller["candidateItems"].as_u64().unwrap())as usize);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(1,0));Ok(())}).unwrap();assert_eq!(serde_json::to_value(**output.as_ref().unwrap()).unwrap(),law["value"]);
 caller.turn(stock(),|cx|{assert_eq!(admit_original_construction(cx,&mut output,&mut admission,demand,1,|_,_|panic!("ready attempt must not rerun"))?,OriginalConstructionStatus::Ready);Ok(())}).unwrap();
 retire(&mut caller,&mut decoder,&mut output);close_admission(&mut caller,&mut admission);finish(&mut caller,&mut decoder);caller.retire(stock());
}
