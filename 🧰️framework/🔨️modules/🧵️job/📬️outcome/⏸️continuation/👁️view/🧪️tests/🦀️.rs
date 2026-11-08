//! 👁️ A public continuation view borrows the complete received causes before and after frame insertion.
use semio_framework_job_context::*;
use semio_framework_job_return::*;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
#[global_allocator]
static VIEW_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn inspect(value:&OriginalJobContinuation,row:&serde_json::Value,caller:&OriginalJobFixtureCaller){
 let fuel=caller.fuel_remaining();let(receipt,heap)=observe(||{let v=value.view();(v.callback.map(|f|f.primary.message()),v.callback.and_then(|f|f.receipt_refusal.as_ref().map(|e|e.message.as_ref())),v.callback.and_then(|f|f.context_refusal.as_ref().map(|e|e.message.as_ref())),v.qualification.map(|f|f.primary.message()),v.receipt_qualification.map(|f|f.primary.message()),v.descriptor.is_some_and(|d|d.kind()==JobOutcomeKind::Yield))});
 assert_eq!(serde_json::to_value(receipt).unwrap(),row["view"]);assert_eq!(caller.fuel_remaining(),fuel);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
#[test]
fn continuation_view_preserves_complete_original_causes_across_return_receiving_and_insertion(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../case.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("../case.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v);}let s=&law["source"];let stock=RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize};let q=RetainedCloneGrant{maximum_items:1,..stock};
 for(index,row)in law["cases"].as_array().unwrap().iter().enumerate(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(7670+index as u64),Generation(1),stock,||Some(0),s["deadlineUs"].as_u64().unwrap(),s["fuel"].as_u64().unwrap());let mut origin=OriginalJobReturnOrigin::empty();let mut ready=false;
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){ready=caller.turn(q,|cx|origin.admit(cx)).unwrap();if ready{break}}assert!(ready);let mut returned=None;
  caller.turn(q,|cx|{let claim=origin.enter_body(cx)?.unwrap();let(result,receipt)=if row["name"]=="callback"{let mut failure=OriginalTurnFailure::from(semio_framework_value_retained_clone::ValueError::literal(semio_framework_value_retained_clone::ValueRefusalKind::UnsupportedOwner,"Original callback cause"));failure.receipt_refusal=Some(semio_framework_value_retained_clone::ValueError::literal(semio_framework_value_retained_clone::ValueRefusalKind::OwnershipLimit,"Original receipt refusal"));failure.context_refusal=Some(semio_framework_value_retained_clone::ValueError::literal(semio_framework_value_retained_clone::ValueRefusalKind::InvariantViolated,"Original context refusal"));(InteractiveJobStep::Continuation(failure),None)}else{(InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(OriginalTurnFailure::from)),Some(OriginalTurnFailure::from(semio_framework_value_retained_clone::ValueError::literal(semio_framework_value_retained_clone::ValueRefusalKind::InvariantViolated,"Original receipt qualification"))))};match claim.finish(cx,result,receipt){OriginalJobReturn::Continuation(value)=>returned=Some(value),_=>panic!("genuine failed original continuation")};Ok(())}).unwrap();
  inspect(returned.as_ref().unwrap(),row,&caller);caller.turn(q,|cx|{assert!(origin.receive_continuation(cx,&mut returned)?);Ok(())}).unwrap();assert!(returned.is_none());inspect(origin.continuation().unwrap(),row,&caller);
  for _ in 0..2{caller.turn(q,|cx|origin.close_step(cx).map(|_|())).unwrap();}inspect(origin.continuation().unwrap(),row,&caller);
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if origin.terminal_is_empty(){break}caller.turn(q,|cx|origin.close_step(cx).map(|_|())).unwrap();}assert!(origin.terminal_is_empty());let(_,heap)=observe(||drop(origin));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(stock);
 }
}
