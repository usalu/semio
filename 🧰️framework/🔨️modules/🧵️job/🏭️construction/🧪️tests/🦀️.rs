//! 🧫️ The lower native gate consumes the same neutral axes and authentic fixture roots.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::OriginalSourceBinding;
use crate::{OriginalJobFixtureCaller,OriginalJobDecodeOwner};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100_000,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128}}
fn clock()->Option<u64>{Some(1)}
fn finish(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for _ in 0..10000{if decoder.terminal_is_empty(){return}caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;crate::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}panic!("original construction decoder close must terminate")}
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
#[test]
fn over_quote_preserves_the_born_candidate_and_actual_original_decoder_receipt(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(73),Generation(5),stock(),clock,u64::MAX,100000);let mut decoder=OriginalJobDecodeOwner::empty();let mut output:Option<Box<[u8;70000]>>=None;
 let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:69999,release_bytes:0,depth:1};
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(stock(),&mut decoder,|cx|admit_original_construction(cx,&mut output,demand,|_|semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok((Box::new([7;70000]),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:70000,..Default::default()})),work:1})));
 let failure=result.unwrap_err();assert!(failure.primary_receiving().unwrap().quote_refusal.is_some());assert_eq!(decoder.owned_bytes(),70000);assert_eq!(caller.receipt().retained_capacity_bytes,70000);assert_eq!((heap.requested_bytes,heap.released_bytes),(70000,0));assert_eq!(output.as_deref().unwrap()[69999],7);
 caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{crate::close_original_retained_step(cx,RetirementDemand{release_bytes:70000,depth:1,..Default::default()},|_|{drop(output.take());semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:70000,..Default::default()})),work:1}})?;Ok(())})).unwrap();drop(failure);finish(&mut caller,&mut decoder);caller.retire(stock());
}

fn actual_byte_birth(_: &mut NativeDecodeControl<'_>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{
 let raw=unsafe{std::alloc::alloc(std::alloc::Layout::new::<u8>())};
 if raw.is_null(){return semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"actual original byte birth refused")),work:1}}
 unsafe{raw.write(7);}semio_framework_value::RetainedWorkOutcome{result:Ok((unsafe{Box::from_raw(raw)},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}
}
#[test]
fn actual_native_construction_forwards_null_attempt_work_without_inventing_physical_items(){
 let row:serde_json::Value=serde_json::from_str(fixture_text!("../../🏭️construction/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&row["workLaw"];let mut caller=OriginalJobFixtureCaller::admit(OperationId(7542),Generation(2),stock(),clock,u64::MAX,100000);let mut decoder=OriginalJobDecodeOwner::empty();let mut output=None;let demand=OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:1,release_bytes:0,depth:1};
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{cx.with_native_receiving::<_,ValueError>(|_|Ok(()))?;let issued=cx.receiving_issued_grant().unwrap();cx.consume_receiving_retained(issued,Default::default())?;Ok(())}).unwrap();
 caller.turn_with_decoder(RetainedCloneGrant{maximum_items:0,..stock()},&mut decoder,|cx|{let before=cx.original_work_units_spent();assert!(!admit_original_construction(cx,&mut output,demand,actual_byte_birth)?);assert_eq!(cx.original_work_units_spent()-before,u128::from(law["deniedWork"].as_u64().unwrap()));assert!(output.is_none());Ok(())}).unwrap();
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let((result,attempts),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||admit_original_construction(cx,&mut output,demand,actual_byte_birth)));assert_eq!(attempts,1);assert!(result.is_err());assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()));assert_eq!(cx.retained_progress().copied_items-physical.copied_items,law["failedItems"].as_u64().unwrap()as usize);assert!(output.is_none());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())}).unwrap();
 caller.turn_with_decoder(stock(),&mut decoder,|cx|{let before=cx.original_work_units_spent();let physical=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_original_construction(cx,&mut output,demand,actual_byte_birth));assert!(result?);assert_eq!(cx.original_work_units_spent()-before,u128::from(law["attemptWork"].as_u64().unwrap()));assert_eq!(cx.retained_progress().copied_items-physical.copied_items,law["successItems"].as_u64().unwrap()as usize);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(1,0));Ok(())}).unwrap();assert_eq!(serde_json::to_value(**output.as_ref().unwrap()).unwrap(),law["value"]);
 caller.turn(stock(),|cx|{crate::close_original_retained_step(cx,RetirementDemand{release_bytes:1,depth:1,..Default::default()},|_|{drop(output.take());semio_framework_value::RetainedWorkOutcome::<_,ValueError>{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:1,..Default::default()})),work:1}})?;Ok(())}).unwrap();finish(&mut caller,&mut decoder);caller.retire(stock());
}
